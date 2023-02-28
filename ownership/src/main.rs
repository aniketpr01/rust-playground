fn main() {
    // -- Ownership rules --
    // 1. Each value in rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // {
    //     // t is not valid here, as it's not yet declared
    //     let t = String::from("Hello"); // t is valid from this point forward
    //                                    // do stuff with t
    // } // this scope is now over, and t is no longer valid

    // let x = 5;
    // let y = x; // Copy of X

    // let s1 = String::from("hey");
    // let s2 = s1; // Move of s1 to s2(does not shallow copy), s1 no longer available
    // let s3 = s2.clone(); // clone does the copy

    // Rust will copy the elements from integer, character and boolean but for any other data type, it will move the value as it's default behavior

    // -- Ownership and functions --
    // let s = String::from("hello"); // s comes into scope
    // takes_ownership(s); // s's value moves into the function...
    //                     // ... and so is no longer valid here
    // // println!("{}", s); // s no longer valid as its moving the data to function similar to variables

    // let y = 5;
    // creates_copy(y);
    // println!("{}", y); // y is still valid as it's copying the data to function as it's still an integer

    // References are hence used for not dealing with moving of data
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // Passing the values as references is called borrowing, and not taking ownership of it
    // References are immutable

    // there is a way to change value without taking ownership
    let mut t = String::from("hello");
    change(&mut t);

    // mutable references can only be used once for a variable, effectively saving race condition
    // can't borrow a reference as mutable if it is first borrowed by immutable
    /*
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
     */
    // There can be unlimited immutable references as data will never change

    // -- Dangling references --
    // let reference_to_nothing = dangle();
    // what happens when we point to reference which has invalid data

    // Slices
    let mut s = String::from("hello world");
    let s2 = "hello world";
    let word = first_word(s2);
    // s.clear(); // This empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    // println!("{}", word);

    // -- String slices --
    let slice: &[i32] = &a[0..2];
}

// fn takes_ownership(some_string: String) {
//     // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn creates_copy(some_integer: i32) {
//     // some_integer comes into scope
//     println!("{}", some_integer);
// }

fn calculate_length(s: &String) -> usize {
    let length = s.len(); // len() returns the length of the string
    length
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // prevented by rust as its memory unsafe
//     let s = String::from("hello");
//     &s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
