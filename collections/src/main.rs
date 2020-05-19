fn main() {
    // VECTORS
    // stores data on heap

    // Type annotation needed since Vectors are implemented using generics and
    // cannot infer type when there is no value inserted in
    let v: Vec<i32> = Vec::new();

    // Otherwise, could initialize vector with initial values
    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    // When a vector goes out of scope, all its contents are also dropped
    
    // Reading elements of Vectors
    // Method 1: get reference using & and []
    // Program will panic if index is out of bound
    let third: &i32 = &v[2]; // immutable borrow, cannot push new value to the vector (modify) right after this line
    println!("The third element is {}", third);

    // Method 2: get Option<&T> using .get()
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // Iterating over values in a Vector
    // Immutable references
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // Mutable references
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", *i);
    }

    // Storing multiple types using enums (only work if know all possible types at compile time)
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    // STRINGS

    // Initialise empty String
    let mut s = String::new();

    // Initialise String from types that implement Display trait (e.g. string literals)
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // Appending to String
    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str takes in a str slice (reference) and doesn't take ownership
    let s2 = "foo";
    s.push_str(s2);

    // push appends a single character to String
    s.push('l');

    // Concatenate 2 Strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;
    // s1 is moved here (owned) and can no longer be used, but s2 is only borrowed and can still be used afterwards
    // "+" uses add(self, s: &str) -> String, but can still add &s2: &str, because Rust can coerce &String into &str (s2[..])
    
    // Use format! for more complicated (mulitple) string concatenation
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    // format! does not take ownership of any parameter
    
    // Rust does not support indexing Strings, instead, slicing Strings
    let slice = &s[0..4]; // a &str that contains the first 4 bytes of the String
    // If the language in UTF-8 is such that each valid character is 2 bytes, trying to slice &s[0..1] will cause Rust to panic
    
    // Iterating over Strings
    // On individual Unicode scalar values
    for c in s.chars() {
        println!("{}", c);
    }

    // On individual bytes
    for b in s.bytes() {
        println!("{}", b);
    }

    // HASHMAPS

    use std::collections::HashMap; // not brought into scope automatically in the prelude
    // stores data on heap

    // Initialise empty HashMap and insert
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Initialise by collecting from a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // For types that implement Copy trait (e.g. i32), values are copied into HashMap
    // For owned values (e.g. String), values will be moved and owned by HashMap
    
    // Accessing values in a HashMap
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // use borrowed K and return Option<&V>, in this case Some(&10)

    // Iterating key/value pair
    for (key, value) in &scores {
        println!("{} : {}", key, value);
    }

    // Overwriting an old value when inserting a new value to an existing key
    scores.insert(String::from("Blue"), 25); // the score will be updated from 10 to 25
    
    // Only inserting if the key has no value
    scores.entry(String::from("Blue")).or_insert(50);
    // .entry return an enum Entry that represents a value that might (not) exist. If doesn't exist, then insert
    // .or_insert returns a mutable reference to the value
    
    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // return reference to current count of word (if new, return 0)
        *count += 1; // adds 1 to the count (must deref), mutable reference goes out of scope at the end of the for loop
    }

    println!("{:?}", map); // return key = word and value = count
}
