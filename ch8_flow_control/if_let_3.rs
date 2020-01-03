/* 
 * Another benefit: if let allows to match enum non-parameterized variants, even 
 * if the enum doesn't #[derive(PartialEq)], neither we implement PartialEq for it. 
 * In such case, classic if Foo::Bar==a fails, because instances of such enum are 
 * not comparable for equality. However, if let works.
 *
 * Would you like a challenge? Fix the following example to use if let:
 */

// This enum purposely doesn't #[derive(PartialEq)],
// neither we implement PartialEq for it. That's why comparing Foo::Bar==a fails below.
enum Foo {Bar}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }
}
