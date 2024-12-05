
// fn main(){
//     let string1 = String::from("Jelly");
//     let string2= String::from("bean!");

//    let concatenated_string = concatenate_strings(&string1, &string2);

//    println!("{}", concatenated_string);
// }

// // conacatenate 2 string slices
// fn concatenate_strings(s:&str, t:&str)-> String{
//     let mut result= String::from(s);
//     result.push_str(t);
//     result
// }



// enum Message{
//     Quit,
//     ChangeColor(i32, i32, i32),
//     Move{x:i32, y:i32},
//     Write(String),
// }

// fn process_message(msg: Message){
//     match msg{
//         Message::Quit => {
//             println!("The Quit variant has no data to destructure.")
//         },
//         Message::ChangeColor(r, g, b) => {
//             println!("Change the color to red {}, green {}, and blue {}", r, g, b);
//         },
//         Message::Move{x, y} => {
//             println!("Move to x: {}, y: {}", x, y);
//         },
//         Message::Write(text) => {
//             println!("Write message: {}", text);
//         },
// }
// }

//It's important to note that if let is best used when 
// you need to check for a single variant.
// If you need to handle multiple enum variants, 
// the match statement is more appropriate.

// enum Animal{
//     Dog(String),
//     Cat(String),
//     Bird(String),
// }

// fn main(){
//     // let msg = Message::Write(String::from("Hello, world!"));
    
//     let my_pet = Animal::Cat("Fluffy".to_string());

//     if let Animal::Cat(name) = my_pet {
//         println!("My Cat's Name is : {}", name);
//     } else{
//         println!("I don't have a cat.");
//     }
//     }


// enum Message{
//     Quit,
//     ChangeColor(u8, u8, u8),
//     Move{x:i32, y:i32},
//     Write(String),
// }

// impl Message {
//     fn call(&self){
//         match self{
//             Message::Quit => {
//                 println!("The Quit variant has no data to destructure.")
//             },
//             Message::ChangeColor(r, g, b) => {
//                 println!("Change the color to red {}, green {}, and blue {}", r, g, b);
//             },
//             Message::Move{x, y} => {
//                 println!("Move to x: {}, y: {}", x, y);
//             },
//             Message::Write(text) => {
//                 println!("Write message: {}", text);
//             },
//         }
//     }
// }

// fn main(){
//     let msg = Message::Write(String::from("Hello, world!"));
//     msg.call();
// }

// fn main() {
// //    enum Option<T>{
// //        Some(T),
// //        None,
// //    }

// let number  = -4.0;
// let result = find_square_root(number);

// //  match result{
// //     Some(value) => println!("The square root of {} is {}", number, value),
// //     None => println!("The square root of {} is not a real number", number),
// //     }

// // enum Result <T, E>{
// //     Ok(T),
// //     Err(E),

// // }

// let a = 10.0;
// let b = 2.0;
// let division_result = divide(a, b);

// match division_result{
//     Ok(value) => println!("The division of {} by {} is {}", a, b, value),
//     Err(message) => println!("Error: {}", message),
//     }

// let base = get_from_database("base");
// let height = get_from_database("height");
// let area_result = calculate_triangle_area(base, height);

// match area_result{
//     Ok(area) => println!("The area of the triangle is {}", area),
//     Err(error_message) => println!("Error: {}", error_message),
//     }

// }

// // fn find_square_root(num: f64) -> Result<f64, String>{

// fn find_square_root(num: f64) -> Option<f64>{
//     if num >= 0.0 {
//         Some(num.sqrt())
//     } else {
//         None
//     }
// }

// fn divide(a: f64, b: f64) -> Result<f64, String>{
//     if b == 0.0 {
//         Err("Cannot divide by zero.".to_string())
//     } else {
//         Ok(a / b)
//     }
// }

// fn get_from_database(key: &str) -> Option<f64>{
//     let database:[(&str, Option<f64>); 2]= [("base", Some(10.0)), ("height", Some(5.0))];

//     for (k,v) in database{
//         if k == key{
//             return v;
//         }
//     }
//     None
// }

// fn calculate_triangle_area(base: Option<f64>, height: Option<f64>) -> Result<f64, String>{
//     match (base, height){
//         (Some(b), Some(h))=>{
//             if b<=0.0 || h<=0.0{
//                 Err("Base and height must be greater than zero.".to_string())
//             } else {
//                 Ok(0.5 * b * h)
//             }
//         }
//         (None, _) => Err("Base is missing.".to_string()),
//         (_, None) => Err("Height is missing.".to_string()),
//     }
// }



// fn main(){

//     let mut numbers  = vec![1, 2, 3, 4, 5];

//     let mut names = Vec::new();

//     names.push(String::from("John")); //[John]

//     names.push(String::from("Doe"));  //[John, Doe]

//     let first_name = &names[0];
//     let second_name = &names[1];

//     println!("First name: {}, Second name is {}", first_name, second_name);

//     names.pop(); // [John]

//     for number in &numbers{
//         println!("{}", number);
//     }

//     let slice  = &numbers[1..3]; // [2, 3]


// // 2 ways if creating strings

// let mut my_string = String::from ("Hello, stos toboi!");
// let mut second_string = "Hello, world!".to_string();

// my_string.push_str(" How are you?");

// println!("{}", my_string);

// for c in my_string.chars(){
//     println!("{}", c);
// }

// for b in my_string.bytes(){
//     println!("{}", b);
// }

// }



//hashmaps

// use std::collections::HashMap;

// fn main (){

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Alice"), 10); //[Alice: 10]
//     scores.insert(String::from("Bob"), 25); //[Alice: 10, Bob: 25]


//     let alice_score = scores.get(&String::from("Alice"));

//     println!("Alice's score is {:?}", alice_score);

//     println!("{:?}", scores);

//     scores.remove(&String::from("Alice"));

//     println!("{:?}", scores);

//     for (key, value) in &scores{
//         println!("{}: {}", key, value);
//     }

// }


// PROJECT  2 CALCULATOR
use std::io;

fn main () {

    //step 4 and 5: Prompt the user to input the first number, 
    //operation, and second number and Parse the user
    // input into appropriate variables.
 
    println!("Enter the first number:");
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).expect("Failed to read input");
    let first_number: f64 = first_input.trim().parse().expect("Please enter a valid number");

    // Prompt the user for the operation
    println!("Enter the operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim();

     // Prompt the user for the second number
     println!("Enter the second number:");
     let mut second_input = String::new();
     io::stdin().read_line(&mut second_input).expect("Failed to read input");
     let second_number: f64 = second_input.trim().parse().expect("Please enter a valid number");

 
    // Step 6: Create an Operation enum instance with the parsed input values.

    let operation_enum = match operation {
        "+" => Operation::Add {
            a: first_number,
            b: second_number,
        },
        "-" => Operation::Subtract {
            a: first_number,
            b: second_number,
        },
        "*" => Operation::Multiply {
            a: first_number,
            b: second_number,
        },
        "/" => {
            if second_number == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            }
            Operation::Divide {
                a: first_number,
                b: second_number,
            }
        }
        _ => {
            println!("Invalid operation. Please enter +, -, *, or /.");
            return;
        }
    };

   // Steps 7 and 8: Call the calculate function and display the result
   let result = calculate(operation_enum);
   println!("The result is: {}", result);

}

// step 1: Define the Operation enum with the appropriate variants and values.
enum Operation{
    Add{a: f64, b: f64},
    Subtract{a: f64, b: f64},
    Multiply{a: f64, b: f64},
    Divide{a: f64, b: f64},
}


// Step 2 and 3: calculate function using pattern matching to perform the appropriate arithmetic operation.

fn calculate (operation: Operation) -> f64{
    match operation{
        Operation::Add{a, b} => a + b,
        Operation::Subtract{a, b} => a - b,
        Operation::Multiply{a, b} => a * b,
        Operation::Divide{a, b} => a / b,
    }
}
