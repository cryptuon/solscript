//! AST construction from pest parse tree (Solidity-Style)

use pest::Parser;
use smol_str::SmolStr;
use solscript_ast::*;

use crate::{ParseError, Rule, SolScriptParser};

type Pair<'a> = pest::iterators::Pair<'a, Rule>;

/// Parse a complete SolScript program
pub fn parse_program(source: &str) -> Result<Program, ParseError> {
    let mut pairs = SolScriptParser::parse(Rule::program, source).map_err(|e| {
        let mut err: ParseError = e.into();
        if let ParseError::Syntax { src, .. } = &mut err {
            *src = source.to_string();
        }
        err
    })?;

    let mut items = Vec::new();
    let mut span = Span::dummy();

    // Get the program rule's inner pairs
    let program_pair = pairs.next().unwrap();
    span = span_from_pair(&program_pair);

    for pair in program_pair.into_inner() {
        match pair.as_rule() {
            Rule::item => {
                let item = parse_item(pair.into_inner().next().unwrap())?;
                items.push(item);
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    Ok(Program { items, span })
}

fn span_from_pair(pair: &Pair) -> Span {
    let span = pair.as_span();
    Span::new(span.start(), span.end())
}

fn parse_ident(pair: Pair) -> Ident {
    Ident::new(pair.as_str(), span_from_pair(&pair))
}

fn parse_item(pair: Pair) -> Result<Item, ParseError> {
    match pair.as_rule() {
        Rule::import_stmt => Ok(Item::Import(parse_import(pair)?)),
        Rule::contract_def => Ok(Item::Contract(parse_contract(pair)?)),
        Rule::interface_def => Ok(Item::Interface(parse_interface(pair)?)),
        Rule::struct_def => Ok(Item::Struct(parse_struct(pair)?)),
        Rule::enum_def => Ok(Item::Enum(parse_enum(pair)?)),
        Rule::event_def => Ok(Item::Event(parse_event(pair)?)),
        Rule::error_def => Ok(Item::Error(parse_error_def(pair)?)),
        Rule::function_def => Ok(Item::Function(parse_function(pair)?)),
        _ => unreachable!("Unexpected rule: {:?}", pair.as_rule()),
    }
}

// =============================================================================
// Import parsing
// =============================================================================

fn parse_import(pair: Pair) -> Result<ImportStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut items = Vec::new();
    let mut source = SmolStr::default();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::import_list => {
                for item in inner.into_inner() {
                    if item.as_rule() == Rule::import_item {
                        items.push(parse_import_item(item)?);
                    }
                }
            }
            Rule::string_lit => {
                source = parse_string_content(inner.as_str());
            }
            _ => {}
        }
    }

    Ok(ImportStmt {
        items,
        source,
        span,
    })
}

fn parse_import_item(pair: Pair) -> Result<ImportItem, ParseError> {
    let span = span_from_pair(&pair);
    let mut inner = pair.into_inner();

    let name = parse_ident(inner.next().unwrap());
    let alias = inner.next().map(parse_ident);

    Ok(ImportItem { name, alias, span })
}

fn parse_string_content(s: &str) -> SmolStr {
    // Remove quotes and handle escape sequences
    let s = &s[1..s.len() - 1];
    SmolStr::new(s)
}

// =============================================================================
// Contract parsing
// =============================================================================

fn parse_contract(pair: Pair) -> Result<ContractDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut attributes = Vec::new();
    let mut is_abstract = false;
    let mut name = None;
    let mut bases = Vec::new();
    let mut members = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::attribute => attributes.push(parse_attribute(inner)?),
            Rule::abstract_kw => is_abstract = true,
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::inheritance_list => {
                for tp in inner.into_inner() {
                    if tp.as_rule() == Rule::type_path {
                        bases.push(parse_type_path(tp)?);
                    }
                }
            }
            Rule::contract_member => {
                let member_inner = inner.into_inner().next().unwrap();
                match member_inner.as_rule() {
                    Rule::state_var => {
                        members.push(ContractMember::StateVar(parse_state_var(member_inner)?));
                    }
                    Rule::constructor_def => {
                        members.push(ContractMember::Constructor(parse_constructor(member_inner)?));
                    }
                    Rule::modifier_def => {
                        members.push(ContractMember::Modifier(parse_modifier_def(member_inner)?));
                    }
                    Rule::function_def => {
                        members.push(ContractMember::Function(parse_function(member_inner)?));
                    }
                    Rule::event_def => {
                        members.push(ContractMember::Event(parse_event(member_inner)?));
                    }
                    Rule::error_def => {
                        members.push(ContractMember::Error(parse_error_def(member_inner)?));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    Ok(ContractDef {
        attributes,
        is_abstract,
        name: name.unwrap(),
        bases,
        members,
        span,
    })
}

fn parse_state_var(pair: Pair) -> Result<StateVar, ParseError> {
    let span = span_from_pair(&pair);
    let mut attributes = Vec::new();
    let mut ty = None;
    let mut visibility = None;
    let mut name = None;
    let mut initializer = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::attribute => attributes.push(parse_attribute(inner)?),
            Rule::type_expr => ty = Some(parse_type_expr(inner)?),
            Rule::visibility => visibility = Some(parse_visibility(inner)),
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::expr => initializer = Some(parse_expr(inner)?),
            _ => {}
        }
    }

    Ok(StateVar {
        attributes,
        ty: ty.unwrap(),
        visibility,
        name: name.unwrap(),
        initializer,
        span,
    })
}

fn parse_constructor(pair: Pair) -> Result<ConstructorDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut params = Vec::new();
    let mut modifiers = Vec::new();
    let mut body = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::param_list => params = parse_param_list(inner)?,
            Rule::modifier_invocation => modifiers.push(parse_modifier_invocation(inner)?),
            Rule::block => body = Some(parse_block(inner)?),
            _ => {}
        }
    }

    Ok(ConstructorDef {
        params,
        modifiers,
        body: body.unwrap(),
        span,
    })
}

fn parse_modifier_def(pair: Pair) -> Result<ModifierDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut name = None;
    let mut params = Vec::new();
    let mut body = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::param_list => params = parse_param_list(inner)?,
            Rule::modifier_block => body = Some(parse_modifier_block(inner)?),
            _ => {}
        }
    }

    Ok(ModifierDef {
        name: name.unwrap(),
        params,
        body: body.unwrap(),
        span,
    })
}

fn parse_modifier_block(pair: Pair) -> Result<Block, ParseError> {
    let span = span_from_pair(&pair);
    let mut stmts = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::modifier_stmt {
            stmts.push(parse_modifier_stmt(inner)?);
        }
    }

    Ok(Block { stmts, span })
}

fn parse_modifier_stmt(pair: Pair) -> Result<Stmt, ParseError> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::placeholder_stmt => Ok(Stmt::Placeholder(span_from_pair(&inner))),
        Rule::var_decl_stmt => Ok(Stmt::VarDecl(parse_var_decl_stmt(inner)?)),
        Rule::return_stmt => Ok(Stmt::Return(parse_return_stmt(inner)?)),
        Rule::if_stmt => Ok(Stmt::If(parse_if_stmt(inner)?)),
        Rule::require_stmt => Ok(Stmt::Require(parse_require_stmt(inner)?)),
        Rule::revert_stmt => Ok(Stmt::Revert(parse_revert_stmt(inner)?)),
        Rule::expr_stmt => Ok(Stmt::Expr(parse_expr_stmt(inner)?)),
        _ => unreachable!("Unexpected modifier_stmt rule: {:?}", inner.as_rule()),
    }
}

// =============================================================================
// Interface parsing
// =============================================================================

fn parse_interface(pair: Pair) -> Result<InterfaceDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut attributes = Vec::new();
    let mut name = None;
    let mut bases = Vec::new();
    let mut members = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::attribute => attributes.push(parse_attribute(inner)?),
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::inheritance_list => {
                for tp in inner.into_inner() {
                    if tp.as_rule() == Rule::type_path {
                        bases.push(parse_type_path(tp)?);
                    }
                }
            }
            Rule::interface_member => {
                // interface_member = { function_sig ~ ";" }
                let sig_pair = inner.into_inner().next().unwrap();
                members.push(parse_function_sig(sig_pair)?);
            }
            _ => {}
        }
    }

    Ok(InterfaceDef {
        attributes,
        name: name.unwrap(),
        bases,
        members,
        span,
    })
}

fn parse_function_sig(pair: Pair) -> Result<FnSig, ParseError> {
    let span = span_from_pair(&pair);
    let mut name = None;
    let mut generic_params = None;
    let mut params = Vec::new();
    let mut visibility = None;
    let mut state_mutability = Vec::new();
    let mut modifiers = Vec::new();
    let mut return_params = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::generic_params => generic_params = Some(parse_generic_params(inner)?),
            Rule::param_list => params = parse_param_list(inner)?,
            Rule::visibility => visibility = Some(parse_visibility(inner)),
            Rule::state_mutability => state_mutability.push(parse_state_mutability(inner)),
            Rule::modifier_invocation => modifiers.push(parse_modifier_invocation(inner)?),
            Rule::returns_clause => return_params = parse_returns_clause(inner)?,
            _ => {}
        }
    }

    Ok(FnSig {
        name: name.unwrap(),
        generic_params,
        params,
        visibility,
        state_mutability,
        modifiers,
        return_params,
        span,
    })
}

// =============================================================================
// Struct parsing
// =============================================================================

fn parse_struct(pair: Pair) -> Result<StructDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut attributes = Vec::new();
    let mut name = None;
    let mut generic_params = None;
    let mut fields = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::attribute => attributes.push(parse_attribute(inner)?),
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::generic_params => generic_params = Some(parse_generic_params(inner)?),
            Rule::struct_field => fields.push(parse_struct_field(inner)?),
            _ => {}
        }
    }

    Ok(StructDef {
        attributes,
        name: name.unwrap(),
        generic_params,
        fields,
        span,
    })
}

fn parse_struct_field(pair: Pair) -> Result<StructField, ParseError> {
    let span = span_from_pair(&pair);
    let mut inner = pair.into_inner();

    // Solidity-style: type name;
    let ty = parse_type_expr(inner.next().unwrap())?;
    let name = parse_ident(inner.next().unwrap());

    Ok(StructField { ty, name, span })
}

// =============================================================================
// Enum parsing
// =============================================================================

fn parse_enum(pair: Pair) -> Result<EnumDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut attributes = Vec::new();
    let mut name = None;
    let mut variants = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::attribute => attributes.push(parse_attribute(inner)?),
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::enum_variant => variants.push(parse_enum_variant(inner)?),
            _ => {}
        }
    }

    Ok(EnumDef {
        attributes,
        name: name.unwrap(),
        variants,
        span,
    })
}

fn parse_enum_variant(pair: Pair) -> Result<EnumVariant, ParseError> {
    let span = span_from_pair(&pair);
    let name = parse_ident(pair.into_inner().next().unwrap());

    Ok(EnumVariant { name, span })
}

// =============================================================================
// Event & Error parsing
// =============================================================================

fn parse_event(pair: Pair) -> Result<EventDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut name = None;
    let mut params = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::event_param => params.push(parse_event_param(inner)?),
            _ => {}
        }
    }

    Ok(EventDef {
        name: name.unwrap(),
        params,
        span,
    })
}

fn parse_event_param(pair: Pair) -> Result<EventParam, ParseError> {
    let span = span_from_pair(&pair);
    let mut ty = None;
    let mut indexed = false;
    let mut name = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::type_expr => ty = Some(parse_type_expr(inner)?),
            Rule::indexed_kw => indexed = true,
            Rule::ident => name = Some(parse_ident(inner)),
            _ => {}
        }
    }

    Ok(EventParam {
        ty: ty.unwrap(),
        indexed,
        name: name.unwrap(),
        span,
    })
}

fn parse_error_def(pair: Pair) -> Result<ErrorDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut name = None;
    let mut params = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::error_param => params.push(parse_error_param(inner)?),
            _ => {}
        }
    }

    Ok(ErrorDef {
        name: name.unwrap(),
        params,
        span,
    })
}

fn parse_error_param(pair: Pair) -> Result<ErrorParam, ParseError> {
    let span = span_from_pair(&pair);
    let mut inner = pair.into_inner();

    let ty = parse_type_expr(inner.next().unwrap())?;
    let name = parse_ident(inner.next().unwrap());

    Ok(ErrorParam { ty, name, span })
}

// =============================================================================
// Function parsing
// =============================================================================

fn parse_function(pair: Pair) -> Result<FnDef, ParseError> {
    let span = span_from_pair(&pair);
    let mut attributes = Vec::new();
    let mut name = None;
    let mut generic_params = None;
    let mut params = Vec::new();
    let mut visibility = None;
    let mut state_mutability = Vec::new();
    let mut modifiers = Vec::new();
    let mut return_params = Vec::new();
    let mut body = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::attribute => attributes.push(parse_attribute(inner)?),
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::generic_params => generic_params = Some(parse_generic_params(inner)?),
            Rule::param_list => params = parse_param_list(inner)?,
            Rule::visibility => visibility = Some(parse_visibility(inner)),
            Rule::state_mutability => state_mutability.push(parse_state_mutability(inner)),
            Rule::modifier_invocation => modifiers.push(parse_modifier_invocation(inner)?),
            Rule::returns_clause => return_params = parse_returns_clause(inner)?,
            Rule::block => body = Some(parse_block(inner)?),
            _ => {}
        }
    }

    Ok(FnDef {
        attributes,
        name: name.unwrap(),
        generic_params,
        params,
        visibility,
        state_mutability,
        modifiers,
        return_params,
        body,  // None for abstract functions (semicolon instead of block)
        span,
    })
}

fn parse_visibility(pair: Pair) -> Visibility {
    match pair.as_str() {
        "public" => Visibility::Public,
        "private" => Visibility::Private,
        "internal" => Visibility::Internal,
        "external" => Visibility::External,
        _ => unreachable!(),
    }
}

fn parse_state_mutability(pair: Pair) -> StateMutability {
    match pair.as_str() {
        "view" => StateMutability::View,
        "pure" => StateMutability::Pure,
        "payable" => StateMutability::Payable,
        _ => unreachable!(),
    }
}

fn parse_modifier_invocation(pair: Pair) -> Result<ModifierInvocation, ParseError> {
    let span = span_from_pair(&pair);
    let mut name = None;
    let mut args = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::arg_list => args = parse_arg_list(inner)?,
            _ => {}
        }
    }

    Ok(ModifierInvocation {
        name: name.unwrap(),
        args,
        span,
    })
}

fn parse_returns_clause(pair: Pair) -> Result<Vec<ReturnParam>, ParseError> {
    let mut return_params = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::return_param_list {
            for rp in inner.into_inner() {
                if rp.as_rule() == Rule::return_param {
                    return_params.push(parse_return_param(rp)?);
                }
            }
        }
    }

    Ok(return_params)
}

fn parse_return_param(pair: Pair) -> Result<ReturnParam, ParseError> {
    let span = span_from_pair(&pair);
    let mut ty = None;
    let mut name = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::type_expr => ty = Some(parse_type_expr(inner)?),
            Rule::ident => name = Some(parse_ident(inner)),
            _ => {}
        }
    }

    Ok(ReturnParam {
        ty: ty.unwrap(),
        name,
        span,
    })
}

fn parse_param_list(pair: Pair) -> Result<Vec<Param>, ParseError> {
    let mut params = Vec::new();
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::param {
            params.push(parse_param(inner)?);
        }
    }
    Ok(params)
}

fn parse_param(pair: Pair) -> Result<Param, ParseError> {
    let span = span_from_pair(&pair);
    let mut ty = None;
    let mut storage_location = None;
    let mut name = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::type_expr => ty = Some(parse_type_expr(inner)?),
            Rule::storage_location => storage_location = Some(parse_storage_location(inner)),
            Rule::ident => name = Some(parse_ident(inner)),
            _ => {}
        }
    }

    Ok(Param {
        ty: ty.unwrap(),
        storage_location,
        name: name.unwrap(),
        span,
    })
}

fn parse_storage_location(pair: Pair) -> StorageLocation {
    match pair.as_str() {
        "memory" => StorageLocation::Memory,
        "storage" => StorageLocation::Storage,
        "calldata" => StorageLocation::Calldata,
        _ => unreachable!(),
    }
}

// =============================================================================
// Generics parsing
// =============================================================================

fn parse_generic_params(pair: Pair) -> Result<GenericParams, ParseError> {
    let span = span_from_pair(&pair);
    let mut params = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::generic_param {
            params.push(parse_generic_param(inner)?);
        }
    }

    Ok(GenericParams { params, span })
}

fn parse_generic_param(pair: Pair) -> Result<GenericParam, ParseError> {
    let span = span_from_pair(&pair);
    let mut name = None;
    let mut bounds = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::type_expr => bounds.push(parse_type_expr(inner)?),
            _ => {}
        }
    }

    Ok(GenericParam {
        name: name.unwrap(),
        bounds,
        span,
    })
}

fn parse_generic_args(pair: Pair) -> Result<GenericArgs, ParseError> {
    let span = span_from_pair(&pair);
    let mut args = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::type_expr {
            args.push(parse_type_expr(inner)?);
        }
    }

    Ok(GenericArgs { args, span })
}

// =============================================================================
// Attribute parsing
// =============================================================================

fn parse_attribute(pair: Pair) -> Result<Attribute, ParseError> {
    let span = span_from_pair(&pair);
    let inner = pair.into_inner().next().unwrap(); // attribute_inner

    let mut name = None;
    let mut args = Vec::new();

    for p in inner.into_inner() {
        match p.as_rule() {
            Rule::ident => name = Some(parse_ident(p)),
            Rule::attribute_args => {
                for arg in p.into_inner() {
                    if arg.as_rule() == Rule::attribute_arg {
                        args.push(parse_attribute_arg(arg)?);
                    }
                }
            }
            _ => {}
        }
    }

    Ok(Attribute {
        name: name.unwrap(),
        args,
        span,
    })
}

fn parse_attribute_arg(pair: Pair) -> Result<AttributeArg, ParseError> {
    let span = span_from_pair(&pair);
    let mut name = None;
    let mut value = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => {
                if name.is_none() && value.is_none() {
                    // Could be name or value
                    let ident = parse_ident(inner);
                    value = Some(AttributeValue::Ident(ident));
                } else {
                    name = value.take().and_then(|v| {
                        if let AttributeValue::Ident(i) = v {
                            Some(i)
                        } else {
                            None
                        }
                    });
                    value = Some(AttributeValue::Ident(parse_ident(inner)));
                }
            }
            Rule::literal => {
                value = Some(AttributeValue::Literal(parse_literal(inner)?));
            }
            Rule::string_lit => {
                let s = parse_string_content(inner.as_str());
                value = Some(AttributeValue::Literal(Literal::String(s, span_from_pair(&inner))));
            }
            _ => {}
        }
    }

    Ok(AttributeArg {
        name,
        value: value.unwrap(),
        span,
    })
}

// =============================================================================
// Type expression parsing
// =============================================================================

fn parse_type_expr(pair: Pair) -> Result<TypeExpr, ParseError> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::mapping_type => {
            let span = span_from_pair(&inner);
            let mut inner_iter = inner.into_inner();
            let key = parse_type_expr(inner_iter.next().unwrap())?;
            let value = parse_type_expr(inner_iter.next().unwrap())?;
            Ok(TypeExpr::Mapping(Box::new(MappingType { key, value, span })))
        }
        Rule::array_type => {
            let span = span_from_pair(&inner);
            let mut element = None;
            let mut sizes = Vec::new();

            for p in inner.into_inner() {
                match p.as_rule() {
                    Rule::type_path => element = Some(parse_type_path(p)?),
                    Rule::array_size => {
                        let size: u64 = p.as_str().parse().unwrap_or(0);
                        sizes.push(Some(size));
                    }
                    _ => {
                        // Empty brackets []
                        if sizes.is_empty() || sizes.last() != Some(&None) {
                            // Only add None if we haven't just added one
                        }
                    }
                }
            }

            // If no explicit sizes were found, it's a dynamic array
            if sizes.is_empty() {
                sizes.push(None);
            }

            Ok(TypeExpr::Array(Box::new(ArrayType {
                element: element.unwrap(),
                sizes,
                span,
            })))
        }
        Rule::type_tuple => {
            let span = span_from_pair(&inner);
            let mut elements = Vec::new();
            for p in inner.into_inner() {
                if p.as_rule() == Rule::type_expr {
                    elements.push(parse_type_expr(p)?);
                }
            }
            Ok(TypeExpr::Tuple(TypeTuple { elements, span }))
        }
        Rule::type_path => Ok(TypeExpr::Path(parse_type_path(inner)?)),
        _ => unreachable!("Unexpected type rule: {:?}", inner.as_rule()),
    }
}

fn parse_type_path(pair: Pair) -> Result<TypePath, ParseError> {
    let span = span_from_pair(&pair);
    let mut segments = Vec::new();
    let mut generic_args = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => segments.push(parse_ident(inner)),
            Rule::generic_args => generic_args = Some(parse_generic_args(inner)?),
            _ => {}
        }
    }

    Ok(TypePath {
        segments,
        generic_args,
        span,
    })
}

// =============================================================================
// Statement parsing
// =============================================================================

fn parse_block(pair: Pair) -> Result<Block, ParseError> {
    let span = span_from_pair(&pair);
    let mut stmts = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::stmt {
            stmts.push(parse_stmt(inner)?);
        }
    }

    Ok(Block { stmts, span })
}

fn parse_stmt(pair: Pair) -> Result<Stmt, ParseError> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::var_decl_stmt => Ok(Stmt::VarDecl(parse_var_decl_stmt(inner)?)),
        Rule::return_stmt => Ok(Stmt::Return(parse_return_stmt(inner)?)),
        Rule::if_stmt => Ok(Stmt::If(parse_if_stmt(inner)?)),
        Rule::while_stmt => Ok(Stmt::While(parse_while_stmt(inner)?)),
        Rule::for_stmt => Ok(Stmt::For(parse_for_stmt(inner)?)),
        Rule::emit_stmt => Ok(Stmt::Emit(parse_emit_stmt(inner)?)),
        Rule::require_stmt => Ok(Stmt::Require(parse_require_stmt(inner)?)),
        Rule::revert_stmt => Ok(Stmt::Revert(parse_revert_stmt(inner)?)),
        Rule::delete_stmt => Ok(Stmt::Delete(parse_delete_stmt(inner)?)),
        Rule::selfdestruct_stmt => Ok(Stmt::Selfdestruct(parse_selfdestruct_stmt(inner)?)),
        Rule::expr_stmt => Ok(Stmt::Expr(parse_expr_stmt(inner)?)),
        _ => unreachable!("Unexpected statement rule: {:?}", inner.as_rule()),
    }
}

fn parse_var_decl_stmt(pair: Pair) -> Result<VarDeclStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut ty = None;
    let mut storage_location = None;
    let mut name = None;
    let mut initializer = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::type_expr => ty = Some(parse_type_expr(inner)?),
            Rule::storage_location => storage_location = Some(parse_storage_location(inner)),
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::expr => initializer = Some(parse_expr(inner)?),
            _ => {}
        }
    }

    Ok(VarDeclStmt {
        ty: ty.unwrap(),
        storage_location,
        name: name.unwrap(),
        initializer,
        span,
    })
}

fn parse_return_stmt(pair: Pair) -> Result<ReturnStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut value = None;

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::expr {
            value = Some(parse_expr(inner)?);
        }
    }

    Ok(ReturnStmt { value, span })
}

fn parse_if_stmt(pair: Pair) -> Result<IfStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut condition = None;
    let mut then_block = None;
    let mut else_branch = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::expr => condition = Some(parse_expr(inner)?),
            Rule::block => {
                if then_block.is_none() {
                    then_block = Some(parse_block(inner)?);
                } else {
                    else_branch = Some(ElseBranch::Else(parse_block(inner)?));
                }
            }
            Rule::if_stmt => {
                else_branch = Some(ElseBranch::ElseIf(Box::new(parse_if_stmt(inner)?)));
            }
            _ => {}
        }
    }

    Ok(IfStmt {
        condition: condition.unwrap(),
        then_block: then_block.unwrap(),
        else_branch,
        span,
    })
}

fn parse_while_stmt(pair: Pair) -> Result<WhileStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut condition = None;
    let mut body = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::expr => condition = Some(parse_expr(inner)?),
            Rule::block => body = Some(parse_block(inner)?),
            _ => {}
        }
    }

    Ok(WhileStmt {
        condition: condition.unwrap(),
        body: body.unwrap(),
        span,
    })
}

fn parse_for_stmt(pair: Pair) -> Result<ForStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut init = None;
    let mut condition = None;
    let mut update = None;
    let mut body = None;

    let mut expr_count = 0;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::for_init => {
                let init_inner = inner.into_inner().next().unwrap();
                match init_inner.as_rule() {
                    Rule::var_decl_stmt_no_semi => {
                        init = Some(ForInit::VarDecl(parse_var_decl_stmt_no_semi(init_inner)?));
                    }
                    Rule::expr => {
                        init = Some(ForInit::Expr(parse_expr(init_inner)?));
                    }
                    _ => {}
                }
            }
            Rule::expr => {
                if expr_count == 0 {
                    condition = Some(parse_expr(inner)?);
                } else {
                    update = Some(parse_expr(inner)?);
                }
                expr_count += 1;
            }
            Rule::block => body = Some(parse_block(inner)?),
            _ => {}
        }
    }

    Ok(ForStmt {
        init,
        condition,
        update,
        body: body.unwrap(),
        span,
    })
}

fn parse_var_decl_stmt_no_semi(pair: Pair) -> Result<VarDeclStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut ty = None;
    let mut storage_location = None;
    let mut name = None;
    let mut initializer = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::type_expr => ty = Some(parse_type_expr(inner)?),
            Rule::storage_location => storage_location = Some(parse_storage_location(inner)),
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::expr => initializer = Some(parse_expr(inner)?),
            _ => {}
        }
    }

    Ok(VarDeclStmt {
        ty: ty.unwrap(),
        storage_location,
        name: name.unwrap(),
        initializer,
        span,
    })
}

fn parse_emit_stmt(pair: Pair) -> Result<EmitStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut event = None;
    let mut args = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => event = Some(parse_ident(inner)),
            Rule::arg_list => args = parse_arg_list(inner)?,
            _ => {}
        }
    }

    Ok(EmitStmt {
        event: event.unwrap(),
        args,
        span,
    })
}

fn parse_require_stmt(pair: Pair) -> Result<RequireStmt, ParseError> {
    let span = span_from_pair(&pair);
    let mut condition = None;
    let mut message = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::expr => condition = Some(parse_expr(inner)?),
            Rule::string_lit => message = Some(parse_string_content(inner.as_str())),
            _ => {}
        }
    }

    Ok(RequireStmt {
        condition: condition.unwrap(),
        message,
        span,
    })
}

fn parse_revert_stmt(pair: Pair) -> Result<RevertStmt, ParseError> {
    let span = span_from_pair(&pair);

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::revert_with_error => {
                // revert ErrorName(args)
                let mut inner_pairs = inner.into_inner();
                let name = parse_ident(inner_pairs.next().unwrap());
                let args = if let Some(arg_list) = inner_pairs.next() {
                    parse_arg_list(arg_list)?
                } else {
                    Vec::new()
                };
                return Ok(RevertStmt {
                    kind: RevertKind::Error { name, args },
                    span,
                });
            }
            Rule::revert_with_message => {
                // revert("message") or revert()
                let message = inner.into_inner().next().map(|s| parse_string_content(s.as_str()));
                return Ok(RevertStmt {
                    kind: RevertKind::Message(message),
                    span,
                });
            }
            _ => {}
        }
    }

    // Default to empty message
    Ok(RevertStmt {
        kind: RevertKind::Message(None),
        span,
    })
}

fn parse_delete_stmt(pair: Pair) -> Result<DeleteStmt, ParseError> {
    let span = span_from_pair(&pair);
    let target = parse_expr(pair.into_inner().next().unwrap())?;
    Ok(DeleteStmt { target, span })
}

fn parse_selfdestruct_stmt(pair: Pair) -> Result<SelfdestructStmt, ParseError> {
    let span = span_from_pair(&pair);
    let recipient = parse_expr(pair.into_inner().next().unwrap())?;
    Ok(SelfdestructStmt { recipient, span })
}

fn parse_expr_stmt(pair: Pair) -> Result<ExprStmt, ParseError> {
    let span = span_from_pair(&pair);
    let expr = parse_expr(pair.into_inner().next().unwrap())?;
    Ok(ExprStmt { expr, span })
}

// =============================================================================
// Expression parsing
// =============================================================================

fn parse_expr(pair: Pair) -> Result<Expr, ParseError> {
    // expr = { ternary_expr }
    parse_ternary_expr(pair.into_inner().next().unwrap())
}

fn parse_ternary_expr(pair: Pair) -> Result<Expr, ParseError> {
    let span = span_from_pair(&pair);
    let mut inner = pair.into_inner();
    let mut left = parse_assign_expr(inner.next().unwrap())?;

    // Check for ternary: expr ? then : else
    if let Some(then_expr_pair) = inner.next() {
        let then_expr = parse_expr(then_expr_pair)?;
        let else_expr = parse_ternary_expr(inner.next().unwrap())?;
        left = Expr::Ternary(Box::new(TernaryExpr {
            condition: left,
            then_expr,
            else_expr,
            span,
        }));
    }

    Ok(left)
}

fn parse_assign_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_or_expr(inner.next().unwrap())?;

    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "=" => AssignOp::Assign,
            "+=" => AssignOp::AddAssign,
            "-=" => AssignOp::SubAssign,
            "*=" => AssignOp::MulAssign,
            "/=" => AssignOp::DivAssign,
            "%=" => AssignOp::RemAssign,
            "&=" => AssignOp::BitAndAssign,
            "|=" => AssignOp::BitOrAssign,
            "^=" => AssignOp::BitXorAssign,
            _ => unreachable!(),
        };
        let right = parse_or_expr(inner.next().unwrap())?;
        let span = Span::dummy();
        left = Expr::Assign(Box::new(AssignExpr {
            target: left,
            op,
            value: right,
            span,
        }));
    }

    Ok(left)
}

fn parse_or_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_and_expr(inner.next().unwrap())?;

    while let Some(right_pair) = inner.next() {
        let right = parse_and_expr(right_pair)?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op: BinaryOp::Or,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_and_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_bit_or_expr(inner.next().unwrap())?;

    while let Some(right_pair) = inner.next() {
        let right = parse_bit_or_expr(right_pair)?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op: BinaryOp::And,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_bit_or_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_bit_xor_expr(inner.next().unwrap())?;

    while let Some(right_pair) = inner.next() {
        let right = parse_bit_xor_expr(right_pair)?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op: BinaryOp::BitOr,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_bit_xor_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_bit_and_expr(inner.next().unwrap())?;

    while let Some(right_pair) = inner.next() {
        let right = parse_bit_and_expr(right_pair)?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op: BinaryOp::BitXor,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_bit_and_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_eq_expr(inner.next().unwrap())?;

    while let Some(right_pair) = inner.next() {
        let right = parse_eq_expr(right_pair)?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op: BinaryOp::BitAnd,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_eq_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_cmp_expr(inner.next().unwrap())?;

    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "==" => BinaryOp::Eq,
            "!=" => BinaryOp::Ne,
            _ => unreachable!(),
        };
        let right = parse_cmp_expr(inner.next().unwrap())?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_cmp_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_shift_expr(inner.next().unwrap())?;

    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "<" => BinaryOp::Lt,
            "<=" => BinaryOp::Le,
            ">" => BinaryOp::Gt,
            ">=" => BinaryOp::Ge,
            _ => unreachable!(),
        };
        let right = parse_shift_expr(inner.next().unwrap())?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_shift_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_add_expr(inner.next().unwrap())?;

    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "<<" => BinaryOp::Shl,
            ">>" => BinaryOp::Shr,
            _ => unreachable!(),
        };
        let right = parse_add_expr(inner.next().unwrap())?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_add_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_mul_expr(inner.next().unwrap())?;

    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "+" => BinaryOp::Add,
            "-" => BinaryOp::Sub,
            _ => unreachable!(),
        };
        let right = parse_mul_expr(inner.next().unwrap())?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_mul_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut left = parse_exp_expr(inner.next().unwrap())?;

    while let Some(op_pair) = inner.next() {
        let op = match op_pair.as_str() {
            "*" => BinaryOp::Mul,
            "/" => BinaryOp::Div,
            "%" => BinaryOp::Rem,
            _ => unreachable!(),
        };
        let right = parse_exp_expr(inner.next().unwrap())?;
        let span = Span::dummy();
        left = Expr::Binary(Box::new(BinaryExpr {
            left,
            op,
            right,
            span,
        }));
    }

    Ok(left)
}

fn parse_exp_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let left = parse_unary_expr(inner.next().unwrap())?;

    // Exponentiation is right-associative
    if let Some(right_pair) = inner.next() {
        let right = parse_exp_expr(right_pair)?;
        let span = Span::dummy();
        return Ok(Expr::Binary(Box::new(BinaryExpr {
            left,
            op: BinaryOp::Exp,
            right,
            span,
        })));
    }

    Ok(left)
}

fn parse_unary_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut ops = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::unary_op => {
                let op = match inner.as_str() {
                    "!" => UnaryOp::Not,
                    "-" => UnaryOp::Neg,
                    "~" => UnaryOp::BitNot,
                    "++" => UnaryOp::PreInc,
                    "--" => UnaryOp::PreDec,
                    _ => unreachable!(),
                };
                ops.push(op);
            }
            Rule::postfix_expr => {
                let mut expr = parse_postfix_expr(inner)?;
                // Apply unary operators in reverse order
                for op in ops.into_iter().rev() {
                    let span = Span::dummy();
                    expr = Expr::Unary(Box::new(UnaryExpr { op, expr, span }));
                }
                return Ok(expr);
            }
            _ => {}
        }
    }

    unreachable!()
}

fn parse_postfix_expr(pair: Pair) -> Result<Expr, ParseError> {
    let mut inner = pair.into_inner();
    let mut expr = parse_primary_expr(inner.next().unwrap())?;

    for postfix in inner {
        // postfix_op wraps the actual operator
        let op = postfix.into_inner().next().unwrap();
        match op.as_rule() {
            Rule::call_op => {
                let span = Span::dummy();
                let args = if let Some(arg_list) = op.into_inner().next() {
                    parse_arg_list(arg_list)?
                } else {
                    Vec::new()
                };
                expr = Expr::Call(Box::new(CallExpr {
                    callee: expr,
                    args,
                    span,
                }));
            }
            Rule::method_call_op => {
                let span = Span::dummy();
                let mut method = None;
                let mut generic_args = None;
                let mut args = Vec::new();

                for p in op.into_inner() {
                    match p.as_rule() {
                        Rule::ident => method = Some(parse_ident(p)),
                        Rule::generic_args => generic_args = Some(parse_generic_args(p)?),
                        Rule::arg_list => args = parse_arg_list(p)?,
                        _ => {}
                    }
                }

                expr = Expr::MethodCall(Box::new(MethodCallExpr {
                    receiver: expr,
                    method: method.unwrap(),
                    generic_args,
                    args,
                    span,
                }));
            }
            Rule::field_access_op => {
                let span = Span::dummy();
                let field = parse_ident(op.into_inner().next().unwrap());
                expr = Expr::FieldAccess(Box::new(FieldAccessExpr { expr, field, span }));
            }
            Rule::index_op => {
                let span = Span::dummy();
                let index = parse_expr(op.into_inner().next().unwrap())?;
                expr = Expr::Index(Box::new(IndexExpr { expr, index, span }));
            }
            Rule::increment_op => {
                let span = Span::dummy();
                let op = match op.as_str() {
                    "++" => UnaryOp::PostInc,
                    "--" => UnaryOp::PostDec,
                    _ => unreachable!(),
                };
                expr = Expr::Unary(Box::new(UnaryExpr { op, expr, span }));
            }
            _ => {}
        }
    }

    Ok(expr)
}

fn parse_primary_expr(pair: Pair) -> Result<Expr, ParseError> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::literal => Ok(Expr::Literal(parse_literal(inner)?)),
        Rule::new_expr => parse_new_expr(inner),
        Rule::if_expr => parse_if_expr(inner),
        Rule::array_literal => parse_array_literal(inner),
        Rule::tuple_expr => parse_tuple_expr(inner),
        Rule::builtin_object => {
            // msg, block, tx - parsed as identifiers
            let span = span_from_pair(&inner);
            let name = inner.as_str();
            Ok(Expr::Ident(Ident::new(name, span)))
        }
        Rule::ident_expr => {
            let ident = parse_ident(inner.into_inner().next().unwrap());
            Ok(Expr::Ident(ident))
        }
        Rule::paren_expr => {
            let expr = parse_expr(inner.into_inner().next().unwrap())?;
            Ok(Expr::Paren(Box::new(expr)))
        }
        _ => unreachable!("Unexpected primary expr rule: {:?}", inner.as_rule()),
    }
}

fn parse_new_expr(pair: Pair) -> Result<Expr, ParseError> {
    let span = span_from_pair(&pair);
    let mut ty = None;
    let mut args = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::type_path => ty = Some(parse_type_path(inner)?),
            Rule::arg_list => args = parse_arg_list(inner)?,
            _ => {}
        }
    }

    Ok(Expr::New(Box::new(NewExpr {
        ty: ty.unwrap(),
        args,
        span,
    })))
}

fn parse_if_expr(pair: Pair) -> Result<Expr, ParseError> {
    let span = span_from_pair(&pair);
    let mut condition = None;
    let mut then_block = None;
    let mut else_branch = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::expr => condition = Some(parse_expr(inner)?),
            Rule::block => {
                if then_block.is_none() {
                    then_block = Some(parse_block(inner)?);
                } else {
                    else_branch = Some(IfExprElse::Else(parse_block(inner)?));
                }
            }
            Rule::if_expr => {
                let inner_if = parse_if_expr(inner)?;
                if let Expr::If(if_expr) = inner_if {
                    else_branch = Some(IfExprElse::ElseIf(*if_expr));
                }
            }
            _ => {}
        }
    }

    Ok(Expr::If(Box::new(IfExpr {
        condition: condition.unwrap(),
        then_block: then_block.unwrap(),
        else_branch: Box::new(else_branch.unwrap()),
        span,
    })))
}

fn parse_array_literal(pair: Pair) -> Result<Expr, ParseError> {
    let span = span_from_pair(&pair);
    let mut elements = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::expr {
            elements.push(parse_expr(inner)?);
        }
    }

    Ok(Expr::Array(ArrayExpr { elements, span }))
}

fn parse_tuple_expr(pair: Pair) -> Result<Expr, ParseError> {
    let span = span_from_pair(&pair);
    let mut elements = Vec::new();

    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::expr {
            elements.push(parse_expr(inner)?);
        }
    }

    Ok(Expr::Tuple(TupleExpr { elements, span }))
}

fn parse_arg_list(pair: Pair) -> Result<Vec<Arg>, ParseError> {
    let mut args = Vec::new();
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::arg {
            args.push(parse_arg(inner)?);
        }
    }
    Ok(args)
}

fn parse_arg(pair: Pair) -> Result<Arg, ParseError> {
    let span = span_from_pair(&pair);
    let mut name = None;
    let mut value = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::ident => name = Some(parse_ident(inner)),
            Rule::expr => value = Some(parse_expr(inner)?),
            _ => {}
        }
    }

    // If we have both name and value, check if name is actually the value
    if name.is_some() && value.is_none() {
        let ident = name.take().unwrap();
        value = Some(Expr::Ident(ident));
    }

    Ok(Arg {
        name,
        value: value.unwrap(),
        span,
    })
}

// =============================================================================
// Literal parsing
// =============================================================================

fn parse_literal(pair: Pair) -> Result<Literal, ParseError> {
    let inner = pair.into_inner().next().unwrap();
    let span = span_from_pair(&inner);

    match inner.as_rule() {
        Rule::bool_lit => {
            let value = inner.as_str() == "true";
            Ok(Literal::Bool(value, span))
        }
        Rule::hex_string_lit => {
            // hex"..." format
            let s = inner.as_str();
            let hex_content = &s[4..s.len() - 1]; // Remove hex" and "
            Ok(Literal::HexString(SmolStr::new(hex_content), span))
        }
        Rule::string_lit => {
            let s = parse_string_content(inner.as_str());
            Ok(Literal::String(s, span))
        }
        Rule::hex_number_lit => {
            // 0x... format
            let s = inner.as_str();
            Ok(Literal::HexInt(SmolStr::new(s), span))
        }
        Rule::number_lit => {
            let s = inner.as_str();
            // Handle number with possible unit (wei, gwei, ether, etc.)
            let value: u128 = s
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            Ok(Literal::Int(value, span))
        }
        Rule::address_lit => {
            // 0x followed by 40 hex digits
            let s = inner.as_str();
            Ok(Literal::Address(SmolStr::new(s), span))
        }
        _ => unreachable!("Unexpected literal rule: {:?}", inner.as_rule()),
    }
}
