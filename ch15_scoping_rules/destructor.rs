/*
 * The notion of a destructor in Rust is provided through the Drop trait. The destructor is 
 * called when the resource goes out of scope. This trait is not required to be implemented 
 * for every type, only implement it for your type if you require its own destructor logic.
 *
 * Run the below example to see how the Drop trait works. When the variable in the main 
 * function goes out of scope the custom destructor will be invoked.
 */

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}
