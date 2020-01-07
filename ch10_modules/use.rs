/*
 * The use declaration can be used to bind a full path to a new name, for easier access. 
 * It is often used like this:
 */

 // extern crate deeply; // normally, this would exist and not be commented out!
 
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn main () {
    my_first_function();
}
