smt: simple_stmt | compound_stmt

simple_stmt: var_decl

compound_stmt: "if" "(" expr ")" "{" stmt "}" ["else" "if"  "("expr")" "{" stmt "}"] ["else" "{" stmt "}"] |
                "while" "(" expr ")" "{" stmt "}" |
                "fn" id "(" func_param_list ")" "{" "}"


func_def : "fn" id [: return_type] "(" func_param_list ")" "{" ["ret" (id | literal]  "}"

func_param_list : type id {',' type id }

var_decl : type id ['=' ['-'] literal] {',' id ['=' ['-'] literal]} ';'

type := "int" | "byte" | "string" | "float" | "bool"

literal : int_literal | byte_literal | string_literal | float_literal | bool_literal