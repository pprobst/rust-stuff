/*
 * A consequence of how bounds work is that even if a trait doesn't include any functionality,
 * you can still use it as a bound. Eq and Ord are examples of such traits from the std library.
 */

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}
// impl Red for Turkey {}

// These funtions are only valid for types which implement these traits. The fact that the traits
// are empty is irrelevant.
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey  = Turkey;

    // 'red()' won't work on a blue jay nor vice versa because of the bounds.
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue hay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));
}
