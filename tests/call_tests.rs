#![feature(rustc_private)]

//! Tests for function call decoration detection.

use ferrous_owl::{DecoKind, TestCase, run_tests};

fn call_string_new() -> TestCase {
    TestCase::new(
        "call_string_new",
        r#"
        fn test() {
            let s = String::new();
            drop(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_call()
}

fn call_string_from() -> TestCase {
    TestCase::new(
        "call_string_from",
        r#"
        fn test() {
            let s = String::from("hello");
            drop(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_call()
}

fn call_vec_new() -> TestCase {
    TestCase::new(
        "call_vec_new",
        r#"
        fn test() {
            let v = Vec::<i32>::new();
            drop(v);
        }
    "#,
    )
    .cursor_on("v = Vec")
    .expect_call()
}

fn call_vec_macro() -> TestCase {
    // vec![] macro expands to code that moves the vector, not a direct call
    TestCase::new(
        "call_vec_macro",
        r#"
        fn test() {
            let v = vec![1, 2, 3];
            drop(v);
        }
    "#,
    )
    .cursor_on("v = vec!")
    .expect_move_at("v") // The macro produces a move, not a call
}

fn call_box_new() -> TestCase {
    TestCase::new(
        "call_box_new",
        r#"
        fn test() {
            let b = Box::new(42);
            drop(b);
        }
    "#,
    )
    .cursor_on("b = Box")
    .expect_call()
}

fn call_option_some() -> TestCase {
    // Some is an enum variant constructor, not a function call
    TestCase::new(
        "call_option_some",
        r#"
        fn test() {
            let opt = Some(42);
            drop(opt);
        }
    "#,
    )
    .cursor_on("opt = Some")
    .forbid(DecoKind::Call)
}

fn call_result_ok() -> TestCase {
    // Ok is an enum variant constructor, not a function call
    TestCase::new(
        "call_result_ok",
        r#"
        fn test() {
            let res: Result<i32, ()> = Ok(42);
            drop(res);
        }
    "#,
    )
    .cursor_on("res:")
    .forbid(DecoKind::Call)
}

fn call_hashmap_new() -> TestCase {
    TestCase::new(
        "call_hashmap_new",
        r#"
        use std::collections::HashMap;

        fn test() {
            let m = HashMap::<String, i32>::new();
            drop(m);
        }
    "#,
    )
    .cursor_on("m = HashMap")
    .expect_call()
}

fn call_custom_function() -> TestCase {
    TestCase::new(
        "call_custom_function",
        r#"
        fn create_string() -> String {
            String::from("hello")
        }

        fn test() {
            let s = create_string();
            drop(s);
        }
    "#,
    )
    .cursor_on("s = create")
    .expect_call()
}

fn call_to_string() -> TestCase {
    TestCase::new(
        "call_to_string",
        r#"
        fn test() {
            let n = 42;
            let s = n.to_string();
            drop(s);
        }
    "#,
    )
    .cursor_on("s = n")
    .expect_call()
}

fn call_default() -> TestCase {
    TestCase::new(
        "call_default",
        r#"
        fn test() {
            let s: String = Default::default();
            drop(s);
        }
    "#,
    )
    .cursor_on("s:")
    .expect_call()
}

fn call_collect() -> TestCase {
    TestCase::new(
        "call_collect",
        r#"
        fn test() {
            let v: Vec<i32> = (0..5).collect();
            drop(v);
        }
    "#,
    )
    .cursor_on("v:")
    .expect_call()
}

#[test]
fn all_call_tests() {
    run_tests(&[
        call_string_new(),
        call_string_from(),
        call_vec_new(),
        call_vec_macro(),
        call_box_new(),
        call_option_some(),
        call_result_ok(),
        call_hashmap_new(),
        call_custom_function(),
        call_to_string(),
        call_default(),
        call_collect(),
    ]);
}
