use declaration_engine::{
    compile,
    language::untyped::{
        constructors::*, declaration::constructors::*, expression::constructors::*, Tree,
    },
};

#[test]
fn var_decl_test() {
    let program = Tree {
        nodes: vec![var_decl("x", None, u8(5u8)), exp(var("x"))],
    };
    println!("{:#?}", program);
    let resolved_program = compile(program);
    println!("{:#?}", resolved_program);
    assert!(false);
}
