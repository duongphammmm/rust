#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("Hello, world!");
    
    // Can change type by providing a suffix
    
    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}", subject = "the quick brown fox", verb = "jump over", object = "the lazy dog");

    // Cannot put position arguments after named arguments
    // println!("Named argument {name} is before positional arguments {1}", name = 1, 2);
    // Cannot provide extra named/positional arguments that are not used as well.

    // Special formatting
    println!("{} of {:b} know binary, the other half doesn't.", 1, 2);

    // Left-aligned
    println!("Hello {:5}!", "x");
    println!("Hello {:<5}!", "x");
    println!("Hello {:1$}!", "x", 5); // postfix $ indicates the usize argument specifying width
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);

    // Right-aligned
    println!("{number:>width$}", number = 1, width = 6);

    //Centre-aligned
    println!("Hello {:^5}!", "x");

    // Padding
    println!("Hello {:<-5}!", "x"); 
    println!("{number:>0width$}", number = 1, width = 6);
    println!("{number:04}", number = 1);

    // Sign/#
    println!("Hello {:+}!", 5); // signed
    println!("Hello {:05}!", -5);

    println!("{:#x}!", 27); // prefix = 0x, similarly, #X prefix 0X, #b prefix 0b, #o prefix 0o, #? prints Debug
    println!("{:#010x}!", 27);

    // Formatting for more complicated structure (e.g. struct)
    // fmt::Debug using {:?} (need to put #[derive(Debug) at the start of the struct)
    // fmt::Display using {} (need to implement fmt::Display trait, hence ToString trait as well.
    
    // Pretty print with Debug
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};
    println!("{:#?}", peter);

    // Precision
    println!("Hello {0} is {1:.5}", "x", 0.01);
    println!("Hello {1} is {2:.0$}", 5, "x", 0.01);
    println!("Hello {} is {:.*}", "x", 5, 0.01);
    println!("Hello, {name:.*} has 3 fractional digits", 3, name = 1234.56);
    println!("Hello, {name:.*} has 3 characters", 3, name = "1234.56");
    println!("Hello, {name:>8.*} has 3 right-aligned characters in width 8", 3, name = "1234.56");

    // Escaping
    println!("Hello {{}}");

    //ACTIVITIES
    println!("My name is {0}, {1} {0}", "Bond", "James");

    println!("Pi is roughly {pi:.*}", 3, pi = 3.141592);

}
