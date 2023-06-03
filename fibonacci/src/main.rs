use std::io;

fn main() {

    // let first = 1;
    let mut second = 1;
    let mut third = 2;

    let mut idx = String::new();

    println!("Provide the index of the fibonacci number (starting from 1):");

    io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");
    let idx: i32 = match idx.trim().parse() {
        Ok(idx) => idx,
        Err(_) => panic!("Bad input for number!")
    };

    if idx == 1 {
        println!("The 1st fibonacci number is 1")
    } else if idx == 2 {
        println!("The 2nd fibonacci number is 1")
    } else if idx == 3 {
        println!("The 3rd fibonacci number is 2")
    } else {
        let mut counter = 3;
        while counter < idx {
            let tmp = third;
            third = second + third;
            second = tmp;
            counter += 1;
        }
        println!("the {idx}th fibonacci number is {third}")
    }
}
