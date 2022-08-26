#![allow(dead_code)]

use de_cc::{
    language::untyped::{
        constructors::*, declaration::constructors::*, expression::constructors::*, Node,
    },
    type_system::type_info::{constructors::t_u64, TypeInfo},
};

pub(crate) fn handle_u64_decl() -> Node {
    let handle_u64_fn = trait_fn("handle_u64_fn", &[func_param("n", t_u64())], t_u64());
    trait_("HandleU64", &[handle_u64_fn])
}

pub(crate) fn handle_u64_impl(type_implementing_for: TypeInfo, n: u64) -> Node {
    let handle_u64_fn = func_decl_raw(
        "handle_u64_fn",
        &[],
        &[func_param("n", t_u64())],
        &[return_(u64(n))],
        t_u64(),
    );
    trait_impl("HandleU64", type_implementing_for, &[], &[handle_u64_fn])
}

pub(crate) fn math_trait_decl(type_info: TypeInfo) -> Node {
    let add_fn = trait_fn(
        "add_fn",
        &[
            func_param("a", type_info.clone()),
            func_param("b", type_info.clone()),
        ],
        type_info.clone(),
    );
    let sub_fn = trait_fn(
        "sub_fn",
        &[
            func_param("a", type_info.clone()),
            func_param("b", type_info.clone()),
        ],
        type_info,
    );
    trait_("Math", &[add_fn, sub_fn])
}

pub(crate) fn math_trait_impl(type_info: TypeInfo) -> Node {
    let add_impl = func_decl_raw(
        "add_fn",
        &[],
        &[
            func_param("a", type_info.clone()),
            func_param("b", type_info.clone()),
        ],
        &[
            var_decl("x", None, var("a")),
            var_decl("y", None, var("b")),
            return_(var("x")),
        ],
        type_info.clone(),
    );
    let sub_impl = func_decl_raw(
        "sub_fn",
        &[],
        &[
            func_param("a", type_info.clone()),
            func_param("b", type_info.clone()),
        ],
        &[
            var_decl("x", None, var("a")),
            var_decl("y", None, var("b")),
            return_(var("y")),
        ],
        type_info.clone(),
    );
    trait_impl("Math", type_info, &[], &[add_impl, sub_impl])
}
