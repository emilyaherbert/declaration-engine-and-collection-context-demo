use de::{
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
                &[func_param("param1", t_u32())],
                &[
                    var_decl("x", None, var("param1")),
                    var_decl("y", None, u8(5u8)),
                    return_(var("x")),
                ],
                t_u32(),
            ),
            var_decl("foo", None, func_app("F", &[], &[u32(1u32)])),
        ],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
}

#[test]
#[should_panic]
fn func_app_error_test() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            func_decl(
                "F",
                &[],
                &[func_param("param1", t_u32())],
                &[
                    var_decl("x", None, var("param1")),
                    var_decl("y", None, u8(5u8)),
                    return_(var("x")),
                ],
                t_u64(),
            ),
            var_decl("foo", None, func_app("F", &[], &[u32(1u32)])),
        ],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
}

#[test]
fn generic_func_test() {
    println!(
        "\n\n**********************************************************************************"
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            func_decl(
                "F",
                &[type_param("T")],
                &[func_param("param1", t_("T"))],
                &[
                    var_decl("x", None, var("param1")),
                    var_decl("y", None, u8(5u8)),
                    return_(var("x")),
                ],
                t_("T"),
            ),
            var_decl("foo", None, func_app("F", &[], &[u32(1u32)])),
            var_decl("bar", None, func_app("F", &[], &[u64(1u64)])),
        ],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
}

#[test]
fn trait_test() {
    println!(
        "\n\n**********************************************************************************"
    );
    let add_fn = trait_fn(
        "add_fn",
        &[func_param("a", t_u8()), func_param("b", t_u8())],
        t_u8(),
    );
    let sub_fn = trait_fn(
        "sub_fn",
        &[func_param("a", t_u8()), func_param("b", t_u8())],
        t_u8(),
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            trait_("Math", &[add_fn, sub_fn]),
            func_decl(
                "F",
                &[type_param("T")],
                &[func_param("param1", t_("T"))],
                &[
                    var_decl("x", None, var("param1")),
                    var_decl("y", None, u8(5u8)),
                    return_(var("x")),
                ],
                t_("T"),
            ),
            var_decl("foo", None, func_app("F", &[], &[u32(1u32)])),
            var_decl("bar", None, func_app("F", &[], &[u64(1u64)])),
        ],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
}
