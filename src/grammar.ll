simple_stmp | compound_stmt

compound_stmt : "if" "(" expr ")" "{" stmt "}" ["else" "if"  "("expr")" "{" stmt "}"] ["else" "{" stmt "}"] |
                "while" "(" expr ")" "{" stmt "}" |
                "fn" "(" param_list ")" "{" "}" | 