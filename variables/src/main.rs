fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUB_COUNT: u32 = 100_000; // constants need static declaration of type unlike let keyword
    println!("The value of SUB_COUNT is: {}", SUB_COUNT);

    let x = 5;
    println!("The value of x is: {}", x);
    let x = "abc"; // shadowing
    println!("The value of x is: {}", x);

    // scaler data types and compound data types
    // scaler data types contains single value, while compound data types contains group of values.
    // scaler data types: integer, floating-point, boolean, character
    // integer types
    //  signed: i8, i16, i32, i64, i128, isize
    //  unsigned: u8, u16, u32, u64, u128, usize

    let a = 98_222; // Decimal
    let b = 0xff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte (u8 only)
    let f = 'A'; // String (u8 only)
    let g: u8 = 255;

    // floating-point types: f32, f64
    let h = 2.0;
    let i: f32 = 3.0;

    // boolean type: bool
    let j = true;
    let k = false;

    // character type: char
    let l = 'a';
    let m = '😻';

    // arithmatic operation
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    // shift
    let shift = 1 << 10;
    // bitwise AND
    let bitwise_and = 0b1010_1100 & 0b1000_1100;
    // bitwise OR
    let bitwise_or = 0b1010_1100 | 0b1000_1100;
    // bitwise XOR
    let bitwise_xor = 0b1010_1100 ^ 0b1000_1100;
    // bitwise NOT
    let bitwise_not = !0b0000_0000;
    // logical AND
    let logical_and = true && false;
    // logical OR
    let logical_or = true || false;
    // logical NOT
    let logical_not = !true;
    // conditional AND
    let conditional_and = 5 > 3 && 5 < 10;
    // conditional OR
    let conditional_or = 5 > 3 || 5 < 10;
    // conditional NOT
    let conditional_not = !(5 > 3);
    // logical shift
    let logical_shift = 1 << 10;
    // arithmetic shift
    let arithmetic_shift = -1 >> 10;
    // bitwise shift
    let bitwise_shift = 1 >> 10;

    // compound data types: tuple, array
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);
    // array
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];
    let third = a[2];
    let fourth = a[3];
    let fifth = a[4];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    println!("The value of third is: {}", third);
    println!("The value of fourth is: {}", fourth);
    println!("The value of fifth is: {}", fifth);
    // let index = 10;
    // let element = a[index];
    // println!("The value of element is: {}", element);
    // println!("The value of index is: {}", index);
    let sum = my_fn(5, 6);
    println!("The value of sum is: {}", sum);

    // Control flow
    // if condition
    let number = 5;
    if number < 5 {
        println!("condition was true");
    } else if number < 22 {
        println!("condition was false");
    } else {
        println!("condition was false");
    }

    // for loop
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("The value of res is: {}", res);

    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!");

    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn my_fn(x: i32, y: i32) -> i32 {
    println!("just another function");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // let sum = x + y; // implicit return
    // return sum; // implicit return
    x + y
}
