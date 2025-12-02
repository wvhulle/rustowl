#![feature(rustc_private)]

//! Tests for combined decoration scenarios.

use ferrous_owl::{DecoKind, ExpectedDeco, TestCase, run_tests};

fn combined_call_and_move() -> TestCase {
    TestCase::new(
        "combined_call_and_move",
        r#"
        fn test() {
            let s = String::new();
            drop(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_call()
    .expect_move()
}

fn combined_call_and_borrow() -> TestCase {
    TestCase::new(
        "combined_call_and_borrow",
        r#"
        fn test() {
            let s = String::from("hello");
            println!("{}", s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_call()
    .expect_imm_borrow()
}

fn combined_call_and_mut_borrow() -> TestCase {
    TestCase::new(
        "combined_call_and_mut_borrow",
        r#"
        fn test() {
            let mut v = Vec::new();
            v.push(1);
        }
    "#,
    )
    .cursor_on("v = Vec")
    .expect_call()
    .expect_mut_borrow()
}

fn combined_multiple_borrows() -> TestCase {
    TestCase::new(
        "combined_multiple_borrows",
        r#"
        fn test() {
            let s = String::from("hello");
            let _len = s.len();
            let _chars = s.chars().count();
            println!("{}", s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_imm_borrow()
}

fn combined_borrow_then_move() -> TestCase {
    TestCase::new(
        "combined_borrow_then_move",
        r#"
        fn test() {
            let s = String::from("hello");
            let _len = s.len();
            drop(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_imm_borrow()
    .expect_move()
}

fn combined_mut_borrow_then_move() -> TestCase {
    TestCase::new(
        "combined_mut_borrow_then_move",
        r#"
        fn test() {
            let mut v = vec![1, 2, 3];
            v.push(4);
            drop(v);
        }
    "#,
    )
    .cursor_on("v = vec!")
    .expect_mut_borrow()
    .expect_move()
}

fn combined_with_text_match() -> TestCase {
    TestCase::new(
        "combined_with_text_match",
        r#"
        fn test() {
            let s = String::new();
            drop(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect(ExpectedDeco::new(DecoKind::Move).with_message("moved"))
}

fn combined_forbid_and_expect() -> TestCase {
    TestCase::new(
        "combined_forbid_and_expect",
        r#"
        fn test() {
            let s = String::from("hello");
            let _len = s.len();
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_imm_borrow()
    .forbid_move()
}

fn combined_multiple_variables() -> TestCase {
    TestCase::new(
        "combined_multiple_variables",
        r#"
        fn test() {
            let a = String::new();
            let b = String::new();
            drop(a);
            drop(b);
        }
    "#,
    )
    .cursor_on("a = String")
    .expect_move()
}

fn combined_nested_function_calls() -> TestCase {
    TestCase::new(
        "combined_nested_function_calls",
        r#"
        fn process(s: String) -> String {
            s.to_uppercase()
        }

        fn test() {
            let s = String::from("hello");
            let _result = process(s);
        }
    "#,
    )
    .cursor_on("s = String")
    .expect_call()
    .expect_move()
}

fn combined_struct_with_methods() -> TestCase {
    TestCase::new(
        "combined_struct_with_methods",
        r#"
        struct Counter {
            count: i32,
        }

        impl Counter {
            fn new() -> Self {
                Counter { count: 0 }
            }

            fn increment(&mut self) {
                self.count += 1;
            }

            fn get(&self) -> i32 {
                self.count
            }
        }

        fn test() {
            let mut c = Counter::new();
            c.increment();
            let _val = c.get();
        }
    "#,
    )
    .cursor_on("c = Counter")
    .expect_call()
    .expect_mut_borrow()
    .expect_imm_borrow()
}

fn combined_option_methods() -> TestCase {
    // Some(...) is an enum variant constructor, not a function call
    // The methods is_some() and as_ref() create immutable borrows
    TestCase::new(
        "combined_option_methods",
        r#"
        fn test() {
            let opt = Some(String::from("hello"));
            let _is_some = opt.is_some();
            let _ref = opt.as_ref();
        }
    "#,
    )
    .cursor_on("opt = Some")
    .expect_imm_borrow()
}

#[test]
fn all_combined_tests() {
    run_tests(&[
        combined_call_and_move(),
        combined_call_and_borrow(),
        combined_call_and_mut_borrow(),
        combined_multiple_borrows(),
        combined_borrow_then_move(),
        combined_mut_borrow_then_move(),
        combined_with_text_match(),
        combined_forbid_and_expect(),
        combined_multiple_variables(),
        combined_nested_function_calls(),
        combined_struct_with_methods(),
        combined_option_methods(),
    ]);
}
