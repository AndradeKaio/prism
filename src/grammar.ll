simple_stmp | compound_stmt

compound_stmt : "if" "(" expr ")" "{" stmt "}" ["else" "if"  "("expr")" "{" stmt "}"] ["else" "{" stmt "}"] |
                "while" "(" expr ")" "{" stmt "}" |
                "fn" id "(" param_list ")" "{" "}" | 
            ;


func_def := "fn" id [: type] "(" param_list ")" "{" ["ret" (id | (int | byte | string | float | boolean)]  "}"

param_list := type id {',' type id }

type := "int" | "byte" | "string" | "float" | "boolean" ;


fn get_id  () {

    ret 2;
}