/*
 * When data is immutably borrowed, it also freezes. Frozen data can't be modified via the 
 * original object until all references to it go out of scope:
 */

fn main() {
    let mut _mutable_integer = 7i32;

    {
        // Borrow '_mutable_integer'.
        let large_integer = &_mutable_integer;

        // Error! '_mutable_integer' is frozen in this scope.
        // _mutable_integer = 50;

        println!("Immutably borrowed {}", large_integer);

        // 'large_integer' goes out of scope.
    }

    // Ok! '_mutable_integer' is not frozen in this scope.
    _mutable_integer = 3;
}
