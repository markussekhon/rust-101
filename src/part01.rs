// Rust-101, Part 01: Expressions, Inherent methods
// ================================================

// For Rust to compile this file, make sure to enable the corresponding line
// in `main.rs` before going on.

//@ Even though our code from the first part works, we can still learn a
//@ lot by making it prettier. That's because Rust is an "expression-based" language, which
//@ means that most of the terms you write down are not just *statements* (executing code), but
//@ *expressions* (returning a value). This applies even to the body of entire functions!

// ## Expression-based programming
//@ For example, consider `sqr`:
fn sqr(i: i32) -> i32 {
    i * i
}
//@ Between the curly braces, we are giving the *expression* that computes the return value.
//@ So we can just write `i * i`, the expression that returns the square of `i`!
//@ This is very close to how mathematicians write down functions (but with more types).

// Conditionals are also just expressions. This is comparable to the ternary `? :` operator
// from languages like C.
fn abs(i: i32) -> i32 {
    if i >= 0 {
        i
    } else {
        -i
    }
}

//@ And the same applies to case distinction with `match`: Every `arm` of the match
//@ gives the expression that is returned in the respective case.
//@ (We repeat the definition from the previous part here.)
enum NumberOrNothing {
    Number(i32),
    Nothing,
}

// ## Inherent implementations
impl NumberOrNothing {
    fn print(self) {
        match self {
            Nothing => println!("The number is: <nothing>"),
            Number(n) => println!("The number is: {}", n),
        };
    }

    fn number_or_default(n: NumberOrNothing, default: i32) -> i32 {
        match n {
            Nothing => default,
            Number(n) => n,
        }
    }
}

use self::NumberOrNothing::{Nothing, Number};

// It is even the case that blocks are expressions, evaluating to the last expression they contain.
fn compute_stuff(x: i32) -> i32 {
    let y = {
        let z = x * x;
        z + 14
    };
    y * y
}

// Let us now refactor `vec_min`.
fn vec_min(v: Vec<i32>) -> NumberOrNothing {
    //@ Remember that helper function `min_i32`? Rust allows us to define such helper functions
    //@ *inside* other functions. This is just a matter of namespacing, the inner function has no
    //@ access to the data of the outer one. Still, being able to nicely group functions can
    //@ significantly increase readability.
    fn min_i32(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        } /*@*/
    }

    let mut min = Nothing;
    for e in v {
        //@ Notice that all we do here is compute a new value for `min`, and that it will always end
        //@ up being a `Number` rather than `Nothing`. In Rust, the structure of the code
        //@ can express this uniformity.
        min = Number(match min {
            /*@*/
            Nothing => e,               /*@*/
            Number(n) => min_i32(n, e), /*@*/
        }); /*@*/
    }
    //@ The `return` keyword exists in Rust, but it is rarely used. Instead, we typically
    //@ make use of the fact that the entire function body is an expression, so we can just
    //@ write down the desired return value.
    min
}

// With our refactored functions and methods, `main` now looks as follows:
fn read_vec() -> Vec<i32> {
    vec![18, 5, 7, 2, 9, 27]
}

pub fn main() {
    let vec = read_vec();
    let min = vec_min(vec);
    min.print();

    let vec = read_vec();
    vec_sum(vec);

    let vec = read_vec();
    vec_print(vec);
}

// **Exercise 01.1**: Write a function `vec_sum` that computes the sum of all values of a `Vec<i32>`.
fn vec_sum(v: Vec<i32>) -> i128 {
    let mut sum: i128 = 0;
    for e in v {
        sum += e as i128
    }
    sum
}

// **Exercise 01.2**: Write a function `vec_print` that takes a vector and prints all its elements.
fn vec_print(v: Vec<i32>) {
    print!("The vector contains: ");
    for e in v {
        print!("{} ", e)
    }
    println!()
}
