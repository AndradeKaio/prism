struct gen;

pub trait Codegen {

    fn gen(&self) -> String;

}
