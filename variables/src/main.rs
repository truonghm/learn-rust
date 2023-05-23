fn test_mutable() {

    println!("Test mutable");
    // if we create x like this, the code will not compile and raise an error instead
    // let x = 5;

    // make x mutable instead
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// this code compiles, but generates a warning because mut is not needed
fn test_mutable_shadowing() {
    println!("Test shadowing and mutable");
    let mut x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");
}


// this code compiles without any warnings
fn test_shadowing() {
    println!("Test shadowing");
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");
}

fn test_mutable2() {
    let mut x: u32 = 1;
    {
        let mut x = x;
        x += 2;
    }
    println!("{x}");
}

fn main() {
    // test_mutable();
    // test_mutable_shadowing();
    // test_shadowing();
    test_mutable2();
}