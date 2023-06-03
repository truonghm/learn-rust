use std::io;

fn main() {

    println!("Please select conversion type.");
    println!("1 for Fahrenheit to Celsius, 2 for Celsius to Fahrenheit:");

    let mut conversion = String::new();

    io:: stdin()
        .read_line(&mut conversion)
        .expect("Failed to read line");

    conversion = conversion.trim().to_string();

    if conversion != "1" && conversion != "2" {
        panic!("Invalid conversion input, must be 1 or 2.")
    }

    println!("Please provide input for conversion.");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: f32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Bad input for number!")
    };

    if conversion == "1" {
        let temp = fahrenheit_to_celsius(num);
        println!("The temperature in Celsius is {temp}")
    } else {
        let temp = celsius_to_fahrenheit(num);
        println!("The temperature in Fahrenheit is {temp}")
    }

}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    let c = (f - 32.) * 5./9. ;
    return c
}

fn celsius_to_fahrenheit(c: f32) -> f32 {
    let f = c * 9./5. + 32. ;
    return f
}