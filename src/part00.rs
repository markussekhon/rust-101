// Rust-101, Part 00: Algebraic datatypes
// ======================================

// An `enum` for "a number or nothing" could look as follows:
// Almost like we made an optional
enum NumberOrNothing {
    Number(i32),
    Nothing,
}

// Adding to our namespace
use self::NumberOrNothing::{Nothing, Number};

// Writing our first function
fn vector_minimum(vector: Vec<i32>) -> NumberOrNothing {
    let mut current = Nothing;

    for element in vector {
        match current {
            Nothing => {
                current = Number(element);
            }
            Number(n) => {
                current = Number(minimum_i32(element, n));
            }
        }
    }

    return current;
}

fn minimum_i32(a: i32, b: i32) -> i32 {
    if a < b {
        return a;
    } else {
        return b;
    }
}

fn read_vector() -> Vec<i32> {
    vec![4, 8, 9, 3, 2, 4]
}

fn print_number_or_nothing(n: NumberOrNothing) {
    match n {
        Nothing => println!("We have nothing!"),
        Number(n) => println!("This is the number: {}", n),
    }
}

// Putting it all together:
pub fn main() {
    let vector = read_vector();
    let minimum = vector_minimum(vector);
    print_number_or_nothing(minimum);

    let vector = vec![];
    let minimum = vector_minimum(vector);
    print_number_or_nothing(minimum);
}

// Finally, try `cargo run` on the console to run it.
