use std::io;

fn main() {
  println!("Please enter in a variable");

  let mut skibidi: String = String::new();

  std::io::stdin()
    .read_line(&mut skibidi)
    .expect("Failed to read line");

  println!("The user has entered: {}!", skibidi);
}
