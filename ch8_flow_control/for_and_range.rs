fn main() {
    // The 'for in' construct can be used to iterate through an Iterator.
    // 'n' will take the values: 1, 2, ..., 100 in each iteration.
    for n in 1..101 {      // EXCLUSIVE RANGE (DEFAULT)
    // for n in 1..=100 {  // INCLUSIVE RANGE ON BOTH ENDS
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}
