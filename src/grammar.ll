smt: simple_stmt | compound_stmt

simple_stmt: var_decl

compound_stmt: "if" "(" expr ")" "{" stmt "}" ["else" "if"  "("expr")" "{" stmt "}"] ["else" "{" stmt "}"] |
                "while" "(" expr ")" "{" stmt "}" |
                func_def

expr: exprs [('<' | '>'| '<=' | '>=' | '!=' | '==') exprs]

exprs: [ '+'| '-'] term { ('+' | '-' | 'or') term}

term: factor {('*'| '/' | 'and') factor)}

factor: "(" expr ")" | id | literal | 'not' factor

func_def : "fn" id "(" func_param_list ")" [: return_type] "{" ["ret" (id | literal]  "}"

func_param_list : type id {',' type id }

#var_decl : type id ['=' expr ] {',' id ['=' expr ]} ';'
var_decl : type id ['=' expr ] ';'

type := "int" | "byte" | "string" | "float" | "bool"

literal : int_literal | byte_literal | string_literal | float_literal | bool_literal
