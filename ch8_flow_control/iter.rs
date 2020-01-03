// iter
// ----
// This borrows each element of the collection through each iteration. Thus leaving the 
// collection untouched and available for reuse after the loop.

fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
