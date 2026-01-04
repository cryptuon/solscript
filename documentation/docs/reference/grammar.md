# Grammar Reference

Formal grammar specification for SolScript.

## Notation

- `|` - Alternative
- `*` - Zero or more
- `+` - One or more
- `?` - Optional
- `()` - Grouping
- `"text"` - Literal text
- `UPPERCASE` - Terminal/token
- `lowercase` - Non-terminal/rule

---

## Source File

```ebnf
source_file = item*

item = contract_def
     | interface_def
     | struct_def
     | enum_def
     | error_def
     | function_def
     | constant_def
```

---

## Contracts

```ebnf
contract_def = "abstract"? "contract" IDENT inheritance? "{" contract_member* "}"

inheritance = "is" IDENT ("," IDENT)*

contract_member = state_var
                | constructor_def
                | function_def
                | modifier_def
                | event_def
                | error_def
                | struct_def
                | enum_def
```

---

## Interfaces

```ebnf
interface_def = "interface" IDENT inheritance? "{" interface_member* "}"

interface_member = function_sig ";"
                 | event_def
                 | error_def
                 | struct_def
                 | enum_def
```

---

## State Variables

```ebnf
state_var = type_expr visibility? "constant"? "immutable"? IDENT ("=" expr)? ";"

visibility = "public" | "private" | "internal"
```

---

## Functions

```ebnf
function_def = "function" IDENT "(" param_list? ")"
               visibility? state_mutability* modifier_call*
               ("returns" "(" return_params? ")")?
               block

function_sig = "function" IDENT "(" param_list? ")"
               visibility? state_mutability*
               ("returns" "(" return_params? ")")?

state_mutability = "view" | "pure" | "payable"

modifier_call = IDENT ("(" arg_list? ")")?

param_list = param ("," param)*

param = type_expr storage_location? IDENT

return_params = return_param ("," return_param)*

return_param = type_expr storage_location? IDENT?

storage_location = "memory" | "storage" | "calldata"
```

---

## Constructor

```ebnf
constructor_def = "constructor" "(" param_list? ")" modifier_call* block
```

---

## Modifiers

```ebnf
modifier_def = "modifier" IDENT "(" param_list? ")" modifier_block

modifier_block = "{" stmt* "_" ";" stmt* "}"
```

---

## Events

```ebnf
event_def = "event" IDENT "(" event_param_list? ")" ";"

event_param_list = event_param ("," event_param)*

event_param = type_expr "indexed"? IDENT?
```

---

## Errors

```ebnf
error_def = "error" IDENT "(" param_list? ")" ";"
```

---

## Structs

```ebnf
struct_def = "struct" IDENT "{" struct_field* "}"

struct_field = type_expr IDENT ";"
```

---

## Enums

```ebnf
enum_def = "enum" IDENT "{" enum_variant ("," enum_variant)* ","? "}"

enum_variant = IDENT
```

---

## Types

```ebnf
type_expr = elementary_type
          | user_type
          | mapping_type
          | array_type
          | function_type

elementary_type = "uint" ("8" | "16" | "32" | "64" | "128" | "256")?
                | "int" ("8" | "16" | "32" | "64" | "128" | "256")?
                | "bool"
                | "address"
                | "string"
                | "bytes" ("1" | "2" | ... | "32")?

user_type = IDENT ("." IDENT)*

mapping_type = "mapping" "(" type_expr "=>" type_expr ")"

array_type = type_expr "[" expr? "]"

function_type = "function" "(" type_list? ")"
                visibility? state_mutability*
                ("returns" "(" type_list? ")")?

type_list = type_expr ("," type_expr)*
```

---

## Statements

```ebnf
stmt = var_decl_stmt
     | expr_stmt
     | if_stmt
     | for_stmt
     | while_stmt
     | do_while_stmt
     | return_stmt
     | emit_stmt
     | require_stmt
     | revert_stmt
     | break_stmt
     | continue_stmt
     | block

block = "{" stmt* "}"

var_decl_stmt = type_expr storage_location? IDENT ("=" expr)? ";"
              | "(" var_decl_tuple ")" "=" expr ";"

var_decl_tuple = (type_expr? IDENT?) ("," (type_expr? IDENT?))*

expr_stmt = expr ";"

if_stmt = "if" "(" expr ")" stmt ("else" stmt)?

for_stmt = "for" "(" (var_decl_stmt | expr_stmt | ";") expr? ";" expr? ")" stmt

while_stmt = "while" "(" expr ")" stmt

do_while_stmt = "do" stmt "while" "(" expr ")" ";"

return_stmt = "return" expr? ";"

emit_stmt = "emit" IDENT "(" arg_list? ")" ";"

require_stmt = "require" "(" expr ("," expr)? ")" ";"

revert_stmt = "revert" (IDENT "(" arg_list? ")")? ";"
            | "revert" "(" expr? ")" ";"

break_stmt = "break" ";"

continue_stmt = "continue" ";"
```

---

## Expressions

```ebnf
expr = assignment_expr

assignment_expr = conditional_expr (assignment_op assignment_expr)?

assignment_op = "=" | "+=" | "-=" | "*=" | "/=" | "%="
              | "&=" | "|=" | "^=" | "<<=" | ">>="

conditional_expr = or_expr ("?" expr ":" conditional_expr)?

or_expr = and_expr ("||" and_expr)*

and_expr = equality_expr ("&&" equality_expr)*

equality_expr = comparison_expr (("==" | "!=") comparison_expr)*

comparison_expr = bitwise_or_expr (("<" | "<=" | ">" | ">=") bitwise_or_expr)*

bitwise_or_expr = bitwise_xor_expr ("|" bitwise_xor_expr)*

bitwise_xor_expr = bitwise_and_expr ("^" bitwise_and_expr)*

bitwise_and_expr = shift_expr ("&" shift_expr)*

shift_expr = add_expr (("<<" | ">>") add_expr)*

add_expr = mul_expr (("+" | "-") mul_expr)*

mul_expr = exp_expr (("*" | "/" | "%") exp_expr)*

exp_expr = unary_expr ("**" exp_expr)?

unary_expr = ("!" | "-" | "~" | "++" | "--") unary_expr
           | postfix_expr

postfix_expr = primary_expr postfix_op*

postfix_op = "++"
           | "--"
           | "[" expr "]"
           | "." IDENT
           | "(" arg_list? ")"

primary_expr = literal
             | IDENT
             | "(" expr ")"
             | new_expr
             | type_expr "(" expr ")"

new_expr = "new" type_expr ("(" arg_list? ")")?
         | "new" type_expr "[" expr "]"

arg_list = arg ("," arg)*

arg = (IDENT ":")? expr

literal = number_lit
        | string_lit
        | bool_lit
        | address_lit

number_lit = DECIMAL_NUMBER
           | HEX_NUMBER

string_lit = "\"" STRING_CONTENT "\""

bool_lit = "true" | "false"

address_lit = "0x" HEX_DIGIT{40}
```

---

## Tokens

```ebnf
IDENT = LETTER (LETTER | DIGIT | "_")*

DECIMAL_NUMBER = DIGIT+ ("e" ("+" | "-")? DIGIT+)?
               | DIGIT+ ("_" DIGIT+)*

HEX_NUMBER = "0x" HEX_DIGIT+

LETTER = "a".."z" | "A".."Z" | "_"
DIGIT = "0".."9"
HEX_DIGIT = DIGIT | "a".."f" | "A".."F"

STRING_CONTENT = (any character except "\"" and newline | escape_seq)*

escape_seq = "\\" ("n" | "r" | "t" | "\\" | "\"" | "'" | "0" | "x" HEX_DIGIT{2})
```

---

## Comments

```ebnf
single_line_comment = "//" (any character except newline)* newline

multi_line_comment = "/*" (any character)* "*/"

doc_comment = "///" (any character except newline)* newline
            | "/**" (any character)* "*/"
```

---

## Whitespace

```ebnf
whitespace = " " | "\t" | "\n" | "\r"
```

Whitespace is ignored except where required for token separation.

---

## Reserved Keywords

```
abstract address assert bool break bytes calldata case catch constant
constructor continue contract default delete do else emit enum error
event external fallback false for function hex if immutable import
indexed int interface internal is library mapping memory modifier new
override payable pragma private public pure receive require return
returns revert storage string struct super this throw true try type
uint unchecked using view virtual while
```

---

## See Also

- [Types Reference](types.md)
- [Attributes Reference](attributes.md)
- [Language Overview](../guide/overview.md)
