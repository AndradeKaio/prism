//pub mod lexer;
//pub mod parser;

#[macro_export]
macro_rules! match_tok{
    ($r:expr, $l: expr) => {
        match $r {
            $l => true,
            _ => false,
        }
    };

}



#[test]
fn true_test() {
    assert!(match_tok!(1, 1), true);
    assert!(match_tok!("string", "string"), true);
}
#[test]
fn false_test(){
//    assert!(match_token!(1, 2), false);
    assert!(match_tok!(true, false), false);
}
