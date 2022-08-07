#[allow(unused_variables)]
use std::io;


// THIS IS A KAKULÁT&OR


fn main(){
    let mut input: String = String::new();
   
    println!("Welcome to RustUlator 1000");
        println!("To continue press ENTER");
        println!("To quit press 'q'");
    
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read");
    while input.trim() != "q" {   
           

        
            //%%%%%%%%%%%%%%%%%%%%%%%%%%// First Number Input
        let mut a1 = String::new();
            
            println!("Please input your first number");

        io::stdin()
            .read_line(&mut a1)
            .expect("failed to read");

                        //if a1.trim() == "q" {break;}

        let a2:f32 = a1.trim().parse().unwrap();



            //%%%%%%%%%%%%%%%%%%%%%%%%%%// Second Number Input
            
        let mut b1 = String::new();

            println!("Please input your second number");

        io::stdin()
                .read_line(&mut b1)
                .expect("failed to read");

                    if b1.trim() == "q" {break;}

        let b2: f32 = b1.trim().parse().unwrap();

            //%%%%%%%%%%%%%%%%%%%%%%%%%%// Operator Input

            let mut operator1 = String::new();

            println!("Please select an operator...");
            println!("Please input the coresponding number!");
            println!("Addition (1), Minusing (2), Multiplication (3), Division (4)");
            
            io::stdin()
                    .read_line(&mut operator1)
                    .expect("failed to read");

                    if operator1.trim() == "q" {break;}

            let operator2 = operator1.trim();


            //%%%%%%%%%%%%%%%%%%%%%%%%%%// Results 



                        let result: f32;

                        if operator2 == "1" {
                            result=a2+b2
                        } else if operator2 == "2" {
                            result=a2-b2
                        } else if operator2 == "3" {
                            result=a2*b2
                        } else if operator2 == "4" {
                            result=a2/b2
                        } else {
                            println!("incorrect operator!, here's wonderwall:");
                            result=69.69
                        };

            println!("The result is...");
            println!("Drumroll...");
            println!("{}", result);
            }

    
}

/*
let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
*/