use std::io ;
fn main() 
   { 
    let mut first_str : String = String::new();
    println!("Enter the first number: ");

    io::stdin()
        .read_line(&mut first_str)
        .expect("failed to read");
    let mut second_str = String::new();
    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut second_str)
        .expect("failed to read");
    let first_num: i32 = first_str.trim().parse::<i32>().unwrap();
    let second_num: i32 = second_str.trim().parse::<i32>().unwrap();
    let mut oper : String = String::new();
    println!("What operator would you like to use ");
    println!(" + - *");
    io::stdin()
        .read_line( &mut oper)
        .expect("Failed to read");
    match oper.as_str().trim(){
        "+" => add(first_num, second_num),
        "-" => subtraction(first_num, second_num),
        "*" => mult(first_num, second_num),
        _ => println!("error"),
    }
    
}

fn add(first_num:i32, second_num:i32)
{
        let added = first_num + second_num;
        println!("{}", added);
}
fn mult(first_num:i32, second_num:i32)
{
    let multed: i32 = first_num + second_num;
    println!("{}", multed)

}
fn subtraction(first_num:i32, second_num:i32)
{
    let subtr: i32 = first_num - second_num;
    println!("{}", subtr)
}

