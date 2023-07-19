
// Fix the error below with least amount of modification to the code
fn main() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

// Fill the blanks in the code to make it compile
fn main() {
    let mut x:i32 = 1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}


// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        
        println!("The value of x is {} and value of y is {}", x, y);
    } 
    println!("The value of x is {} and value of y is {}", x, y);
}



// Fix the error below with least amount of modification
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        
        println!("The value of x is {} and value of y is {}", x, y);
    } 
    println!("The value of x is {} and value of y is {}", x, y);
}



// Fix the error with the use of define_x
fn main() {
    define_x();
}

fn define_x() {
    let x: &str = "hello";
    println!("{}, world", x); 
}



// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x:i32 = 42;
    println!("{}", x); // Prints "42".
}



// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7; //x = 7
    // Shadowing and re-binding
    x += 3; // x = 10 (7 + 3)


    let  y: i32 = 4;
    // Shadowing
    let y: &str = "I can also be bound to text!"; 

    println!("Success!");
}



fn main() {
    let _x = 1;
}

// Numbers in rust



// Remove something to make it work
fn main() {
    let x: i32 = 5;
    let mut y: i32 = 5;

    y = x;
    
    let z:i32 = 10; // Type of z ? 

    println!("Success!");
}



// Fill the blank
fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}



// Modify `assert_eq!` to make it work
fn main() {
    let x:i32 = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}



// Fill the blanks to make it work
fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}


// Fill the blank to make it work
fn main() {
    let x:f64 = 1_000.000_1; // ?
    let y: f32 = 0.12; // f32
    let z:f64 = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}


// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1u32 + 2u32 == 3u32);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2i8 == -1i8); 
    
    assert!(3 * 50 == 150);

    assert!(9.6 as f32 / 3.2 as f32 == 3.0 as f32); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
