use std::io;

fn main(){
  println!("Enter something");
  let mut input_string = String::new();
  io::stdin().read_line(&mut input_string).expect("Failed to read input");
  let integer_input:i32 = input_string.trim().parse().unwrap();//converting string into mentioned datatype 
  println!("You entered {}",integer_input);
}
