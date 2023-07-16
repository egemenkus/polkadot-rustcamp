use std::io::{self, Write};

/// assert_eq!(check(Some((3,4,5))),true);
///
///
/// # Parameters
/// `nums` - An optional tuple containing the sides of a triangle
/// `Some(side_a,side_b,hypotenuse)` - Program will check if it is a pythagorean triple
///
/// # Returns
/// bool - If passed parameters, returns whether it was a pythagorean triple or not
/// a^2 + b^2 == c^2  

pub fn check(nums: Option<(i32, i32, i32)>) -> bool {
    println!("Kushynoda's Pythagorian Triple Checker");
    let mut from_parameters = false;
    let mut is_triple = false;

    while !from_parameters {
        let (side_a, side_b, side_c) = match nums {
            Some(i) => {
                from_parameters = true;
                i
            }
            None => prompt(),
        };
        if side_a.pow(2) + side_b.pow(2) == side_c.pow(2) {
            println!("It's Pythagorian Triple");
            is_triple = true;
        } else {
            println!("It's not a Pythagorian Triple");
            is_triple = false;
        }
    }
    is_triple
}

pub fn prompt() -> (i32, i32, i32) {
    let mut side_a = String::new();
    let mut side_b = String::new();
    let mut side_c = String::new();
    let stdin = io::stdin();

    print!("Side A: ");
    io::stdout().flush().expect("FAIL : flush to screen");

    stdin
        .read_line(&mut side_a)
        .expect("FAIL : read from stdin");

    let side_a: i32 = match side_a.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("FAIL: Side A is Invalid!");
            std::process::exit(1);
        }
    };

    print!("Side B: ");
    io::stdout().flush().expect("FAIL : flush to screen");

    stdin
        .read_line(&mut side_b)
        .expect("FAIL : read from stdin");
    let side_b: i32 = match side_b.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("FAIL: Side B is Invalid!");
            std::process::exit(1);
        }
    };

    print!("Hypotenuse: ");
    io::stdout().flush().expect("FAIL : flush to screen");

    stdin
        .read_line(&mut side_c)
        .expect("FAIL : read from stdin");
    let side_c: i32 = match side_c.trim().parse() {
        Ok(i) => i,
        Err(_) => {
            println!("FAIL: The Hypotenuse is Invalid!");
            std::process::exit(1);
        }
    };
    (side_a, side_b, side_c)
}
