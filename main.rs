
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

// Warning: unused variable: `x`
