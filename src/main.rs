use std::io::stdin;
fn main()  {
    // code here
    let mut input:String=String::new();
    println!("Hello there -.- Input a number to convert it to binary\n");
    stdin().read_line(&mut input).unwrap();
    let mut num:i64 = input.trim().parse().unwrap_or_else(|_err|panic!("Could not convert {} to int",input));
    let mut holder:String = String::new();
    loop{
        if  num % 2 == 1{
            holder.push_str("1");
        }
        else{
            holder.push_str("0");
        }
        num=num/2;
        if num==0{
            break;
        }
    }
    // Reverse number output
    // Check https://stackoverflow.com/questions/27996430/reversing-a-string-in-rust
    println!("There you go: {}",holder.chars().rev().collect::<String>());
}