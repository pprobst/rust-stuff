/*
 * Result is a richer version of the Option type that describes possible error instead of 
 * possible absence.
 *
 * That is, Result<T, E> could have one of two outcomes:
 *
 * - Ok(T): An element T was found.
 * - Err(E): An error was found with element E.
 *
 * By convention, the expected outcome is Ok while the unexpected outcome is Err.
 */

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
