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

    let main_fn = func_decl(
        "main",
        &[],
        &[],
        &[var_decl("x", None, u8(5u8)), return_(var("x"))],
        t_u8(),
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![main_fn.clone()],
    };
    let program_2 = File {
        name: "alice.sw".to_string(),
        nodes: vec![main_fn],
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

    let f_fn = func_decl(
        "F",
        &[],
        &[],
        &[var_decl("x", None, u8(5u8)), return_(var("x"))],
        t_u8(),
    );
    let main_fn = func_decl("main", &[], &[], &[], t_unit());
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![f_fn, main_fn],
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

    let f_fn = func_decl(
        "F",
        &[],
        &[func_param("param1", t_u32())],
        &[
            var_decl("x", None, var("param1")),
            var_decl("y", None, u8(5u8)),
            return_(var("x")),
        ],
        t_u32(),
    );
    let main_fn = func_decl(
        "main",
        &[],
        &[],
        &[var_decl("foo", None, func_app("F", &[], &[u32(1u32)]))],
        t_unit(),
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![f_fn, main_fn],
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

    let f_fn = func_decl(
        "F",
        &[],
        &[func_param("param1", t_u32())],
        &[
            var_decl("x", None, var("param1")),
            var_decl("y", None, u8(5u8)),
            return_(var("x")),
        ],
        t_u64(),
    );
    let main_fn = func_decl(
        "main",
        &[],
        &[],
        &[var_decl("foo", None, func_app("F", &[], &[u32(1u32)]))],
        t_unit(),
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![f_fn, main_fn],
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

    let f_fn = func_decl(
        "F",
        &[type_param("T", None)],
        &[func_param("param1", t_("T"))],
        &[
            var_decl("x", None, var("param1")),
            var_decl("y", None, u8(5u8)),
            return_(var("x")),
        ],
        t_("T"),
    );
    let main_fn = func_decl(
        "main",
        &[],
        &[],
        &[
            var_decl("foo", None, func_app("F", &[], &[u32(1u32)])),
            var_decl("bar", None, func_app("F", &[], &[u64(1u64)])),
        ],
        t_unit(),
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![f_fn, main_fn],
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
    let add_impl = func_decl_raw(
        "add_fn",
        &[],
        &[func_param("a", t_u8()), func_param("b", t_u8())],
        &[
            var_decl("x", None, var("a")),
            var_decl("y", None, var("b")),
            return_(var("x")),
        ],
        t_u8(),
    );
    let sub_impl = func_decl_raw(
        "sub_fn",
        &[],
        &[func_param("a", t_u8()), func_param("b", t_u8())],
        &[
            var_decl("x", None, var("a")),
            var_decl("y", None, var("b")),
            return_(var("y")),
        ],
        t_u8(),
    );
    let main_fn = func_decl("main", &[], &[], &[], t_unit());
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            trait_("Math", &[add_fn, sub_fn]),
            trait_impl("Math", t_u8(), &[], &[add_impl, sub_impl]),
            main_fn,
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
fn struct_test() {
    println!(
        "\n\n**********************************************************************************"
    );

    let struct_data = struct_(
        "Data",
        &[],
        &[
            struct_field("field_one", t_u8()),
            struct_field("field_two", t_u32()),
        ],
    );
    let foo_decl = var_decl(
        "foo",
        None,
        struct_exp(
            "Data",
            &[],
            &[
                struct_exp_field("field_one", u8(2u8)),
                struct_exp_field("field_two", u32(3u32)),
            ],
        ),
    );
    let bar_decl = var_decl(
        "bar",
        None,
        struct_exp(
            "Data",
            &[],
            &[
                struct_exp_field("field_one", u8(99u8)),
                struct_exp_field("field_two", u32(24u32)),
            ],
        ),
    );
    let main_fn = func_decl("main", &[], &[], &[foo_decl, bar_decl], t_unit());
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![struct_data, main_fn],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
}
