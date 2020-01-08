/* 
 * The compiler provides a dead_code lint that will warn about unused 
 * functions. An attribute can be used to disable the lint.
 */

fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

//#[allow(dead_code)]
fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
}

/* Note that in real programs, you should eliminate dead code. In these 
 * examples we'll allow dead code in some places because of the interactive 
 * nature of the examples. */
