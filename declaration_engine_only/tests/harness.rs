use declaration_engine::{
    compile,
    language::untyped::{
        constructors::*, declaration::constructors::*, expression::constructors::*, Application,
        File,
    },
    type_system::type_info::constructors::*,
};

#[test]
fn var_decl_test() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![var_decl("x", None, u8(5u8)), exp(var("x"))],
    };
    let program_2 = File {
        name: "alice.sw".to_string(),
        nodes: vec![var_decl("x", None, u8(5u8)), exp(var("x"))],
    };
    let application = Application {
        files: vec![program_1, program_2],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
    //panic!();
}

#[test]
fn func_decl_test() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![func_decl(
            "F",
            &[],
            &[],
            &[var_decl("x", None, u8(5u8)), return_(var("x"))],
            t_u8(),
        )],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
    //panic!();
}

#[test]
fn func_app_test() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            func_decl(
                "F",
                &[],
                &[],
                &[var_decl("x", None, u8(5u8)), return_(var("x"))],
                t_u8(),
            ),
            var_decl("foo", None, func_app("F", &[], &[])),
        ],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
    //panic!();
}

#[test]
fn out_of_order() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            var_decl("foo", None, func_app("F", &[], &[])),
            func_decl(
                "F",
                &[],
                &[],
                &[var_decl("x", None, u8(5u8)), return_(var("x"))],
                t_u8(),
            ),
        ],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
    //panic!();
}

#[test]
#[should_panic]
fn out_of_order_type_error() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            var_decl("foo", Some(t_u32()), func_app("F", &[], &[])),
            func_decl(
                "F",
                &[],
                &[],
                &[var_decl("x", None, u8(5u8)), return_(var("x"))],
                t_u8(),
            ),
        ],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
    //panic!();
}
