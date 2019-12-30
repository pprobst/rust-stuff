/* 
 * Printing is handled by a series of macros defined in std::fmt, some of which include:
 * format!   : write formatted text to String
 * print!    : same as format! but the text is printed to the console (io::stdout)
 * println!  : same as print! but with a newline appended
 * eprint!   : same as format! but the text is printed to the standard error (io::stderr)
 * eprintln! : same as eprint! but with a newline appended
 *
 * All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.
 */

fn main() {
    // In general, '{}' will be automatically replaced with any arguments.
    // These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes i32. You can change what type 31 is 
    // by providing a suffix.

    // There are various optional patterns this works with. Positional arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can be named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a ':'.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a '1'.
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This is output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond"); <-- WRONG!
    println!("My name is {0}, {1} {0}.", "Bond", "James");

    // Create a structure named 'Structure', which contains an 'i32'.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom typed such as this structure require more complicated handling.
    // This will not work.
    // println!("This struct '{}' won't print...", Structure(3));
    // (just comment this for now)

    /* Activities:
     *
     * 1) Fix the two issues in the above code (see FIXME) so that it runs without error.
     *
     * 2) Add a println! macro that prints: Pi is roughly 3.142 by controlling the number 
     *    of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 
     *    as an estimate for pi. (Hint: you may need to check the std::fmt documentation for 
     *    setting the number of decimals to display)
     */

    // https://doc.rust-lang.org/std/fmt/
    // Hello {next arg ("x")} is {arg "number" (0.01) with precision specified
    //                           in arg "prec" (5)}
    println!("Hello {} is {number:.prec$}", "x", prec = 5, number = 0.01);

    let pi = 3.141592;
    println!("{:.*}", 3, pi);
    println!("{:.3}", pi);
}

