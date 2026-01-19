//! Rust/Anchor code generator
//!
//! Generates Anchor-compatible Rust code from Solana IR.

use crate::error::CodegenError;
use crate::ir::*;
use crate::GeneratedProject;

/// Rust code generator for Anchor programs
pub struct RustGenerator {
    /// Events for looking up field names
    events: Vec<Event>,
    /// Current instruction's signer parameter names (for generating ctx.accounts access)
    signer_params: std::collections::HashSet<String>,
    /// Internal (non-public) function names
    internal_functions: std::collections::HashSet<String>,
    /// Whether we're currently generating a helper function body (not inside #[program])
    in_helper_function: bool,
}

impl RustGenerator {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            signer_params: std::collections::HashSet::new(),
            internal_functions: std::collections::HashSet::new(),
            in_helper_function: false,
        }
    }

    /// Generate a complete Anchor project from Solana IR
    pub fn generate(&mut self, programs: &[SolanaProgram]) -> Result<GeneratedProject, CodegenError> {
        if programs.is_empty() {
            return Err(CodegenError::MissingElement(
                "No deployable contracts found (abstract contracts cannot be deployed)".to_string()
            ));
        }

        // Use the last contract (child contracts come after their parents)
        // In inheritance hierarchies, we generate code for the most derived contract
        let program = programs.last().unwrap();

        // Store events for lookup during emit generation
        self.events = program.events.clone();

        // Collect internal function names for proper call generation
        self.internal_functions.clear();
        for instruction in &program.instructions {
            if !instruction.is_public {
                self.internal_functions.insert(to_snake_case(&instruction.name));
            }
        }

        let lib_rs = self.generate_lib_rs(program)?;
        let state_rs = self.generate_state_rs(program)?;
        let instructions_rs = self.generate_instructions_rs(program)?;
        let error_rs = self.generate_error_rs(program)?;
        let events_rs = self.generate_events_rs(program)?;
        let anchor_toml = self.generate_anchor_toml(program);
        let cargo_toml = self.generate_cargo_toml(program);

        // Generate TypeScript client
        let mut ts_gen = crate::ts_gen::TypeScriptGenerator::new();
        let client_ts = ts_gen.generate(program)?;

        // Generate TypeScript tests
        let mut test_gen = crate::test_gen::TestGenerator::new();
        let tests_ts = test_gen.generate(program)?;

        // Generate IDL
        let mut idl_gen = crate::idl_gen::IdlGenerator::new();
        let idl_json = idl_gen.generate(program)?;

        // Generate package.json
        let package_json = self.generate_package_json(program);

        // Generate README and .gitignore
        let readme = self.generate_readme(program);
        let gitignore = self.generate_gitignore();

        // Generate Rust tests from #[test] functions
        let rust_tests = self.generate_rust_tests(program)?;
        let has_tests = !program.tests.is_empty();

        Ok(GeneratedProject {
            lib_rs,
            state_rs,
            instructions_rs,
            error_rs,
            events_rs,
            anchor_toml,
            cargo_toml,
            client_ts,
            tests_ts,
            idl_json,
            package_json,
            readme,
            gitignore,
            rust_tests,
            has_tests,
        })
    }

    /// Generate Rust unit tests from #[test] functions
    fn generate_rust_tests(&self, program: &SolanaProgram) -> Result<String, CodegenError> {
        if program.tests.is_empty() {
            return Ok(String::new());
        }

        let mut output = String::new();
        output.push_str("//! Generated tests from SolScript #[test] functions\n\n");
        output.push_str("#[cfg(test)]\n");
        output.push_str("mod solscript_tests {\n");
        output.push_str("    use super::*;\n\n");

        for test in &program.tests {
            let test_name = to_snake_case(&test.name);

            // Determine if this is a should_fail test
            if let Some(expected_msg) = &test.should_fail {
                if expected_msg.is_empty() {
                    output.push_str(&format!("    #[test]\n    #[should_panic]\n    fn {}() {{\n", test_name));
                } else {
                    output.push_str(&format!("    #[test]\n    #[should_panic(expected = \"{}\")]\n    fn {}() {{\n", expected_msg, test_name));
                }
            } else {
                output.push_str(&format!("    #[test]\n    fn {}() {{\n", test_name));
            }

            // Generate test body
            for stmt in &test.body {
                let stmt_code = self.generate_statement(stmt, 2)?;
                output.push_str(&stmt_code);
            }

            output.push_str("    }\n\n");
        }

        output.push_str("}\n");
        Ok(output)
    }

    fn generate_lib_rs(&mut self, program: &SolanaProgram) -> Result<String, CodegenError> {
        let name = to_snake_case(&program.name);
        let uses_token = program.instructions.iter().any(|i| i.uses_token_program);

        let mut imports = String::from("use anchor_lang::prelude::*;\n");
        if uses_token {
            imports.push_str("use anchor_spl::token::CpiContext;\n");
        }

        // Generate helper functions (internal/private functions)
        let helper_fns = self.generate_helper_functions(program)?;

        Ok(format!(
            r#"//! Generated by SolScript compiler
//! Contract: {}

{}
mod state;
mod instructions;
mod error;
mod events;

pub use state::*;
pub use instructions::*;
pub use error::*;
// Events are accessed via events:: prefix to avoid name collisions

declare_id!("11111111111111111111111111111111");

{}

#[program]
pub mod {} {{
    use super::*;

{}
}}
"#,
            program.name,
            imports,
            helper_fns,
            name,
            self.generate_instruction_handlers(program)?
        ))
    }

    fn generate_helper_functions(&mut self, program: &SolanaProgram) -> Result<String, CodegenError> {
        let mut helpers = String::new();

        for instruction in &program.instructions {
            if !instruction.is_public {
                helpers.push_str(&self.generate_helper_function(instruction, program)?);
                helpers.push('\n');
            }
        }

        Ok(helpers)
    }

    fn generate_helper_function(
        &mut self,
        instruction: &Instruction,
        program: &SolanaProgram,
    ) -> Result<String, CodegenError> {
        let name = to_snake_case(&instruction.name);

        // Generate parameters
        let params: Vec<String> = instruction
            .params
            .iter()
            .map(|p| format!("{}: {}", to_snake_case(&p.name), self.type_to_rust(&p.ty)))
            .collect();

        // Add state parameter for functions that access state
        let state_type = format!("&mut crate::state::{}State", to_pascal_case(&program.name));
        let mut all_params = vec![format!("state: {}", state_type)];
        all_params.extend(params);
        let params_str = all_params.join(", ");

        // Return type
        let return_type = match &instruction.returns {
            Some(ty) => format!("Result<{}>", self.type_to_rust(ty)),
            None => "Result<()>".to_string(),
        };

        // Generate body (with in_helper_function flag set)
        self.in_helper_function = true;
        let body = self.generate_helper_body(instruction, program)?;
        self.in_helper_function = false;

        Ok(format!(
            r#"/// Internal helper function: {}
fn {}({}) -> {} {{
{}
}}
"#,
            instruction.name,
            name,
            params_str,
            return_type,
            body
        ))
    }

    fn generate_helper_body(
        &mut self,
        instruction: &Instruction,
        _program: &SolanaProgram,
    ) -> Result<String, CodegenError> {
        let mut body = String::new();

        // Generate statements
        for stmt in &instruction.body {
            body.push_str(&self.generate_statement(stmt, 1)?);
            body.push('\n');
        }

        // Add return if not present
        if instruction.returns.is_none() && !body.contains("Ok(())") {
            body.push_str("    Ok(())\n");
        }

        Ok(body)
    }

    fn generate_instruction_handlers(&mut self, program: &SolanaProgram) -> Result<String, CodegenError> {
        let mut handlers = String::new();

        for instruction in &program.instructions {
            // Only generate public functions as Anchor instructions
            if instruction.is_public {
                handlers.push_str(&self.generate_instruction_handler(instruction, program)?);
                handlers.push('\n');
            }
        }

        Ok(handlers)
    }

    fn generate_instruction_handler(
        &mut self,
        instruction: &Instruction,
        program: &SolanaProgram,
    ) -> Result<String, CodegenError> {
        let name = to_snake_case(&instruction.name);
        let ctx_type = to_pascal_case(&instruction.name);

        // Generate parameters (skip Signer types as they're in ctx.accounts)
        let params: Vec<String> = instruction
            .params
            .iter()
            .filter(|p| !matches!(p.ty, SolanaType::Signer))
            .map(|p| format!("{}: {}", to_snake_case(&p.name), self.type_to_rust(&p.ty)))
            .collect();

        let params_str = if params.is_empty() {
            String::new()
        } else {
            format!(", {}", params.join(", "))
        };

        // Generate return type
        let return_type = match &instruction.returns {
            Some(ty) => format!("Result<{}>", self.type_to_rust(ty)),
            None => "Result<()>".to_string(),
        };

        // Generate body
        let body = self.generate_instruction_body(instruction, program)?;

        Ok(format!(
            "    pub fn {}(ctx: Context<{}>{}) -> {} {{\n{}\n    }}\n",
            name, ctx_type, params_str, return_type, body
        ))
    }

    fn generate_instruction_body(
        &mut self,
        instruction: &Instruction,
        program: &SolanaProgram,
    ) -> Result<String, CodegenError> {
        // Track signer params for this instruction
        self.signer_params.clear();
        for param in &instruction.params {
            if matches!(param.ty, SolanaType::Signer) {
                self.signer_params.insert(to_snake_case(&param.name));
            }
        }

        let mut body = String::new();

        // If no modifiers, just generate the function body directly
        if instruction.modifiers.is_empty() {
            for stmt in &instruction.body {
                body.push_str(&self.generate_statement(stmt, 2)?);
            }
        } else {
            // Inline modifiers: wrap the function body with modifier code
            // For now, we handle single modifiers. Multiple modifiers would need nesting.
            for modifier_call in &instruction.modifiers {
                // Find the modifier definition
                if let Some(modifier_def) = program.modifiers.iter().find(|m| m.name == modifier_call.name) {
                    // Generate modifier body, replacing Placeholder with function body
                    for stmt in &modifier_def.body {
                        self.generate_inlined_statement(
                            stmt,
                            &instruction.body,
                            2,
                            &mut body,
                        )?;
                    }
                } else {
                    // Modifier not found, add comment and continue
                    body.push_str(&format!(
                        "        // Modifier: {} (definition not found)\n",
                        modifier_call.name
                    ));
                    for stmt in &instruction.body {
                        body.push_str(&self.generate_statement(stmt, 2)?);
                    }
                }
            }
        }

        // Add default return if needed
        if instruction.returns.is_none() && !body.contains("Ok(") {
            body.push_str("        Ok(())\n");
        }

        Ok(body)
    }

    /// Generate a statement, replacing Placeholder with the inner function body
    fn generate_inlined_statement(
        &self,
        stmt: &Statement,
        inner_body: &[Statement],
        indent: usize,
        output: &mut String,
    ) -> Result<(), CodegenError> {
        match stmt {
            Statement::Placeholder => {
                // Replace placeholder with the inner function body
                for inner_stmt in inner_body {
                    output.push_str(&self.generate_statement(inner_stmt, indent)?);
                }
            }
            Statement::If { condition, then_block, else_block } => {
                // Need to recursively handle if statements that might contain placeholders
                let ind = "    ".repeat(indent);
                output.push_str(&format!(
                    "{}if {} {{\n",
                    ind,
                    self.generate_expression(condition)?
                ));
                for s in then_block {
                    self.generate_inlined_statement(s, inner_body, indent + 1, output)?;
                }
                if let Some(else_stmts) = else_block {
                    output.push_str(&format!("{}}} else {{\n", ind));
                    for s in else_stmts {
                        self.generate_inlined_statement(s, inner_body, indent + 1, output)?;
                    }
                }
                output.push_str(&format!("{}}}\n", ind));
            }
            _ => {
                // For other statements, generate normally
                output.push_str(&self.generate_statement(stmt, indent)?);
            }
        }
        Ok(())
    }

    fn generate_statement(&self, stmt: &Statement, indent: usize) -> Result<String, CodegenError> {
        let ind = "    ".repeat(indent);

        match stmt {
            Statement::VarDecl { name, ty, value } => {
                let name = to_snake_case(name);
                let ty_str = self.type_to_rust(ty);
                match value {
                    Some(expr) => Ok(format!(
                        "{}let {}: {} = {};\n",
                        ind,
                        name,
                        ty_str,
                        self.generate_expression(expr)?
                    )),
                    None => Ok(format!(
                        "{}let {}: {} = Default::default();\n",
                        ind, name, ty_str
                    )),
                }
            }
            Statement::Assign { target, value } => {
                Ok(format!(
                    "{}{} = {};\n",
                    ind,
                    self.generate_expression(target)?,
                    self.generate_expression(value)?
                ))
            }
            Statement::If {
                condition,
                then_block,
                else_block,
            } => {
                let mut result = format!(
                    "{}if {} {{\n",
                    ind,
                    self.generate_expression(condition)?
                );
                for s in then_block {
                    result.push_str(&self.generate_statement(s, indent + 1)?);
                }
                result.push_str(&format!("{}}}", ind));

                if let Some(else_stmts) = else_block {
                    result.push_str(" else {\n");
                    for s in else_stmts {
                        result.push_str(&self.generate_statement(s, indent + 1)?);
                    }
                    result.push_str(&format!("{}}}", ind));
                }
                result.push('\n');
                Ok(result)
            }
            Statement::While { condition, body } => {
                let mut result = format!(
                    "{}while {} {{\n",
                    ind,
                    self.generate_expression(condition)?
                );
                for s in body {
                    result.push_str(&self.generate_statement(s, indent + 1)?);
                }
                result.push_str(&format!("{}}}\n", ind));
                Ok(result)
            }
            Statement::For {
                init,
                condition,
                update,
                body,
            } => {
                let mut result = String::new();

                // For loops become while loops in Rust
                if let Some(init_stmt) = init {
                    result.push_str(&self.generate_statement(init_stmt, indent)?);
                }

                let cond = match condition {
                    Some(c) => self.generate_expression(c)?,
                    None => "true".to_string(),
                };

                result.push_str(&format!("{}while {} {{\n", ind, cond));

                for s in body {
                    result.push_str(&self.generate_statement(s, indent + 1)?);
                }

                if let Some(upd) = update {
                    result.push_str(&format!(
                        "{}    {};\n",
                        ind,
                        self.generate_expression(upd)?
                    ));
                }

                result.push_str(&format!("{}}}\n", ind));
                Ok(result)
            }
            Statement::Return(expr) => match expr {
                Some(e) => Ok(format!("{}Ok({})\n", ind, self.generate_expression(e)?)),
                None => Ok(format!("{}Ok(())\n", ind)),
            },
            Statement::Emit { event, args } => {
                // Look up the event to get field names
                let event_def = self.events.iter().find(|e| e.name == *event);
                let args_str: Vec<String> = args
                    .iter()
                    .enumerate()
                    .map(|(i, a)| {
                        let val = self.generate_expression(a)?;
                        let field_name = event_def
                            .and_then(|e| e.fields.get(i))
                            .map(|f| to_snake_case(&f.name))
                            .unwrap_or_else(|| format!("field{}", i));
                        Ok(format!("{}: {}", field_name, val))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(format!(
                    "{}emit!(events::{} {{ {} }});\n",
                    ind,
                    to_pascal_case(event),
                    args_str.join(", ")
                ))
            }
            Statement::Require { condition, message: _ } => {
                Ok(format!(
                    "{}require!({}, CustomError::RequireFailed);\n",
                    ind,
                    self.generate_expression(condition)?
                ))
            }
            Statement::RevertWithError { error_name, args: _ } => {
                // Note: Anchor's #[error_code] doesn't support struct-style errors with data
                // So we generate simple error variants. The error parameters in Solidity
                // are useful for type checking but don't translate to Anchor error data.
                Ok(format!(
                    "{}return Err(error!(CustomError::{}));\n",
                    ind,
                    to_pascal_case(error_name)
                ))
            }
            Statement::Delete(target) => {
                // Delete resets the target to its default value
                // For state variables: assign Default::default()
                // For mappings: we would need to close the PDA (not supported yet)
                let target_expr = self.generate_expression(target)?;
                Ok(format!("{}{} = Default::default();\n", ind, target_expr))
            }
            Statement::Selfdestruct { .. } => {
                // Selfdestruct is handled by the close constraint on the state account
                // The actual closing is done by Anchor based on the account constraint
                // We just add a comment here for clarity
                Ok(format!("{}// State account will be closed, rent sent to recipient\n", ind))
            }
            Statement::Expr(expr) => {
                Ok(format!("{}{};\n", ind, self.generate_expression(expr)?))
            }
            Statement::Placeholder => {
                // Placeholder should be replaced during modifier inlining
                // This should not appear in generated code
                Ok(String::new())
            }
        }
    }

    fn generate_expression(&self, expr: &Expression) -> Result<String, CodegenError> {
        match expr {
            Expression::Literal(lit) => self.generate_literal(lit),
            Expression::Var(name) => {
                let snake_name = to_snake_case(name);
                // If this is a signer param, access it from ctx.accounts
                if self.signer_params.contains(&snake_name) {
                    Ok(format!("ctx.accounts.{}.key()", snake_name))
                } else {
                    Ok(snake_name)
                }
            }
            Expression::StateAccess(field) => {
                if self.in_helper_function {
                    Ok(format!("state.{}", to_snake_case(field)))
                } else {
                    Ok(format!("ctx.accounts.state.{}", to_snake_case(field)))
                }
            }
            Expression::MappingAccess { mapping_name: _, keys: _, account_name } => {
                // Access the PDA account's value field
                Ok(format!("ctx.accounts.{}.value", to_snake_case(account_name)))
            }
            Expression::MsgSender => Ok("ctx.accounts.signer.key()".to_string()),
            Expression::MsgValue => Ok("0u64 /* msg.value not supported */".to_string()),
            Expression::BlockTimestamp => Ok("Clock::get()?.unix_timestamp as u64".to_string()),
            // Solana Clock sysvar fields
            Expression::ClockSlot => Ok("Clock::get()?.slot".to_string()),
            Expression::ClockEpoch => Ok("Clock::get()?.epoch".to_string()),
            Expression::ClockUnixTimestamp => Ok("Clock::get()?.unix_timestamp".to_string()),
            // Solana Rent sysvar methods
            Expression::RentMinimumBalance { data_len } => {
                let len_str = self.generate_expression(data_len)?;
                Ok(format!("Rent::get()?.minimum_balance({} as usize)", len_str))
            }
            Expression::RentIsExempt { lamports, data_len } => {
                let lamports_str = self.generate_expression(lamports)?;
                let len_str = self.generate_expression(data_len)?;
                Ok(format!("Rent::get()?.is_exempt({}, {} as usize)", lamports_str, len_str))
            }
            Expression::Binary { op, left, right } => {
                let l = self.generate_expression(left)?;
                let r = self.generate_expression(right)?;
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Rem => "%",
                    BinaryOp::Eq => "==",
                    BinaryOp::Ne => "!=",
                    BinaryOp::Lt => "<",
                    BinaryOp::Le => "<=",
                    BinaryOp::Gt => ">",
                    BinaryOp::Ge => ">=",
                    BinaryOp::And => "&&",
                    BinaryOp::Or => "||",
                    BinaryOp::BitAnd => "&",
                    BinaryOp::BitOr => "|",
                    BinaryOp::BitXor => "^",
                    BinaryOp::Shl => "<<",
                    BinaryOp::Shr => ">>",
                };
                Ok(format!("({} {} {})", l, op_str, r))
            }
            Expression::Unary { op, expr } => {
                let e = self.generate_expression(expr)?;
                let op_str = match op {
                    UnaryOp::Neg => "-",
                    UnaryOp::Not => "!",
                    UnaryOp::BitNot => "!",
                };
                Ok(format!("({}{})", op_str, e))
            }
            Expression::Call { func, args } => {
                let func_name = to_snake_case(func);
                let args_str: Vec<String> = args
                    .iter()
                    .map(|a| self.generate_expression(a))
                    .collect::<Result<Vec<_>, _>>()?;

                // Check if this is a call to an internal function
                if self.internal_functions.contains(&func_name) {
                    // Internal functions receive state as first parameter
                    let state_arg = if self.in_helper_function {
                        "state".to_string()
                    } else {
                        "&mut ctx.accounts.state".to_string()
                    };
                    let mut all_args = vec![state_arg];
                    all_args.extend(args_str);
                    Ok(format!("{}({})?", func_name, all_args.join(", ")))
                } else {
                    Ok(format!("{}({})", func_name, args_str.join(", ")))
                }
            }
            Expression::MethodCall {
                receiver,
                method,
                args,
            } => {
                // Handle special assignment marker
                if method == "__assign__" && args.len() == 1 {
                    let target = self.generate_expression(receiver)?;
                    let value = self.generate_expression(&args[0])?;
                    return Ok(format!("{} = {}", target, value));
                }

                let recv = self.generate_expression(receiver)?;
                let args_str: Vec<String> = args
                    .iter()
                    .map(|a| self.generate_expression(a))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(format!("{}.{}({})", recv, to_snake_case(method), args_str.join(", ")))
            }
            Expression::CpiCall { program, interface_name, method, args } => {
                let prog = self.generate_expression(program)?;
                let args_str: Vec<String> = args
                    .iter()
                    .map(|a| self.generate_expression(a))
                    .collect::<Result<Vec<_>, _>>()?;

                // Generate Anchor-style instruction discriminator
                // Format: sha256("global:{method_name}")[0..8]
                let method_snake = to_snake_case(method);

                // Build instruction data serialization
                let mut data_parts = Vec::new();
                data_parts.push(format!(
                    "let discriminator = anchor_lang::solana_program::hash::hash(b\"global:{}\").to_bytes();",
                    method_snake
                ));
                data_parts.push("let mut data = discriminator[..8].to_vec();".to_string());

                // Serialize each argument using Borsh
                for arg in &args_str {
                    data_parts.push(format!(
                        "AnchorSerialize::serialize(&({}), &mut data).unwrap();",
                        arg
                    ));
                }

                // Generate account metas from address-type arguments
                // This is a heuristic - addresses become account metas
                let mut account_metas = Vec::new();
                let mut account_infos = Vec::new();
                for arg in args_str.iter() {
                    // Add each argument that looks like a pubkey as an account
                    account_metas.push(format!(
                        "AccountMeta::new({}, false)",
                        arg
                    ));
                    account_infos.push(format!(
                        "/* account_info for {} */",
                        arg
                    ));
                }

                let accounts_vec = if account_metas.is_empty() {
                    "vec![]".to_string()
                } else {
                    format!("vec![{}]", account_metas.join(", "))
                };

                Ok(format!(
                    r#"{{
            use anchor_lang::prelude::*;
            // CPI to {interface_name}.{method}
            let cpi_program = {prog};

            // Build instruction data with Anchor discriminator
            {data_code}

            // Build the instruction
            let ix = anchor_lang::solana_program::instruction::Instruction {{
                program_id: cpi_program,
                accounts: {accounts},
                data,
            }};

            // Execute CPI
            // Note: You may need to add the appropriate account_infos based on your context
            anchor_lang::solana_program::program::invoke(
                &ix,
                &[cpi_program.to_account_info()],
            )?
        }}"#,
                    interface_name = interface_name,
                    method = method,
                    prog = prog,
                    data_code = data_parts.join("\n            "),
                    accounts = accounts_vec,
                ))
            }
            Expression::InterfaceCast { interface_name: _, program_id } => {
                // InterfaceCast is typically used in method call chains (IERC20(addr).transfer(...))
                // and converted to CpiCall. If used standalone, just return the program_id.
                self.generate_expression(program_id)
            }
            Expression::TokenTransfer { from, to, authority, amount } => {
                let from_str = self.generate_expression(from)?;
                let to_str = self.generate_expression(to)?;
                let auth_str = self.generate_expression(authority)?;
                let amt_str = self.generate_expression(amount)?;
                // Note: In a real implementation, these would be account references from ctx.accounts
                // For now, we generate the CPI pattern - the developer needs to adjust account types
                Ok(format!(
                    r#"{{
            let cpi_accounts = anchor_spl::token::Transfer {{
                from: ctx.accounts.{}.to_account_info(),
                to: ctx.accounts.{}.to_account_info(),
                authority: ctx.accounts.{}.to_account_info(),
            }};
            let cpi_program = ctx.accounts.token_program.to_account_info();
            anchor_spl::token::transfer(CpiContext::new(cpi_program, cpi_accounts), {} as u64)?
        }}"#,
                    to_snake_case(&from_str), to_snake_case(&to_str), to_snake_case(&auth_str), amt_str
                ))
            }
            Expression::TokenMint { mint, to, authority, amount } => {
                let mint_str = self.generate_expression(mint)?;
                let to_str = self.generate_expression(to)?;
                let auth_str = self.generate_expression(authority)?;
                let amt_str = self.generate_expression(amount)?;
                Ok(format!(
                    r#"{{
            let cpi_accounts = anchor_spl::token::MintTo {{
                mint: ctx.accounts.{}.to_account_info(),
                to: ctx.accounts.{}.to_account_info(),
                authority: ctx.accounts.{}.to_account_info(),
            }};
            let cpi_program = ctx.accounts.token_program.to_account_info();
            anchor_spl::token::mint_to(CpiContext::new(cpi_program, cpi_accounts), {} as u64)?
        }}"#,
                    to_snake_case(&mint_str), to_snake_case(&to_str), to_snake_case(&auth_str), amt_str
                ))
            }
            Expression::TokenBurn { from, mint, authority, amount } => {
                let from_str = self.generate_expression(from)?;
                let mint_str = self.generate_expression(mint)?;
                let auth_str = self.generate_expression(authority)?;
                let amt_str = self.generate_expression(amount)?;
                Ok(format!(
                    r#"{{
            let cpi_accounts = anchor_spl::token::Burn {{
                from: ctx.accounts.{}.to_account_info(),
                mint: ctx.accounts.{}.to_account_info(),
                authority: ctx.accounts.{}.to_account_info(),
            }};
            let cpi_program = ctx.accounts.token_program.to_account_info();
            anchor_spl::token::burn(CpiContext::new(cpi_program, cpi_accounts), {} as u64)?
        }}"#,
                    to_snake_case(&from_str), to_snake_case(&mint_str), to_snake_case(&auth_str), amt_str
                ))
            }
            Expression::GetATA { owner, mint } => {
                let owner_str = self.generate_expression(owner)?;
                let mint_str = self.generate_expression(mint)?;
                Ok(format!(
                    "anchor_spl::associated_token::get_associated_token_address(&{}, &{})",
                    owner_str, mint_str
                ))
            }
            Expression::Index { expr, index } => {
                let e = self.generate_expression(expr)?;
                let i = self.generate_expression(index)?;
                // Cast index to usize for array/vec indexing
                Ok(format!("{}[{} as usize]", e, i))
            }
            Expression::Field { expr, field } => {
                let e = self.generate_expression(expr)?;
                // Convert Solidity's .length to Rust's .len() with cast to u128
                if field == "length" {
                    Ok(format!("({}.len() as u128)", e))
                } else {
                    Ok(format!("{}.{}", e, to_snake_case(field)))
                }
            }
            Expression::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                let c = self.generate_expression(condition)?;
                let t = self.generate_expression(then_expr)?;
                let e = self.generate_expression(else_expr)?;
                Ok(format!("if {} {{ {} }} else {{ {} }}", c, t, e))
            }
            Expression::Assert { condition, message } => {
                let c = self.generate_expression(condition)?;
                if let Some(msg) = message {
                    Ok(format!("assert!({}, \"{}\")", c, msg))
                } else {
                    Ok(format!("assert!({})", c))
                }
            }
            Expression::AssertEq { left, right, message } => {
                let l = self.generate_expression(left)?;
                let r = self.generate_expression(right)?;
                if let Some(msg) = message {
                    Ok(format!("assert_eq!({}, {}, \"{}\")", l, r, msg))
                } else {
                    Ok(format!("assert_eq!({}, {})", l, r))
                }
            }
            Expression::AssertNe { left, right, message } => {
                let l = self.generate_expression(left)?;
                let r = self.generate_expression(right)?;
                if let Some(msg) = message {
                    Ok(format!("assert_ne!({}, {}, \"{}\")", l, r, msg))
                } else {
                    Ok(format!("assert_ne!({}, {})", l, r))
                }
            }
            Expression::AssertGt { left, right, message } => {
                let l = self.generate_expression(left)?;
                let r = self.generate_expression(right)?;
                if let Some(msg) = message {
                    Ok(format!("assert!({} > {}, \"{}\")", l, r, msg))
                } else {
                    Ok(format!("assert!({} > {})", l, r))
                }
            }
            Expression::AssertGe { left, right, message } => {
                let l = self.generate_expression(left)?;
                let r = self.generate_expression(right)?;
                if let Some(msg) = message {
                    Ok(format!("assert!({} >= {}, \"{}\")", l, r, msg))
                } else {
                    Ok(format!("assert!({} >= {})", l, r))
                }
            }
            Expression::AssertLt { left, right, message } => {
                let l = self.generate_expression(left)?;
                let r = self.generate_expression(right)?;
                if let Some(msg) = message {
                    Ok(format!("assert!({} < {}, \"{}\")", l, r, msg))
                } else {
                    Ok(format!("assert!({} < {})", l, r))
                }
            }
            Expression::AssertLe { left, right, message } => {
                let l = self.generate_expression(left)?;
                let r = self.generate_expression(right)?;
                if let Some(msg) = message {
                    Ok(format!("assert!({} <= {}, \"{}\")", l, r, msg))
                } else {
                    Ok(format!("assert!({} <= {})", l, r))
                }
            }
        }
    }

    fn generate_literal(&self, lit: &Literal) -> Result<String, CodegenError> {
        match lit {
            Literal::Bool(b) => Ok(b.to_string()),
            Literal::Int(n) => Ok(format!("{}i128", n)),
            Literal::Uint(n) => Ok(format!("{}u128", n)),
            Literal::String(s) => Ok(format!("\"{}\"", s.replace('\"', "\\\""))),
            Literal::Pubkey(s) => {
                // For address literals, we'd need to parse or use a placeholder
                Ok(format!("Pubkey::default() /* {} */", s))
            }
            Literal::ZeroAddress => {
                // address(0) - the zero/null address
                Ok("Pubkey::default()".to_string())
            }
            Literal::ZeroBytes(n) => {
                // bytes32(0), bytes4(0), etc. - zero-filled fixed bytes
                Ok(format!("[0u8; {}]", n))
            }
        }
    }

    fn generate_state_rs(&self, program: &SolanaProgram) -> Result<String, CodegenError> {
        let mut content = String::from(
            r#"//! Program state definitions

use anchor_lang::prelude::*;

"#,
        );

        // Generate user-defined enums
        for enum_def in &program.enums {
            content.push_str("#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]\n");
            content.push_str(&format!("pub enum {} {{\n", to_pascal_case(&enum_def.name)));
            for (i, variant) in enum_def.variants.iter().enumerate() {
                if i == 0 {
                    content.push_str("    #[default]\n");
                }
                content.push_str(&format!("    {},\n", to_pascal_case(variant)));
            }
            content.push_str("}\n\n");
        }

        // Generate user-defined structs (before state account so they can be used as field types)
        for struct_def in &program.structs {
            content.push_str(&format!(
                "#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]\n"
            ));
            content.push_str(&format!("pub struct {} {{\n", to_pascal_case(&struct_def.name)));
            for field in &struct_def.fields {
                content.push_str(&format!(
                    "    pub {}: {},\n",
                    to_snake_case(&field.name),
                    self.type_to_rust(&field.ty)
                ));
            }
            content.push_str("}\n\n");
        }

        // Generate state account struct with InitSpace derive for automatic space calculation
        content.push_str("#[account]\n");
        content.push_str("#[derive(InitSpace)]\n");
        content.push_str(&format!("pub struct {}State {{\n", to_pascal_case(&program.name)));

        for field in &program.state.fields {
            // Add #[max_len] attribute for dynamic types (String, Vec, etc.)
            if let Some(max_len_attr) = self.get_max_len_attribute(&field.ty) {
                content.push_str(&format!("    {}\n", max_len_attr));
            }
            content.push_str(&format!(
                "    pub {}: {},\n",
                to_snake_case(&field.name),
                self.type_to_rust(&field.ty)
            ));
        }

        content.push_str("}\n\n");

        // Generate mapping entry account structs with InitSpace
        for mapping in &program.mappings {
            let struct_name = format!("{}Entry", to_pascal_case(&mapping.name));
            // For nested mappings, get the innermost value type
            let innermost_ty = self.innermost_value_type(&mapping.value_ty);
            let value_type = self.type_to_rust(&innermost_ty);
            let key_type = self.type_to_rust(&mapping.key_ty);

            // Check if key or value need max_len attributes
            let key_max_len = self.get_max_len_attribute(&mapping.key_ty);
            let value_max_len = self.get_max_len_attribute(&innermost_ty);

            content.push_str(&format!(
                "/// PDA account for {} mapping entries\n#[account]\n#[derive(InitSpace)]\npub struct {} {{\n",
                mapping.name,
                struct_name,
            ));

            // Key field with optional max_len
            if let Some(attr) = key_max_len {
                content.push_str(&format!("    {}\n", attr));
            }
            content.push_str(&format!("    /// The key for this entry\n    pub key: {},\n", key_type));

            // Value field with optional max_len
            if let Some(attr) = value_max_len {
                content.push_str(&format!("    {}\n", attr));
            }
            content.push_str(&format!("    /// The value stored at this key\n    pub value: {},\n", value_type));

            content.push_str("}\n\n");
        }

        Ok(content)
    }

    /// Generate #[max_len(...)] attribute for dynamic types
    /// Returns None for fixed-size types that don't need the attribute
    fn get_max_len_attribute(&self, ty: &SolanaType) -> Option<String> {
        match ty {
            SolanaType::String => Some("#[max_len(200)]".to_string()),
            SolanaType::Bytes => Some("#[max_len(1000)]".to_string()),
            SolanaType::Vec(elem) => {
                // For Vec<T>, we need max_len for the outer vec
                // and potentially nested max_len for the element if it's dynamic
                if self.get_max_len_attribute(elem).is_some() {
                    // Nested dynamic type - use (outer_len, inner_len) format
                    let inner_len = match elem.as_ref() {
                        SolanaType::String => 200,
                        SolanaType::Bytes => 1000,
                        _ => 100,
                    };
                    Some(format!("#[max_len(100, {})]", inner_len))
                } else {
                    Some("#[max_len(100)]".to_string())
                }
            }
            SolanaType::Option(inner) => self.get_max_len_attribute(inner),
            _ => None, // Fixed-size types don't need max_len
        }
    }

    fn generate_instructions_rs(&self, program: &SolanaProgram) -> Result<String, CodegenError> {
        // Check if any public instruction uses token program
        let uses_token = program.instructions.iter()
            .filter(|i| i.is_public)
            .any(|i| i.uses_token_program);

        let mut content = String::from("//! Instruction account contexts\n\nuse anchor_lang::prelude::*;\n");

        if uses_token {
            content.push_str("use anchor_spl::token::Token;\n");
        }

        content.push_str("use crate::state::*;\n\n");

        // Generate context struct only for public instructions
        for instruction in &program.instructions {
            if instruction.is_public {
                content.push_str(&self.generate_context_struct(instruction, program)?);
                content.push('\n');
            }
        }

        Ok(content)
    }

    fn generate_context_struct(
        &self,
        instruction: &Instruction,
        program: &SolanaProgram,
    ) -> Result<String, CodegenError> {
        let name = to_pascal_case(&instruction.name);
        let state_name = format!("{}State", to_pascal_case(&program.name));

        // Collect instruction params used in mapping seeds
        let mut seed_params: Vec<(&String, &SolanaType)> = Vec::new();
        for access in &instruction.mapping_accesses {
            for key_expr in &access.key_exprs {
                self.collect_seed_params(key_expr, instruction, &mut seed_params);
            }
        }

        let mut content = String::new();
        content.push_str("#[derive(Accounts)]\n");

        // Add #[instruction(...)] if there are params used in seeds
        if !seed_params.is_empty() {
            let params_str: Vec<String> = seed_params
                .iter()
                .map(|(name, ty)| format!("{}: {}", to_snake_case(name), self.type_to_rust(ty)))
                .collect();
            content.push_str(&format!("#[instruction({})]\n", params_str.join(", ")));
        }

        content.push_str(&format!("pub struct {}<'info> {{\n", name));

        // State account
        if instruction.name == "initialize" {
            content.push_str(&format!(
                r#"    #[account(
        init,
        payer = signer,
        space = 8 + {}::INIT_SPACE
    )]
    pub state: Account<'info, {}>,
"#,
                state_name, state_name
            ));
        } else if instruction.is_view {
            content.push_str(&format!(
                "    pub state: Account<'info, {}>,\n",
                state_name
            ));
        } else if instruction.closes_state {
            // Selfdestruct: close the state account and send rent to signer
            content.push_str(&format!(
                "    #[account(mut, close = signer)]\n    pub state: Account<'info, {}>,\n",
                state_name
            ));
        } else {
            content.push_str(&format!(
                "    #[account(mut)]\n    pub state: Account<'info, {}>,\n",
                state_name
            ));
        }

        // Signer
        content.push_str("    #[account(mut)]\n");
        content.push_str("    pub signer: Signer<'info>,\n");

        // Add additional signers for parameters with Signer type
        for param in &instruction.params {
            if matches!(param.ty, SolanaType::Signer) {
                content.push_str(&format!(
                    "    pub {}: Signer<'info>,\n",
                    to_snake_case(&param.name)
                ));
            }
        }

        // Add PDA accounts for mapping accesses
        for access in &instruction.mapping_accesses {
            let entry_type = format!("{}Entry", to_pascal_case(&access.mapping_name));
            let account_name = to_snake_case(&access.account_name);

            // Generate the key expressions for seeds (handles nested mappings)
            let key_seeds: Vec<String> = access
                .key_exprs
                .iter()
                .map(|k| self.generate_key_seed_expr(k))
                .collect::<Result<Vec<_>, _>>()?;
            let seeds_str = key_seeds.iter().map(|s| format!("{}.as_ref()", s)).collect::<Vec<_>>().join(", ");

            if access.is_write {
                // Use init_if_needed for write accesses
                content.push_str(&format!(
                    r#"    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + {}::INIT_SPACE,
        seeds = [b"{}", {}],
        bump
    )]
    pub {}: Account<'info, {}>,
"#,
                    entry_type,
                    to_snake_case(&access.mapping_name),
                    seeds_str,
                    account_name,
                    entry_type
                ));
            } else {
                // Read-only access
                content.push_str(&format!(
                    r#"    #[account(
        seeds = [b"{}", {}],
        bump
    )]
    pub {}: Account<'info, {}>,
"#,
                    to_snake_case(&access.mapping_name),
                    seeds_str,
                    account_name,
                    entry_type
                ));
            }
        }

        // System program (needed if any init_if_needed is used, or for payable functions)
        let needs_system_program = instruction.name == "initialize"
            || instruction.mapping_accesses.iter().any(|a| a.is_write)
            || instruction.is_payable;
        if needs_system_program {
            content.push_str("    pub system_program: Program<'info, System>,\n");
        }

        // Token program (needed if any SPL token operations are used)
        if instruction.uses_token_program {
            content.push_str("    pub token_program: Program<'info, Token>,\n");
        }

        content.push_str("}\n");

        Ok(content)
    }

    /// Generate the seed expression for a mapping key (used in #[account] attributes)
    fn generate_key_seed_expr(&self, key_expr: &Expression) -> Result<String, CodegenError> {
        match key_expr {
            // In account attributes, we reference accounts directly without ctx.accounts prefix
            Expression::MsgSender => Ok("signer.key()".to_string()),
            Expression::Var(name) => Ok(to_snake_case(name)),
            Expression::Literal(Literal::Pubkey(s)) => Ok(format!("Pubkey::default() /* {} */", s)),
            Expression::Literal(Literal::ZeroAddress) => Ok("Pubkey::default()".to_string()),
            Expression::Literal(Literal::ZeroBytes(n)) => Ok(format!("[0u8; {}]", n)),
            Expression::StateAccess(field) => {
                // For state field access in seeds, reference via state account
                Ok(format!("state.{}", to_snake_case(field)))
            }
            _ => {
                // For other expressions, try to simplify for seed context
                // This may need refinement for complex cases
                let expr_str = self.generate_expression(key_expr)?;
                // Remove ctx.accounts. prefix if present
                Ok(expr_str.replace("ctx.accounts.", ""))
            }
        }
    }

    /// Collect instruction parameters that are used in seed expressions
    fn collect_seed_params<'a>(
        &self,
        key_expr: &'a Expression,
        instruction: &'a Instruction,
        params: &mut Vec<(&'a String, &'a SolanaType)>,
    ) {
        match key_expr {
            Expression::Var(name) => {
                // Check if this var is an instruction parameter
                if let Some(param) = instruction.params.iter().find(|p| &p.name == name) {
                    // Avoid duplicates
                    if !params.iter().any(|(n, _)| *n == name) {
                        params.push((&param.name, &param.ty));
                    }
                }
            }
            Expression::MethodCall { receiver, args, .. } => {
                self.collect_seed_params(receiver, instruction, params);
                for arg in args {
                    self.collect_seed_params(arg, instruction, params);
                }
            }
            Expression::Binary { left, right, .. } => {
                self.collect_seed_params(left, instruction, params);
                self.collect_seed_params(right, instruction, params);
            }
            _ => {}
        }
    }

    fn generate_error_rs(&self, program: &SolanaProgram) -> Result<String, CodegenError> {
        let mut content = String::from(
            r#"//! Custom error definitions

use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Requirement failed")]
    RequireFailed,
"#,
        );

        // Add custom errors from the program
        for error in &program.errors {
            content.push_str(&format!(
                "    #[msg(\"{}\")]\n    {},\n",
                error.name, to_pascal_case(&error.name)
            ));
        }

        content.push_str("}\n");

        Ok(content)
    }

    fn generate_events_rs(&self, program: &SolanaProgram) -> Result<String, CodegenError> {
        let mut content = String::from(
            r#"//! Event definitions

use anchor_lang::prelude::*;

"#,
        );

        for event in &program.events {
            content.push_str("#[event]\n");
            content.push_str(&format!("pub struct {} {{\n", to_pascal_case(&event.name)));

            for field in &event.fields {
                // Note: #[index] is not supported in Anchor events
                // Indexed fields in Solidity become regular fields
                content.push_str(&format!(
                    "    pub {}: {},\n",
                    to_snake_case(&field.name),
                    self.type_to_rust(&field.ty)
                ));
            }

            content.push_str("}\n\n");
        }

        Ok(content)
    }

    fn generate_anchor_toml(&self, program: &SolanaProgram) -> String {
        let name = to_snake_case(&program.name);
        format!(
            r#"[features]
seeds = false
skip-lint = false

[programs.localnet]
{} = "11111111111111111111111111111111"

[registry]
url = "https://api.apr.dev"

[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
"#,
            name
        )
    }

    fn generate_cargo_toml(&self, program: &SolanaProgram) -> String {
        let name = to_snake_case(&program.name);
        let uses_token = program.instructions.iter().any(|i| i.uses_token_program);

        let mut deps = String::from("anchor-lang = { version = \"0.32.0\", features = [\"init-if-needed\"] }\n");
        if uses_token {
            deps.push_str("anchor-spl = \"0.32.0\"\n");
        }

        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
description = "Generated by SolScript compiler"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]
name = "{}"

[features]
no-entrypoint = []
no-idl = []
no-log-ix-name = []
cpi = ["no-entrypoint"]
default = []

[dependencies]
{}
"#,
            name, name, deps
        )
    }

    fn type_to_rust(&self, ty: &SolanaType) -> String {
        match ty {
            SolanaType::U8 => "u8".to_string(),
            SolanaType::U16 => "u16".to_string(),
            SolanaType::U32 => "u32".to_string(),
            SolanaType::U64 => "u64".to_string(),
            SolanaType::U128 => "u128".to_string(),
            SolanaType::I8 => "i8".to_string(),
            SolanaType::I16 => "i16".to_string(),
            SolanaType::I32 => "i32".to_string(),
            SolanaType::I64 => "i64".to_string(),
            SolanaType::I128 => "i128".to_string(),
            SolanaType::Bool => "bool".to_string(),
            SolanaType::Pubkey => "Pubkey".to_string(),
            SolanaType::Signer => "Pubkey".to_string(), // Signers are Pubkeys in function params
            SolanaType::String => "String".to_string(),
            SolanaType::Bytes => "Vec<u8>".to_string(),
            SolanaType::FixedBytes(n) => format!("[u8; {}]", n),
            SolanaType::Array(elem, size) => format!("[{}; {}]", self.type_to_rust(elem), size),
            SolanaType::Vec(elem) => format!("Vec<{}>", self.type_to_rust(elem)),
            SolanaType::Option(inner) => format!("Option<{}>", self.type_to_rust(inner)),
            SolanaType::Mapping(_, _) => "/* Mapping - use PDAs */".to_string(),
            SolanaType::Custom(name) => to_pascal_case(name),
        }
    }

    /// Get the innermost value type for nested mappings
    /// For `mapping(A => mapping(B => C))`, returns `C`
    fn innermost_value_type(&self, ty: &SolanaType) -> SolanaType {
        match ty {
            SolanaType::Mapping(_, value_ty) => self.innermost_value_type(value_ty),
            other => other.clone(),
        }
    }

    fn generate_package_json(&self, program: &SolanaProgram) -> String {
        let name = to_snake_case(&program.name);
        format!(
            r#"{{
  "name": "{}-client",
  "version": "0.1.0",
  "description": "Generated client for {} Solana program",
  "main": "app/client.ts",
  "scripts": {{
    "test": "anchor test",
    "build": "anchor build",
    "deploy": "anchor deploy"
  }},
  "dependencies": {{
    "@coral-xyz/anchor": "^0.32.0",
    "@solana/web3.js": "^1.95.0"
  }},
  "devDependencies": {{
    "@types/chai": "^4.3.0",
    "@types/mocha": "^10.0.0",
    "chai": "^4.3.0",
    "mocha": "^10.2.0",
    "ts-mocha": "^10.0.0",
    "typescript": "^5.0.0"
  }}
}}
"#,
            name, program.name
        )
    }

    fn generate_readme(&self, program: &SolanaProgram) -> String {
        let name = &program.name;
        let _snake_name = to_snake_case(name);

        // Count public functions
        let public_fns: Vec<&str> = program.instructions
            .iter()
            .filter(|i| i.is_public)
            .map(|i| i.name.as_str())
            .collect();

        let fn_list = public_fns
            .iter()
            .map(|f| format!("- `{}`", to_snake_case(f)))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"# {} Solana Program

Generated by [SolScript](https://github.com/example/solscript) compiler.

## Overview

This is an Anchor-based Solana program with a TypeScript client.

## Project Structure

```
.
├── programs/
│   └── solscript_program/
│       └── src/
│           ├── lib.rs          # Main program entry
│           ├── state.rs        # Account state definitions
│           ├── instructions.rs # Instruction contexts
│           ├── error.rs        # Custom errors
│           └── events.rs       # Event definitions
├── app/
│   └── client.ts               # TypeScript client
├── tests/
│   └── program.test.ts         # Anchor tests
├── target/
│   └── idl/
│       └── program.json        # Anchor IDL
├── Anchor.toml
├── Cargo.toml
└── package.json
```

## Available Instructions

{}

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/)

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Deploy

```bash
anchor deploy
```

## Usage

See `app/client.ts` for the TypeScript client implementation.

```typescript
import {{ {}Client }} from './app/client';

// Initialize client with provider
const client = new {}Client(provider);

// Call instructions...
```
"#,
            name,
            fn_list,
            to_pascal_case(name),
            to_pascal_case(name)
        )
    }

    fn generate_gitignore(&self) -> String {
        r#"# Anchor
target/
.anchor/
node_modules/

# Rust
Cargo.lock
**/*.rs.bk

# IDE
.idea/
.vscode/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Solana
test-ledger/
.env

# TypeScript
dist/
*.js
*.d.ts
*.map
!anchor.js
"#.to_string()
    }
}

impl Default for RustGenerator {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            signer_params: std::collections::HashSet::new(),
            internal_functions: std::collections::HashSet::new(),
            in_helper_function: false,
        }
    }
}

// Helper functions
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_upper = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_upper = true;
        } else {
            result.push(c);
            prev_upper = false;
        }
    }

    result
}

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}
