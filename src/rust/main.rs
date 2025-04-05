use std::io;

fn main() {
  println!("Please enter in a variable");

  let mut input: String = String::new();

  std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

  println!("The user has entered: {}!", input);

  if (input == "input") {
    println!("The user has entered in input");
  } else {
    println!("The user has not entered in input!");
  }
}
