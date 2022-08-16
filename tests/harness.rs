use declaration_engine::{
    compile,
    language::untyped::{
        constructors::*, declaration::constructors::*, expression::constructors::*, Tree,
    },
    type_system::type_info::constructors::*,
};

#[test]
fn var_decl_test() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program = Tree {
        nodes: vec![var_decl("x", None, u8(5u8)), exp(var("x"))],
    };
    println!("{}", program);
    let resolved_program = compile(program, false);
    println!("{}", resolved_program);
    //panic!();
}

#[test]
fn func_decl_test() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program = Tree {
        nodes: vec![func_decl(
            "f",
            &[],
            &[],
            &[var_decl("x", None, u8(5u8)), return_(var("x"))],
            t_u8(),
        )],
    };
    println!("{}", program);
    let resolved_program = compile(program, false);
    println!("{}", resolved_program);
    //panic!();
}
