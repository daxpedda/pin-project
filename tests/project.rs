#![recursion_limit = "128"]
#![no_std]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]
#![cfg(feature = "project_attr")]
#![feature(proc_macro_hygiene, stmt_expr_attributes)]

use core::pin::Pin;
use pin_project::{pin_project, project};

#[project] // Nightly does not need a dummy attribute to the function.
#[test]
fn test_project_attr() {
    // struct

    #[pin_project]
    struct Foo<T, U> {
        #[pin]
        field1: T,
        field2: U,
    }

    let mut foo = Foo { field1: 1, field2: 2 };

    #[project]
    let Foo { field1, field2 } = Pin::new(&mut foo).project();

    let x: Pin<&mut i32> = field1;
    assert_eq!(*x, 1);

    let y: &mut i32 = field2;
    assert_eq!(*y, 2);

    // tuple struct

    #[pin_project]
    struct Bar<T, U>(#[pin] T, U);

    let mut bar = Bar(1, 2);

    #[project]
    let Bar(x, y) = Pin::new(&mut bar).project();

    let x: Pin<&mut i32> = x;
    assert_eq!(*x, 1);

    let y: &mut i32 = y;
    assert_eq!(*y, 2);

    // enum

    #[pin_project]
    enum Baz<A, B, C, D> {
        Variant1(#[pin] A, B),
        Variant2 {
            #[pin]
            field1: C,
            field2: D,
        },
        None,
    }

    let mut baz = Baz::Variant1(1, 2);

    let mut baz = Pin::new(&mut baz).project();

    #[project]
    match &mut baz {
        Baz::Variant1(x, y) => {
            let x: &mut Pin<&mut i32> = x;
            assert_eq!(**x, 1);

            let y: &mut &mut i32 = y;
            assert_eq!(**y, 2);
        }
        Baz::Variant2 { field1, field2 } => {
            let _x: &mut Pin<&mut i32> = field1;
            let _y: &mut &mut i32 = field2;
        }
        Baz::None => {}
    }
}

#[test]
fn test_project_attr_nightly() {
    // enum

    #[pin_project]
    enum Baz<A, B, C, D> {
        Variant1(#[pin] A, B),
        Variant2 {
            #[pin]
            field1: C,
            field2: D,
        },
        None,
    }

    let mut baz = Baz::Variant1(1, 2);

    let mut baz = Pin::new(&mut baz).project();

    #[project]
    match &mut baz {
        Baz::Variant1(x, y) => {
            let x: &mut Pin<&mut i32> = x;
            assert_eq!(**x, 1);

            let y: &mut &mut i32 = y;
            assert_eq!(**y, 2);
        }
        Baz::Variant2 { field1, field2 } => {
            let _x: &mut Pin<&mut i32> = field1;
            let _y: &mut &mut i32 = field2;
        }
        Baz::None => {}
    }

    let () = #[project]
    match &mut baz {
        Baz::Variant1(x, y) => {
            let x: &mut Pin<&mut i32> = x;
            assert_eq!(**x, 1);

            let y: &mut &mut i32 = y;
            assert_eq!(**y, 2);
        }
        Baz::Variant2 { field1, field2 } => {
            let _x: &mut Pin<&mut i32> = field1;
            let _y: &mut &mut i32 = field2;
        }
        Baz::None => {}
    };
}
