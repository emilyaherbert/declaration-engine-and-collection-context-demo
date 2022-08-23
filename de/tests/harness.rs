use de::{
    compile,
    language::untyped::{
        constructors::*, declaration::constructors::*, expression::constructors::*, Application,
        File,
    },
    type_system::type_info::constructors::*,
};

use crate::helpers::{handle_u64_decl, handle_u64_impl, math_trait_decl, math_trait_impl};

mod helpers;

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
        &[func_param("param1", t_gen_("T"))],
        &[
            var_decl("x", Some(t_gen_("T")), var("param1")),
            var_decl("y", None, u8(5u8)),
            return_(var("x")),
        ],
        t_gen_("T"),
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

    let math_trait = math_trait_decl(t_u8());
    let math_impl = math_trait_impl(t_u8());
    let main_fn = func_decl("main", &[], &[], &[], t_unit());
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![math_trait, math_impl, main_fn],
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

    let data_decl = struct_(
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
        nodes: vec![data_decl, main_fn],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
}

#[test]
fn generic_struct_test() {
    println!(
        "\n\n**********************************************************************************"
    );

    let data_decl = struct_(
        "Data",
        &[type_param("T", None)],
        &[
            struct_field("field_one", t_u8()),
            struct_field("field_two", t_u32()),
            struct_field("field_three", t_gen_("T")),
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
                struct_exp_field("field_three", u64(100u64)),
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
                struct_exp_field("field_three", u16(1u16)),
            ],
        ),
    );
    let main_fn = func_decl("main", &[], &[], &[foo_decl, bar_decl], t_unit());
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![data_decl, main_fn],
    };
    let application = Application {
        files: vec![program_1],
    };
    println!("{}", application);
    let resolved_application = compile(application);
    println!("{}", resolved_application);
}

#[test]
fn generic_struct_with_trait_test() {
    println!(
        "\n\n**********************************************************************************"
    );

    let handle_u64_decl = handle_u64_decl();
    let data_decl = struct_(
        "Data",
        &[],
        &[
            struct_field("field_one", t_u8()),
            struct_field("field_two", t_u32()),
        ],
    );
    let point_decl = struct_(
        "Point",
        &[],
        &[
            struct_field("x_cord", t_u64()),
            struct_field("y_cord", t_u64()),
        ],
    );

    let impl_handle_for_data = handle_u64_impl(t_cus_("Data"), 99);
    let impl_handle_for_point = handle_u64_impl(t_cus_("Point"), 222);

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
            "Point",
            &[],
            &[
                struct_exp_field("x_cord", u64(99u64)),
                struct_exp_field("y_cord", u64(24u64)),
            ],
        ),
    );
    let main_fn = func_decl("main", &[], &[], &[foo_decl, bar_decl], t_unit());
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            handle_u64_decl,
            data_decl,
            point_decl,
            impl_handle_for_data,
            impl_handle_for_point,
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
fn method_call_test() {
    println!(
        "\n\n**********************************************************************************"
    );

    let handle_u64_decl = handle_u64_decl();
    let data_decl = struct_(
        "Data",
        &[],
        &[
            struct_field("field_one", t_u8()),
            struct_field("field_two", t_u32()),
        ],
    );
    let point_decl = struct_(
        "Point",
        &[],
        &[
            struct_field("x_cord", t_u64()),
            struct_field("y_cord", t_u64()),
        ],
    );

    let impl_handle_for_data = handle_u64_impl(t_cus_("Data"), 99);
    let impl_handle_for_point = handle_u64_impl(t_cus_("Point"), 222);

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
            "Point",
            &[],
            &[
                struct_exp_field("x_cord", u64(99u64)),
                struct_exp_field("y_cord", u64(24u64)),
            ],
        ),
    );
    let apple_decl = var_decl(
        "apple",
        None,
        method_app("foo", "handle_u64_fn", &[], &[u64(8u64)]),
    );
    let orange_decl = var_decl(
        "orange",
        None,
        method_app("bar", "handle_u64_fn", &[], &[u64(8u64)]),
    );
    let main_fn = func_decl(
        "main",
        &[],
        &[],
        &[foo_decl, bar_decl, apple_decl, orange_decl],
        t_unit(),
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            handle_u64_decl,
            data_decl,
            point_decl,
            impl_handle_for_data,
            impl_handle_for_point,
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
fn trait_constraint_test() {
    println!(
        "\n\n**********************************************************************************"
    );

    let handle_u64_decl = handle_u64_decl();
    let data_decl = struct_(
        "Data",
        &[],
        &[
            struct_field("field_one", t_u8()),
            struct_field("field_two", t_u32()),
        ],
    );
    let point_decl = struct_(
        "Point",
        &[],
        &[
            struct_field("x_cord", t_u64()),
            struct_field("y_cord", t_u64()),
        ],
    );

    let impl_handle_for_data = handle_u64_impl(t_cus_("Data"), 99);
    let impl_handle_for_point = handle_u64_impl(t_cus_("Point"), 222);

    let call_it_fn = func_decl(
        "call_it",
        &[type_param("T", Some("HandleU64"))],
        &[func_param("value", t_gen_("T"))],
        &[return_(method_app(
            "value",
            "handle_u64_fn",
            &[],
            &[u64(75u64)],
        ))],
        t_u64(),
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
            "Point",
            &[],
            &[
                struct_exp_field("x_cord", u64(99u64)),
                struct_exp_field("y_cord", u64(24u64)),
            ],
        ),
    );
    let apple_decl = var_decl("apple", None, func_app("call_it", &[], &[var("foo")]));
    let orange_decl = var_decl("orange", None, func_app("call_it", &[], &[var("bar")]));
    let main_fn = func_decl(
        "main",
        &[],
        &[],
        &[foo_decl, bar_decl, apple_decl, orange_decl],
        t_unit(),
    );
    let program_1 = File {
        name: "bob.sw".to_string(),
        nodes: vec![
            handle_u64_decl,
            data_decl,
            point_decl,
            impl_handle_for_data,
            impl_handle_for_point,
            call_it_fn,
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
