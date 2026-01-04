//! SolScript Parser
//!
//! This crate parses SolScript source code into an AST using pest.

#![allow(unused_assignments)] // Suppress false positives from derive macros

mod error;
mod parser;

pub use error::*;
pub use parser::*;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../../../grammar/solscript.pest"]
pub struct SolScriptParser;

/// Parse SolScript source code into an AST
pub fn parse(source: &str) -> Result<solscript_ast::Program, ParseError> {
    parser::parse_program(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_contract() {
        let source = r#"
            contract Empty {
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_contract_with_state() {
        let source = r#"
            contract Counter {
                uint256 public count;
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_contract_with_function() {
        let source = r#"
            contract Counter {
                uint256 public count;

                function increment() public {
                    count += 1;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_struct() {
        let source = r#"
            struct Point {
                uint256 x;
                uint256 y;
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_enum() {
        let source = r#"
            enum Status {
                Pending,
                Active,
                Completed
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_import() {
        let source = r#"
            import { Token, PDA } from "@solana/token";
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_event_and_error() {
        let source = r#"
            event Transfer(address indexed from, address indexed to, uint256 amount);
            error InsufficientBalance(uint256 available, uint256 required);
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_mapping_types() {
        let source = r#"
            contract Storage {
                mapping(address => uint256) public balances;
                mapping(address => mapping(address => uint256)) public allowances;
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_function_with_params() {
        let source = r#"
            contract Math {
                function add(uint256 a, uint256 b) public pure returns (uint256) {
                    return a + b;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_if_statement() {
        let source = r#"
            contract Logic {
                function check(uint256 x) public pure returns (bool) {
                    if (x > 10) {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_var_declaration() {
        let source = r#"
            contract Vars {
                function compute() public pure returns (uint256) {
                    uint256 x = 10;
                    uint256 y = 20;
                    uint256 result = x + y;
                    return result;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_method_chaining() {
        let source = r#"
            contract Chain {
                mapping(bytes32 => uint256) public data;

                function process(bytes32 key) public view returns (uint256) {
                    uint256 result = data[key];
                    return result;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_interface_definition() {
        let source = r#"
            interface IERC20 {
                function transfer(address to, uint256 amount) external returns (bool);
                function balanceOf(address account) external view returns (uint256);
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_constructor() {
        let source = r#"
            contract Token {
                address public owner;
                uint256 public totalSupply;

                constructor(uint256 initialSupply) {
                    owner = msg.sender;
                    totalSupply = initialSupply;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_modifier() {
        let source = r#"
            contract Owned {
                address public owner;

                modifier onlyOwner() {
                    require(msg.sender == owner, "Not owner");
                    _;
                }

                function withdraw() public onlyOwner {
                    return;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_for_loop() {
        let source = r#"
            contract Loops {
                function sum(uint256 n) public pure returns (uint256) {
                    uint256 total = 0;
                    for (uint256 i = 0; i < n; i++) {
                        total += i;
                    }
                    return total;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_while_loop() {
        let source = r#"
            contract Loops {
                function countdown(uint256 n) public pure returns (uint256) {
                    while (n > 0) {
                        n -= 1;
                    }
                    return n;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_emit_statement() {
        let source = r#"
            event Transfer(address from, address to, uint256 amount);

            contract Events {
                function doTransfer(address to, uint256 amount) public {
                    emit Transfer(msg.sender, to, amount);
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_require_and_revert() {
        let source = r#"
            contract Validation {
                function transfer(address to, uint256 amount) public {
                    require(amount > 0, "Amount must be positive");
                    require(to != msg.sender, "Cannot transfer to self");
                    if (amount > 1000) {
                        revert("Amount too large");
                    }
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_array_types() {
        let source = r#"
            contract Arrays {
                uint256[] public dynamicArray;
                uint256[10] public fixedArray;

                function getElement(uint256 i) public view returns (uint256) {
                    return dynamicArray[i];
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_ternary_expression() {
        let source = r#"
            contract Ternary {
                function max(uint256 a, uint256 b) public pure returns (uint256) {
                    return a > b ? a : b;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_new_expression() {
        let source = r#"
            contract Factory {
                function createToken() public returns (Token) {
                    Token token = new Token(1000);
                    return token;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_inheritance() {
        let source = r#"
            interface IERC20 {
                function transfer(address to, uint256 amount) external returns (bool);
            }

            contract Token is IERC20 {
                mapping(address => uint256) public balances;

                function transfer(address to, uint256 amount) external returns (bool) {
                    balances[msg.sender] -= amount;
                    balances[to] += amount;
                    return true;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_multiple_modifiers() {
        let source = r#"
            contract MultiMod {
                function restricted() public view onlyOwner whenNotPaused returns (uint256) {
                    return 42;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_address_literal() {
        let source = r#"
            contract Addresses {
                address public ZERO = 0x0000000000000000000000000000000000000000;

                function isZero(address addr) public pure returns (bool) {
                    return addr == 0x0000000000000000000000000000000000000000;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_bitwise_operations() {
        let source = r#"
            contract Bitwise {
                function operations(uint256 a, uint256 b) public pure returns (uint256) {
                    uint256 and_result = a & b;
                    uint256 or_result = a | b;
                    uint256 xor_result = a ^ b;
                    uint256 not_result = ~a;
                    uint256 shift_left = a << 2;
                    uint256 shift_right = a >> 2;
                    return and_result + or_result + xor_result;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_visibility_modifiers() {
        let source = r#"
            contract Visibility {
                uint256 public publicVar;
                uint256 private privateVar;
                uint256 internal internalVar;

                function publicFunc() public {}
                function privateFunc() private {}
                function internalFunc() internal {}
                function externalFunc() external {}
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());
    }

    #[test]
    fn test_parse_events_errors_in_contract() {
        let source = r#"
            contract Token {
                uint256 public totalSupply;

                event Transfer(address from, address to, uint256 amount);
                event Approval(address owner, address spender, uint256 amount);
                error InsufficientBalance(uint256 available, uint256 required);
                error Unauthorized(address caller);

                function transfer(address to, uint256 amount) public {
                    totalSupply -= amount;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let program = result.unwrap();
        let contract = match &program.items[0] {
            solscript_ast::Item::Contract(c) => c,
            _ => panic!("Expected contract"),
        };

        // Verify we have 2 events and 2 errors in the contract
        let events: Vec<_> = contract.members.iter().filter(|m| matches!(m, solscript_ast::ContractMember::Event(_))).collect();
        let errors: Vec<_> = contract.members.iter().filter(|m| matches!(m, solscript_ast::ContractMember::Error(_))).collect();
        assert_eq!(events.len(), 2, "Expected 2 events");
        assert_eq!(errors.len(), 2, "Expected 2 errors");
    }

    #[test]
    fn test_parse_abstract_contract() {
        let source = r#"
            abstract contract Base {
                uint256 public value;

                // Abstract function (no body)
                function getValue() public view returns (uint256);

                // Implemented function
                function setValue(uint256 newValue) public {
                    value = newValue;
                }
            }

            contract Derived is Base {
                constructor() {
                    value = 0;
                }

                function getValue() public view returns (uint256) {
                    return value;
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let program = result.unwrap();

        // First contract should be abstract
        let base = match &program.items[0] {
            solscript_ast::Item::Contract(c) => c,
            _ => panic!("Expected contract"),
        };
        assert!(base.is_abstract, "Base should be abstract");
        assert_eq!(base.name.name.as_str(), "Base");

        // Check that abstract function has no body
        let get_value_fn = base.members.iter().find_map(|m| {
            if let solscript_ast::ContractMember::Function(f) = m {
                if f.name.name.as_str() == "getValue" {
                    return Some(f);
                }
            }
            None
        }).expect("Should have getValue function");
        assert!(get_value_fn.body.is_none(), "Abstract function should have no body");

        // Check that implemented function has a body
        let set_value_fn = base.members.iter().find_map(|m| {
            if let solscript_ast::ContractMember::Function(f) = m {
                if f.name.name.as_str() == "setValue" {
                    return Some(f);
                }
            }
            None
        }).expect("Should have setValue function");
        assert!(set_value_fn.body.is_some(), "Implemented function should have body");

        // Second contract should not be abstract
        let derived = match &program.items[1] {
            solscript_ast::Item::Contract(c) => c,
            _ => panic!("Expected contract"),
        };
        assert!(!derived.is_abstract, "Derived should not be abstract");
        assert_eq!(derived.name.name.as_str(), "Derived");
    }

    #[test]
    fn test_parse_selfdestruct() {
        let source = r#"
            contract Closeable {
                address public owner;

                function destroy() public {
                    selfdestruct(msg.sender);
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let program = result.unwrap();
        let contract = match &program.items[0] {
            solscript_ast::Item::Contract(c) => c,
            _ => panic!("Expected contract"),
        };

        // Find the destroy function
        let destroy_fn = contract.members.iter().find_map(|m| {
            if let solscript_ast::ContractMember::Function(f) = m {
                if f.name.name.as_str() == "destroy" {
                    return Some(f);
                }
            }
            None
        }).expect("Should have destroy function");

        // Check that body contains a selfdestruct statement
        let body = destroy_fn.body.as_ref().expect("destroy should have a body");
        assert_eq!(body.stmts.len(), 1);
        assert!(matches!(body.stmts[0], solscript_ast::Stmt::Selfdestruct(_)));
    }

    #[test]
    fn test_parse_interface_cpi() {
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

                function checkBalance(address account) public view returns (uint256) {
                    return IERC20(tokenProgram).balanceOf(account);
                }
            }
        "#;
        let result = parse(source);
        assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

        let program = result.unwrap();

        // First item should be the interface
        let interface = match &program.items[0] {
            solscript_ast::Item::Interface(i) => i,
            _ => panic!("Expected interface"),
        };
        assert_eq!(interface.name.name.as_str(), "IERC20");
        assert_eq!(interface.members.len(), 2);

        // Second item should be the contract
        let contract = match &program.items[1] {
            solscript_ast::Item::Contract(c) => c,
            _ => panic!("Expected contract"),
        };
        assert_eq!(contract.name.name.as_str(), "TokenUser");

        // Find the doTransfer function and verify it has a method call expression
        let do_transfer_fn = contract.members.iter().find_map(|m| {
            if let solscript_ast::ContractMember::Function(f) = m {
                if f.name.name.as_str() == "doTransfer" {
                    return Some(f);
                }
            }
            None
        }).expect("Should have doTransfer function");

        let body = do_transfer_fn.body.as_ref().expect("doTransfer should have a body");
        assert_eq!(body.stmts.len(), 1);

        // The statement should be an expression statement with a method call
        if let solscript_ast::Stmt::Expr(expr_stmt) = &body.stmts[0] {
            if let solscript_ast::Expr::MethodCall(mc) = &expr_stmt.expr {
                assert_eq!(mc.method.name.as_str(), "transfer");
                // The receiver should be a Call expression: IERC20(tokenProgram)
                if let solscript_ast::Expr::Call(call) = &mc.receiver {
                    if let solscript_ast::Expr::Ident(ident) = &call.callee {
                        assert_eq!(ident.name.as_str(), "IERC20");
                    } else {
                        panic!("Expected identifier callee");
                    }
                } else {
                    panic!("Expected call expression as receiver");
                }
            } else {
                panic!("Expected method call expression");
            }
        } else {
            panic!("Expected expression statement");
        }
    }
}
