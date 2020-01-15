/*
 * There are multiple ways to write string literals with special characters in them. All result in 
 * a similar &str so it's best to use the form that is the most convenient to write. Similarly there
 * are multiple ways to write byte string literals, which all result in &[u8; N].
 *
 * Generally special characters are escaped with a backslash character: \. This way you can add any
 * character to your string, even unprintable ones and ones that you don't know how to type. If you
 * want a literal backslash, escape it with another one: \\
 *
 * String or character literal delimiters occuring within a literal must be escaped: "\"", '\''.
 */

fn main() {
    // You can use escapes to write bytes by their hexadecimal values...
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );


    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);
}
