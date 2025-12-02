#![feature(rustc_private)]

//! Tests for move decoration detection.

use ferrous_owl::{TestCase, run_tests};

fn move_to_drop() -> TestCase {
    TestCase::new(
        "move_to_drop",
        r#"
        fn test() {
            let s = String::new();
            drop(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_to_function() -> TestCase {
    TestCase::new(
        "move_to_function",
        r#"
        fn consume(_s: String) {}

        fn test() {
            let s = String::from("hello");
            consume(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_into_vec() -> TestCase {
    TestCase::new(
        "move_into_vec",
        r#"
        fn test() {
            let s = String::new();
            let mut v = Vec::new();
            v.push(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_into_option() -> TestCase {
    TestCase::new(
        "move_into_option",
        r#"
        fn test() {
            let s = String::new();
            let _opt = Some(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_into_result() -> TestCase {
    TestCase::new(
        "move_into_result",
        r#"
        fn test() {
            let s = String::new();
            let _res: Result<String, ()> = Ok(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_into_box() -> TestCase {
    TestCase::new(
        "move_into_box",
        r#"
        fn test() {
            let s = String::new();
            let _b = Box::new(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_return_value() -> TestCase {
    TestCase::new(
        "move_return_value",
        r#"
        fn test() -> String {
            let s = String::from("hello");
            s
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_struct_field() -> TestCase {
    TestCase::new(
        "move_struct_field",
        r#"
        struct Wrapper { inner: String }

        fn test() {
            let s = String::new();
            let _w = Wrapper { inner: s };
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_tuple() -> TestCase {
    TestCase::new(
        "move_tuple",
        r#"
        fn test() {
            let s = String::new();
            let _t = (1, s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_closure_capture() -> TestCase {
    // With `move` keyword but only using s.len(), Rust may optimize to borrow
    // since len() only needs &self. The actual decoration is imm-borrow.
    TestCase::new(
        "move_closure_capture",
        r#"
        fn test() {
            let s = String::new();
            let f = move || s.len();
            let _ = f();
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_imm_borrow()
}

fn move_assignment() -> TestCase {
    TestCase::new(
        "move_assignment",
        r#"
        fn test() {
            let s = String::new();
            let _t = s;
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_move()
}

fn move_match_arm() -> TestCase {
    TestCase::new(
        "move_match_arm",
        r#"
        fn test() {
            let s = Some(String::new());
            match s {
                Some(inner) => drop(inner),
                None => {}
            }
        }
    "#,
    )
    .cursor_on("s = Some")
    .expect_move()
}

fn move_if_let() -> TestCase {
    TestCase::new(
        "move_if_let",
        r#"
        fn test() {
            let s = Some(String::new());
            if let Some(inner) = s {
                drop(inner);
            }
        }
    "#,
    )
    .cursor_on("s = Some")
    .expect_move()
}

fn move_for_loop() -> TestCase {
    TestCase::new(
        "move_for_loop",
        r#"
        fn test() {
            let v = vec![String::new(), String::new()];
            for s in v {
                drop(s);
            }
        }
    "#,
    )
    .cursor_on("v = vec!")
    .expect_move()
}

#[test]
fn all_move_tests() {
    run_tests(&[
        move_to_drop(),
        move_to_function(),
        move_into_vec(),
        move_into_option(),
        move_into_result(),
        move_into_box(),
        move_return_value(),
        move_struct_field(),
        move_tuple(),
        move_closure_capture(),
        move_assignment(),
        move_match_arm(),
        move_if_let(),
        move_for_loop(),
    ]);
}
