//! SolScript Code Generator
//!
//! This crate generates Anchor-compatible Rust code from the SolScript AST.
//! The generated code can be compiled using `anchor build` to produce Solana BPF bytecode.

mod error;
mod idl_gen;
mod ir;
mod rust_gen;
mod test_gen;
mod ts_gen;

pub use error::CodegenError;
pub use idl_gen::IdlGenerator;
pub use ir::*;
pub use rust_gen::RustGenerator;
pub use test_gen::TestGenerator;
pub use ts_gen::TypeScriptGenerator;

use solscript_ast::Program;

/// Generate Anchor Rust code from a SolScript program
pub fn generate(program: &Program) -> Result<GeneratedProject, CodegenError> {
    // Lower AST to Solana IR
    let ir = lower_to_ir(program)?;

    // Generate Rust code
    let mut generator = RustGenerator::new();
    generator.generate(&ir)
}

/// A generated Anchor project
#[derive(Debug)]
pub struct GeneratedProject {
    /// The main lib.rs content
    pub lib_rs: String,
    /// State account structs (state.rs)
    pub state_rs: String,
    /// Instruction handlers (instructions.rs)
    pub instructions_rs: String,
    /// Error definitions (error.rs)
    pub error_rs: String,
    /// Event definitions (events.rs)
    pub events_rs: String,
    /// Anchor.toml configuration
    pub anchor_toml: String,
    /// Cargo.toml for the program
    pub cargo_toml: String,
    /// TypeScript client (client.ts)
    pub client_ts: String,
    /// TypeScript tests (tests.ts)
    pub tests_ts: String,
    /// Anchor IDL (idl.json)
    pub idl_json: String,
    /// package.json for the project
    pub package_json: String,
    /// README.md for the project
    pub readme: String,
    /// .gitignore file
    pub gitignore: String,
    /// Rust unit tests (rust_tests.rs) - from #[test] functions
    pub rust_tests: String,
    /// Whether there are any SolScript tests
    pub has_tests: bool,
}

impl GeneratedProject {
    /// Write the project to a directory
    pub fn write_to_dir(&self, dir: &std::path::Path) -> std::io::Result<()> {
        use std::fs;

        // Create directory structure
        let programs_dir = dir.join("programs").join("solscript_program");
        let src_dir = programs_dir.join("src");
        let app_dir = dir.join("app");
        let tests_dir = dir.join("tests");
        fs::create_dir_all(&src_dir)?;
        fs::create_dir_all(&app_dir)?;
        fs::create_dir_all(&tests_dir)?;

        // Write Rust program files
        fs::write(src_dir.join("lib.rs"), &self.lib_rs)?;
        fs::write(src_dir.join("state.rs"), &self.state_rs)?;
        fs::write(src_dir.join("instructions.rs"), &self.instructions_rs)?;
        fs::write(src_dir.join("error.rs"), &self.error_rs)?;
        fs::write(src_dir.join("events.rs"), &self.events_rs)?;
        fs::write(programs_dir.join("Cargo.toml"), &self.cargo_toml)?;
        fs::write(dir.join("Anchor.toml"), &self.anchor_toml)?;

        // Write TypeScript client
        fs::write(app_dir.join("client.ts"), &self.client_ts)?;

        // Write tests
        fs::write(tests_dir.join("program.test.ts"), &self.tests_ts)?;

        // Write Rust tests if any
        if self.has_tests && !self.rust_tests.is_empty() {
            fs::write(src_dir.join("tests.rs"), &self.rust_tests)?;
        }

        // Write IDL
        let target_dir = dir.join("target").join("idl");
        fs::create_dir_all(&target_dir)?;
        fs::write(target_dir.join("program.json"), &self.idl_json)?;

        // Write package.json
        fs::write(dir.join("package.json"), &self.package_json)?;

        // Write README and .gitignore
        fs::write(dir.join("README.md"), &self.readme)?;
        fs::write(dir.join(".gitignore"), &self.gitignore)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_and_generate(source: &str) -> Result<GeneratedProject, String> {
        let program =
            solscript_parser::parse(source).map_err(|e| format!("Parse error: {:?}", e))?;
        generate(&program).map_err(|e| format!("Codegen error: {:?}", e))
    }

    #[test]
    fn test_simple_contract() {
        let source = r#"
            contract Counter {
                uint256 public count;

                constructor() {
                    count = 0;
                }

                function increment() public {
                    count += 1;
                }

                function get() public view returns (uint256) {
                    return count;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Check lib.rs contains the program module
        assert!(result.lib_rs.contains("#[program]"));
        assert!(result.lib_rs.contains("pub mod counter"));
        assert!(result.lib_rs.contains("pub fn initialize"));
        assert!(result.lib_rs.contains("pub fn increment"));
        assert!(result.lib_rs.contains("pub fn get"));

        // Check state.rs contains the state struct
        assert!(result.state_rs.contains("#[account]"));
        assert!(result.state_rs.contains("pub struct CounterState"));
        assert!(result.state_rs.contains("pub count: u128"));
    }

    #[test]
    fn test_state_variable_access() {
        let source = r#"
            contract Token {
                uint256 public totalSupply;
                address public owner;

                constructor(uint256 supply) {
                    totalSupply = supply;
                    owner = msg.sender;
                }

                function addSupply(uint256 amount) public {
                    totalSupply += amount;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // State variables should be accessed via ctx.accounts.state
        assert!(result.lib_rs.contains("ctx.accounts.state.total_supply"));
        assert!(result.lib_rs.contains("ctx.accounts.state.owner"));

        // msg.sender should become ctx.accounts.signer.key()
        assert!(result.lib_rs.contains("ctx.accounts.signer.key()"));
    }

    #[test]
    fn test_event_emit() {
        let source = r#"
            event Transfer(address indexed from, address indexed to, uint256 value);

            contract Token {
                mapping(address => uint256) public balances;

                function transfer(address to, uint256 amount) public {
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                    emit Transfer(msg.sender, to, amount);
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Events.rs should contain the event struct
        assert!(result.events_rs.contains("#[event]"));
        assert!(result.events_rs.contains("pub struct Transfer"));
        // Note: #[index] is not supported in Anchor, so we don't generate it
        assert!(result.events_rs.contains("pub from: Pubkey"));
        assert!(result.events_rs.contains("pub to: Pubkey"));
        assert!(result.events_rs.contains("pub value: u128"));

        // lib.rs should emit with qualified event name
        assert!(result.lib_rs.contains("emit!(events::Transfer { from:"));
    }

    #[test]
    fn test_custom_errors() {
        let source = r#"
            error InsufficientBalance(uint256 available, uint256 required);
            error Unauthorized(address caller);

            contract Vault {
                uint256 public balance;

                function withdraw(uint256 amount) public {
                    require(balance >= amount, "Insufficient balance");
                    balance -= amount;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Error.rs should contain custom errors
        assert!(result.error_rs.contains("#[error_code]"));
        assert!(result.error_rs.contains("InsufficientBalance"));
        assert!(result.error_rs.contains("Unauthorized"));
    }

    #[test]
    fn test_modifiers() {
        let source = r#"
            contract Owned {
                address public owner;

                constructor() {
                    owner = msg.sender;
                }

                modifier onlyOwner() {
                    require(msg.sender == owner, "Not owner");
                    _;
                }

                function setOwner(address newOwner) public onlyOwner {
                    owner = newOwner;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // lib.rs should have the inlined modifier check
        assert!(result.lib_rs.contains("require!"));
        assert!(result.lib_rs.contains("pub fn set_owner"));
        // The modifier should be inlined (owner check before the body)
        assert!(result.lib_rs.contains("ctx.accounts.signer.key()"));
    }

    #[test]
    fn test_view_functions() {
        let source = r#"
            contract Storage {
                uint256 public value;

                function getValue() public view returns (uint256) {
                    return value;
                }

                function setValue(uint256 newValue) public {
                    value = newValue;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // View functions should have read-only state account
        // Non-view functions should have mutable state account
        assert!(result.instructions_rs.contains("pub struct GetValue"));
        assert!(result.instructions_rs.contains("pub struct SetValue"));
    }

    #[test]
    fn test_require_statement() {
        let source = r#"
            contract Guard {
                function checkPositive(uint256 value) public pure {
                    require(value > 0, "Value must be positive");
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // require should become require! macro
        assert!(result.lib_rs.contains("require!"));
        assert!(result.lib_rs.contains("CustomError::RequireFailed"));
    }

    #[test]
    fn test_binary_expressions() {
        let source = r#"
            contract Math {
                function test(uint256 a, uint256 b) public pure returns (uint256) {
                    return a + b * 2;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Binary expressions should be properly parenthesized
        assert!(result.lib_rs.contains("+"));
        assert!(result.lib_rs.contains("*"));
    }

    #[test]
    fn test_if_statement() {
        let source = r#"
            contract Conditional {
                uint256 public value;

                function checkAndSet(uint256 newValue) public {
                    if (newValue > 100) {
                        value = 100;
                    } else {
                        value = newValue;
                    }
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // If statement should be properly generated
        assert!(result.lib_rs.contains("if"));
        assert!(result.lib_rs.contains("else"));
    }

    #[test]
    fn test_anchor_project_structure() {
        let source = r#"
            contract SimpleContract {
                uint256 public value;
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Check Anchor.toml
        assert!(result.anchor_toml.contains("[programs.localnet]"));
        assert!(result.anchor_toml.contains("[provider]"));

        // Check Cargo.toml
        assert!(result.cargo_toml.contains("[package]"));
        assert!(result.cargo_toml.contains("anchor-lang"));
        assert!(result.cargo_toml.contains("[lib]"));
    }

    // ========== Integration Tests ==========

    #[test]
    fn test_full_token_contract() {
        let source = r#"
            event Transfer(address indexed from, address indexed to, uint256 amount);
            event Approval(address indexed owner, address indexed spender, uint256 amount);

            error InsufficientBalance(uint256 available, uint256 required);

            contract Token {
                string public name;
                string public symbol;
                uint256 public totalSupply;
                address public owner;
                mapping(address => uint256) public balances;
                mapping(address => mapping(address => uint256)) public allowances;

                constructor(string memory _name, string memory _symbol, uint256 initialSupply) {
                    name = _name;
                    symbol = _symbol;
                    totalSupply = initialSupply;
                    owner = msg.sender;
                    balances[msg.sender] = initialSupply;
                }

                function transfer(address to, uint256 amount) public {
                    require(balances[msg.sender] >= amount, "Insufficient balance");
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                    emit Transfer(msg.sender, to, amount);
                }

                function balanceOf(address account) public view returns (uint256) {
                    return balances[account];
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Verify complete token contract generates properly
        assert!(result.lib_rs.contains("pub mod token"));
        assert!(result.lib_rs.contains("pub fn initialize"));
        assert!(result.lib_rs.contains("pub fn transfer"));
        assert!(result.lib_rs.contains("pub fn balance_of"));

        // State struct
        assert!(result.state_rs.contains("pub name: String"));
        assert!(result.state_rs.contains("pub symbol: String"));
        assert!(result.state_rs.contains("pub total_supply: u128"));
        assert!(result.state_rs.contains("pub owner: Pubkey"));

        // Events
        assert!(result.events_rs.contains("pub struct Transfer"));
        assert!(result.events_rs.contains("pub struct Approval"));

        // Errors
        assert!(result.error_rs.contains("InsufficientBalance"));
    }

    #[test]
    fn test_while_loop_codegen() {
        let source = r#"
            contract LoopTest {
                uint256 public sum;

                function sumUpTo(uint256 n) public {
                    uint256 i = 0;
                    sum = 0;
                    while (i < n) {
                        sum += i;
                        i += 1;
                    }
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        assert!(result.lib_rs.contains("while"));
        assert!(result.lib_rs.contains("ctx.accounts.state.sum"));
    }

    #[test]
    fn test_for_loop_codegen() {
        let source = r#"
            contract ForLoopTest {
                uint256 public result;

                function computeFactorial(uint256 n) public {
                    result = 1;
                    for (uint256 i = 1; i <= n; i += 1) {
                        result *= i;
                    }
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // For loops become while loops in Rust
        assert!(result.lib_rs.contains("while"));
        assert!(result.lib_rs.contains("ctx.accounts.state.result"));
    }

    #[test]
    fn test_nested_if_codegen() {
        let source = r#"
            contract NestedIf {
                uint256 public level;

                function classify(uint256 value) public {
                    if (value < 10) {
                        level = 1;
                    } else {
                        if (value < 100) {
                            level = 2;
                        } else {
                            level = 3;
                        }
                    }
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        assert!(result.lib_rs.contains("if"));
        assert!(result.lib_rs.contains("else"));
        assert!(result.lib_rs.contains("ctx.accounts.state.level"));
    }

    #[test]
    fn test_multiple_functions_codegen() {
        let source = r#"
            contract MultiFn {
                uint256 public a;
                uint256 public b;

                function setA(uint256 value) public {
                    a = value;
                }

                function setB(uint256 value) public {
                    b = value;
                }

                function getSum() public view returns (uint256) {
                    return a + b;
                }

                function swap() public {
                    uint256 temp = a;
                    a = b;
                    b = temp;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        assert!(result.lib_rs.contains("pub fn set_a"));
        assert!(result.lib_rs.contains("pub fn set_b"));
        assert!(result.lib_rs.contains("pub fn get_sum"));
        assert!(result.lib_rs.contains("pub fn swap"));

        // Context structs for each function
        assert!(result.instructions_rs.contains("pub struct SetA"));
        assert!(result.instructions_rs.contains("pub struct SetB"));
        assert!(result.instructions_rs.contains("pub struct GetSum"));
        assert!(result.instructions_rs.contains("pub struct Swap"));
    }

    #[test]
    fn test_comparison_operators() {
        let source = r#"
            contract Comparisons {
                function testComparisons(uint256 a, uint256 b) public pure returns (bool) {
                    if (a == b) { return true; }
                    if (a != b) { return true; }
                    if (a < b) { return true; }
                    if (a <= b) { return true; }
                    if (a > b) { return true; }
                    if (a >= b) { return true; }
                    return false;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        assert!(result.lib_rs.contains("=="));
        assert!(result.lib_rs.contains("!="));
        assert!(result.lib_rs.contains("< "));
        assert!(result.lib_rs.contains("<="));
        assert!(result.lib_rs.contains("> "));
        assert!(result.lib_rs.contains(">="));
    }

    #[test]
    fn test_logical_operators() {
        let source = r#"
            contract Logic {
                function testLogic(bool a, bool b) public pure returns (bool) {
                    return (a && b) || (!a);
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        assert!(result.lib_rs.contains("&&"));
        assert!(result.lib_rs.contains("||"));
        assert!(result.lib_rs.contains("!"));
    }

    #[test]
    fn test_arithmetic_operators() {
        let source = r#"
            contract Arithmetic {
                function compute(uint256 a, uint256 b) public pure returns (uint256) {
                    uint256 sum = a + b;
                    uint256 diff = a - b;
                    uint256 prod = a * b;
                    uint256 quot = a / b;
                    uint256 rem = a % b;
                    return sum + diff + prod + quot + rem;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        assert!(result.lib_rs.contains("+"));
        assert!(result.lib_rs.contains("-"));
        assert!(result.lib_rs.contains("*"));
        assert!(result.lib_rs.contains("/"));
        assert!(result.lib_rs.contains("%"));
    }

    #[test]
    fn test_compound_assignment() {
        let source = r#"
            contract CompoundAssign {
                uint256 public value;

                function testCompound(uint256 x) public {
                    value = 100;
                    value += x;
                    value -= 10;
                    value *= 2;
                    value /= 5;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Compound assignments should expand to binary operations
        assert!(result.lib_rs.contains("ctx.accounts.state.value"));
        assert!(result.lib_rs.contains("+"));
        assert!(result.lib_rs.contains("-"));
        assert!(result.lib_rs.contains("*"));
        assert!(result.lib_rs.contains("/"));
    }

    #[test]
    fn test_ternary_expression_codegen() {
        let source = r#"
            contract Ternary {
                function max(uint256 a, uint256 b) public pure returns (uint256) {
                    return a > b ? a : b;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Ternary becomes if-else expression in Rust
        assert!(result.lib_rs.contains("if"));
        assert!(result.lib_rs.contains("else"));
    }

    #[test]
    fn test_multiple_events_and_errors() {
        let source = r#"
            event Deposit(address indexed user, uint256 amount);
            event Withdraw(address indexed user, uint256 amount);
            event OwnerChanged(address indexed oldOwner, address indexed newOwner);

            error NotOwner(address caller);
            error InsufficientFunds(uint256 requested, uint256 available);
            error ZeroAmount(string reason);

            contract Vault {
                address public owner;
                mapping(address => uint256) public deposits;

                constructor() {
                    owner = msg.sender;
                }

                function deposit(uint256 amount) public {
                    require(amount > 0, "Amount must be positive");
                    deposits[msg.sender] += amount;
                    emit Deposit(msg.sender, amount);
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // All events present
        assert!(result.events_rs.contains("pub struct Deposit"));
        assert!(result.events_rs.contains("pub struct Withdraw"));
        assert!(result.events_rs.contains("pub struct OwnerChanged"));

        // All errors present
        assert!(result.error_rs.contains("NotOwner"));
        assert!(result.error_rs.contains("InsufficientFunds"));
        assert!(result.error_rs.contains("ZeroAmount"));
    }

    #[test]
    fn test_local_variables() {
        let source = r#"
            contract LocalVars {
                uint256 public result;

                function compute(uint256 x, uint256 y) public {
                    uint256 temp1 = x * 2;
                    uint256 temp2 = y * 3;
                    uint256 sum = temp1 + temp2;
                    result = sum;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        assert!(result.lib_rs.contains("let temp1"));
        assert!(result.lib_rs.contains("let temp2"));
        assert!(result.lib_rs.contains("let sum"));
    }

    #[test]
    fn test_function_with_multiple_params() {
        let source = r#"
            contract MultiParams {
                function process(uint256 a, uint256 b, uint256 c, address target) public pure returns (uint256) {
                    return a + b + c;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        assert!(result.lib_rs.contains("a: u128"));
        assert!(result.lib_rs.contains("b: u128"));
        assert!(result.lib_rs.contains("c: u128"));
        assert!(result.lib_rs.contains("target: Pubkey"));
    }

    #[test]
    fn test_mapping_pda_codegen() {
        let source = r#"
            contract Balances {
                mapping(address => uint256) public balances;

                constructor() {
                    balances[msg.sender] = 1000;
                }

                function deposit(uint256 amount) public {
                    balances[msg.sender] += amount;
                }

                function transfer(address to, uint256 amount) public {
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                }

                function balanceOf(address account) public view returns (uint256) {
                    return balances[account];
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Mapping entry struct should be generated
        assert!(result.state_rs.contains("pub struct BalancesEntry"));
        assert!(result.state_rs.contains("pub key: Pubkey"));
        assert!(result.state_rs.contains("pub value: u128"));

        // Mapping should NOT be in main state struct
        assert!(!result.state_rs.contains("pub balances:"));

        // PDA accounts should be in instruction contexts
        assert!(result.instructions_rs.contains("balances_entry_0"));

        // Seeds should use proper account references
        assert!(result.instructions_rs.contains(r#"seeds = [b"balances""#));
        assert!(result.instructions_rs.contains("signer.key().as_ref()"));
        assert!(result.instructions_rs.contains("to.as_ref()"));
        assert!(result.instructions_rs.contains("account.as_ref()"));

        // init_if_needed should be used for PDA accounts
        assert!(result.instructions_rs.contains("init_if_needed"));

        // Mapping accesses should use .value field
        assert!(result.lib_rs.contains(".value"));
    }

    #[test]
    fn test_multiple_mappings_codegen() {
        let source = r#"
            contract MultiMap {
                mapping(address => uint256) public balances;
                mapping(address => bool) public approved;

                function setBalance(address user, uint256 amount) public {
                    balances[user] = amount;
                }

                function setApproved(address user, bool status) public {
                    approved[user] = status;
                }
            }
        "#;

        let result = parse_and_generate(source).unwrap();

        // Both mapping entry structs should be generated
        assert!(result.state_rs.contains("pub struct BalancesEntry"));
        assert!(result.state_rs.contains("pub struct ApprovedEntry"));

        // Both should have correct value types
        assert!(result.state_rs.contains("pub value: u128")); // BalancesEntry
        assert!(result.state_rs.contains("pub value: bool")); // ApprovedEntry
    }

    #[test]
    fn test_inheritance() {
        let source = r#"
            contract Ownable {
                address public owner;

                modifier onlyOwner() {
                    require(msg.sender == owner, "Not owner");
                    _;
                }

                function transferOwnership(address newOwner) public onlyOwner {
                    owner = newOwner;
                }
            }

            contract Token is Ownable {
                uint256 public totalSupply;

                constructor(uint256 supply) {
                    owner = msg.sender;
                    totalSupply = supply;
                }

                function mint(uint256 amount) public onlyOwner {
                    totalSupply += amount;
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // The Token contract should have both inherited and own state variables
        assert!(result.state_rs.contains("pub owner: Pubkey")); // inherited
        assert!(result.state_rs.contains("pub total_supply: u128")); // own

        // Should have inherited function
        assert!(result.lib_rs.contains("pub fn transfer_ownership"));

        // Should have own function
        assert!(result.lib_rs.contains("pub fn mint"));

        // The modifier should be inlined in both functions
        assert!(result.lib_rs.contains("require!"));
    }

    #[test]
    fn test_nested_mapping() {
        let source = r#"
            contract ERC20 {
                mapping(address => mapping(address => uint256)) public allowances;

                function approve(address spender, uint256 amount) public {
                    allowances[msg.sender][spender] = amount;
                }

                function allowance(address owner, address spender) public view returns (uint256) {
                    return allowances[owner][spender];
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // The state should contain the allowances field
        assert!(result.state_rs.contains("allowances"));

        // The approve function should be generated
        assert!(result.lib_rs.contains("pub fn approve"));

        // The allowance function should be generated
        assert!(result.lib_rs.contains("pub fn allowance"));

        // The PDA seeds should reference multiple keys (owner and spender)
        // For nested mappings, both keys contribute to the PDA seeds in instructions.rs
        assert!(
            result.instructions_rs.contains("seeds = [b\"allowances\""),
            "Should have PDA seeds"
        );

        // Check that nested mapping seeds contain both keys
        // In approve: msg.sender and spender
        assert!(
            result
                .instructions_rs
                .contains("signer.key().as_ref(), spender.as_ref()"),
            "Approve should use signer and spender as seeds"
        );

        // In allowance: owner and spender
        assert!(
            result
                .instructions_rs
                .contains("owner.as_ref(), spender.as_ref()"),
            "Allowance should use owner and spender as seeds"
        );
    }

    /// Integration test that verifies generated code compiles with Anchor.
    /// Run with: cargo test --package solscript-codegen anchor_build_integration -- --ignored
    #[test]
    #[ignore] // Requires Anchor installed, slower test
    fn anchor_build_integration() {
        use std::fs;
        use std::process::Command;

        let source = r#"
            contract Token {
                uint256 public totalSupply;
                mapping(address => uint256) public balances;
                address public owner;

                event Transfer(address from, address to, uint256 amount);
                error InsufficientBalance(uint256 available, uint256 required);

                modifier onlyOwner() {
                    require(msg.sender == owner, "Not owner");
                    _;
                }

                constructor(uint256 supply) {
                    owner = msg.sender;
                    totalSupply = supply;
                    balances[msg.sender] = supply;
                }

                function transfer(address to, uint256 amount) public {
                    require(balances[msg.sender] >= amount, "Insufficient balance");
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                }

                function balanceOf(address account) public view returns (uint256) {
                    return balances[account];
                }

                function mint(uint256 amount) public onlyOwner {
                    totalSupply += amount;
                    balances[owner] += amount;
                }
            }
        "#;

        // Parse and generate
        let program = solscript_parser::parse(source).expect("Parse failed");
        let mut checker = solscript_typeck::TypeChecker::new(source.to_string());
        let _ = checker.check_program(&program); // Type check (errors are non-fatal for codegen)
        let project = generate(&program).expect("Codegen failed");

        // Create temp directory
        let temp_dir = std::env::temp_dir().join(format!("solscript_test_{}", std::process::id()));
        fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

        // Write project
        project
            .write_to_dir(&temp_dir)
            .expect("Failed to write project");

        // Run cargo check on the generated program
        let program_dir = temp_dir.join("programs").join("solscript_program");
        let output = Command::new("cargo")
            .args(["check", "--lib"])
            .current_dir(&program_dir)
            .output()
            .expect("Failed to run cargo check");

        // Clean up temp directory
        let _ = fs::remove_dir_all(&temp_dir);

        // Check result
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            panic!(
                "Anchor build failed!\n\nSTDOUT:\n{}\n\nSTDERR:\n{}",
                stdout, stderr
            );
        }
    }

    #[test]
    fn test_struct_codegen() {
        let source = r#"
            struct Point {
                uint256 x;
                uint256 y;
            }

            contract Geometry {
                Point public origin;

                function setOrigin(uint256 newX, uint256 newY) public {
                    origin.x = newX;
                    origin.y = newY;
                }

                function getX() public view returns (uint256) {
                    return origin.x;
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // Check struct is generated with correct derives
        assert!(
            result
                .state_rs
                .contains("#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]"),
            "Struct should have Anchor derives"
        );
        assert!(
            result.state_rs.contains("pub struct Point"),
            "Struct Point should be generated"
        );
        assert!(
            result.state_rs.contains("pub x: u128"),
            "Struct should have x field"
        );
        assert!(
            result.state_rs.contains("pub y: u128"),
            "Struct should have y field"
        );

        // Check state uses the struct
        assert!(
            result.state_rs.contains("pub origin: Point"),
            "State should use Point struct"
        );
    }

    #[test]
    fn test_enum_codegen() {
        let source = r#"
            enum Status {
                Pending,
                Active,
                Completed
            }

            contract Task {
                Status public status;

                function getStatus() public view returns (uint8) {
                    return 0;
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // Check enum is generated with correct derives
        assert!(
            result.state_rs.contains("#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]"),
            "Enum should have Anchor derives"
        );
        assert!(
            result.state_rs.contains("pub enum Status"),
            "Enum Status should be generated"
        );
        assert!(
            result.state_rs.contains("#[default]"),
            "First variant should have #[default]"
        );
        assert!(
            result.state_rs.contains("Pending"),
            "Enum should have Pending variant"
        );
        assert!(
            result.state_rs.contains("Active"),
            "Enum should have Active variant"
        );
        assert!(
            result.state_rs.contains("Completed"),
            "Enum should have Completed variant"
        );

        // Check state uses the enum
        assert!(
            result.state_rs.contains("pub status: Status"),
            "State should use Status enum"
        );
    }

    #[test]
    fn test_dynamic_array_codegen() {
        let source = r#"
            contract Storage {
                uint256[] public numbers;

                function push(uint256 value) public {
                    numbers.push(value);
                }

                function getLength() public view returns (uint256) {
                    return numbers.length;
                }

                function get(uint256 index) public view returns (uint256) {
                    return numbers[index];
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // Check dynamic array is generated as Vec
        assert!(
            result.state_rs.contains("pub numbers: Vec<u128>"),
            "Dynamic array should be Vec<u128>"
        );

        // Check push method works
        assert!(
            result.lib_rs.contains(".push(value)"),
            "Push should be generated"
        );

        // Check length is converted to len() with cast
        assert!(
            result.lib_rs.contains(".len() as u128"),
            "Length should be converted to len() with u128 cast"
        );

        // Check array indexing uses usize cast
        assert!(
            result.lib_rs.contains("[index as usize]"),
            "Index should be cast to usize"
        );
    }

    #[test]
    fn test_payable_function_codegen() {
        let source = r#"
            contract Donation {
                uint256 public totalDonations;

                function donate() public payable {
                    totalDonations += 1;
                }

                function getBalance() public view returns (uint256) {
                    return totalDonations;
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // Payable function should have system_program in its context
        assert!(
            result.instructions_rs.contains("pub struct Donate"),
            "Donate context should be generated"
        );

        // Donate (payable) should have system_program
        assert!(
            result
                .instructions_rs
                .contains("pub system_program: Program<'info, System>"),
            "Payable function should have system_program"
        );

        // View function should NOT have system_program (no writes)
        // Check that GetBalance doesn't have system_program
        let get_balance_section = result
            .instructions_rs
            .split("pub struct GetBalance")
            .nth(1)
            .and_then(|s| s.split("pub struct").next())
            .unwrap_or("");
        assert!(
            !get_balance_section.contains("system_program"),
            "View-only function should not have system_program"
        );
    }

    #[test]
    fn test_spl_token_operations() {
        let source = r#"
            contract TokenVault {
                uint256 public totalTransfers;

                function transferTokens(address from, address to, address auth, uint256 amt) public {
                    token.transfer(from, to, auth, amt);
                    totalTransfers += 1;
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // Token program should be included in context
        assert!(
            result
                .instructions_rs
                .contains("pub token_program: Program<'info, Token>"),
            "Token operations should include token_program account"
        );

        // anchor_spl import should be present
        assert!(
            result
                .instructions_rs
                .contains("use anchor_spl::token::Token"),
            "Should import Token from anchor_spl"
        );

        // CPI call should be generated
        assert!(
            result.lib_rs.contains("anchor_spl::token::Transfer"),
            "Should generate Transfer CPI struct"
        );
        assert!(
            result.lib_rs.contains("anchor_spl::token::transfer"),
            "Should generate transfer CPI call"
        );

        // Cargo.toml should include anchor-spl
        assert!(
            result.cargo_toml.contains("anchor-spl"),
            "Should include anchor-spl dependency"
        );
    }

    #[test]
    fn test_multiple_signers() {
        let source = r#"
            contract MultiSig {
                address public admin;

                function transferWithApproval(signer approver, uint256 amount) public {
                    require(approver == admin, "Not admin");
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // Additional signer should be in context
        assert!(
            result
                .instructions_rs
                .contains("pub approver: Signer<'info>"),
            "Signer param should be in context as Signer<'info>"
        );

        // Signer param should NOT be in function params
        assert!(
            !result.lib_rs.contains("approver: Pubkey"),
            "Signer param should not be in function params"
        );

        // Access to signer should use ctx.accounts
        assert!(
            result.lib_rs.contains("ctx.accounts.approver.key()"),
            "Signer variable should access ctx.accounts"
        );
    }

    #[test]
    fn test_interface_cpi_codegen() {
        let source = r#"
            interface IERC20 {
                function transfer(address to, uint256 amount) external returns (bool);
                function balanceOf(address account) external view returns (uint256);
            }

            contract TokenUser {
                address public tokenProgram;

                function doTransfer(address to, uint256 amount) public {
                    IERC20(tokenProgram).transfer(to, amount);
                }
            }
        "#;

        let result = parse_and_generate(source);
        assert!(result.is_ok(), "Failed to generate: {:?}", result.err());
        let result = result.unwrap();

        // CPI call should be generated with invoke
        assert!(
            result.lib_rs.contains("// CPI to IERC20.transfer"),
            "CPI comment should be generated"
        );

        assert!(
            result
                .lib_rs
                .contains("anchor_lang::solana_program::program::invoke"),
            "CPI invoke should be generated"
        );
    }
}
