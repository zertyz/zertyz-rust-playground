#![doc = include_str!("../README.md")]
#![recursion_limit="256"]

fn main() {
    eprintln!("######################");
    eprintln!("## META PROGRAMMING ##");
    eprintln!("######################");

    generics_expressions();
    concrete_constexprs();
    macro_constexprs();
}

use std::dbg;
use std::ops::{Add, Sub, Mul, Div};

fn eq<T: PartialEq> (a: T, b: T) -> bool {
    a == b
}

fn add<T: Add<Output = T>> (a: T, b: T) -> T {
    a + b
}

fn sub<T: Sub<Output = T>> (a:T, b: T) -> T {
    a - b
}

fn mul<T: Mul<Output = T>> (a:T, b: T) -> T {
    a * b
}

fn div<T: Div<Output = T>> (a:T, b: T) -> T {
    a / b
}

use std::cmp::PartialEq;
use num::FromPrimitive;

fn fact<T: Sub<Output=T> + Mul<Output=T> + PartialEq + PartialOrd + Copy + FromPrimitive> (n: T) -> T {
    if n <= T::from_usize(0).unwrap() {
        T::from_usize(1).unwrap()
    } else {
        mul::<T>(fact::<T>(sub::<T>(n, T::from_usize(1).unwrap())), n as T)
    }
}

fn fib<T: Sub<Output=T> + Add<Output=T> + PartialEq + PartialOrd + Copy + FromPrimitive> (n: T) -> T {
    if n <= T::from_usize(0).unwrap() {
        T::from_usize(0).unwrap()
    } else if n <= T::from_usize(1).unwrap() {
        T::from_usize(1).unwrap()
    } else {
        add::<T>(
            fib::<T>(sub::<T>(n, T::from_usize(1).unwrap())),
            fib::<T>(sub::<T>(n, T::from_usize(2).unwrap()))
        )
    }
}

fn generics_expressions() {
    eprintln!();
    eprintln!("# Compile-time generics expansion:");
    eprintln!("##################################");
    eprintln!("# on rustc 1.53.0, adding a const to the following functions gives the rather cryptic/buggy error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable");
    eprintln!("# conclusion: constexprs with generics in rust 2018 are not possible. We might try it with macros and without generics");
    dbg!(eq(1, 1));
    dbg!(eq(1.23, 1.23));
    dbg!(eq(10.05, 10.04));
    dbg!(add(1, 2));
    dbg!(add(1.23, 2.31));
    dbg!(sub(2, 1));
    dbg!(sub(2.31, 1.23));
    dbg!(mul(2, 1));
    dbg!(mul(2.31, 1.23));
    dbg!(div(2, 1));
    dbg!(div(2.31, 1.23));
    dbg!(fact(3));
    dbg!(fact(4));
    dbg!(fact(5.1));
    dbg!(fib(8));
    dbg!(fib(9));
    dbg!(fib(5.1));
}

const fn usize_fib(n: usize) -> usize {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        usize_fib(n-1) + usize_fib(n-2)
    }
}

// proof that usize_fib is evaluated at compile-time:
static A: [u32; usize_fib(8)] = [0; usize_fib(8)];


fn concrete_constexprs() {
    eprintln!();
    eprintln!("# Concrete constant expansion:");
    eprintln!("##############################");
    dbg!(usize_fib(8));
    dbg!(usize_fib(9));
}

macro_rules! mfib {
    ($n: literal) => {
        if $n == 0 {
            0
        }  else if $n == 1 {
            1
        } else {
            mfib!(n-1) + mfib!(n-2)
        }
    }
}

macro_rules! recurrence {
    (a[n] = [$($inits:expr),+ ], $recur:expr) => {
        {
            println!("inits={}, recur={}", "$inits", $recur);
            recurrence!($($inits),*);
        }
    };
    ($str:literal) => {println!("literal='{}'", $str);};
    ($head:expr) => {println!("expr={}", $head);};
    ($head:expr, $($tail:expr),*) => {println!("composed={}", $head); recurrence!($($tail),*);};
    ($shit:stmt;) => {println!("starting...");$shit;println!("done");};
}

fn doit() {
    println!("  ran...");
}

fn macro_constexprs() {
    eprintln!();
    eprintln!("## Macro constant expansion:");
    eprintln!("############################");
    eprintln!("# wonderful book here: https://danielkeep.github.io/tlborm/book/mbe-min-captures-and-expansion-redux.html");
    //eprintln!(mfib!(8));
    //eprintln!(mfib!(9));

    let fib = recurrence![a[n] = [0, "caca"], "a[n-1] + a[n-2]"];
    let fib2 = recurrence!(10+2);
    recurrence!(doit(););
    //for e in fib.take(10) { println!("{}", e) }
    dbg!(fib);
}

macro_rules! make_function {
    ($name:ident, $value:expr) => {
        #[doc = concat!("The `", stringify!($name), "` example.")]
        ///
        /// # Example
        ///
        /// ```
        #[doc = concat!(
            "assert_eq!(", module_path!(), "::", stringify!($name), "(), ",
            stringify!($value), ");")
        ]
        /// ```
        pub fn $name() -> i32 {
            $value
        }
    };
}
make_function! {func_name, 123}

make_function! {func_noname, 1234}

#[cfg(any(test, feature = "dox"))]
mod tests {
    //! This is some important set of tests
    //!

    use super::*;

    /// The test enforces some business rules I, hereby, document
    #[cfg_attr(not(feature = "dox"), test)]
    fn it_works() {
        assert!(working_fn());
    }
}
