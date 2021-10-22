use std::io::{stdin,stdout,Write};

mod keygenerator;
use keygenerator::hash_test;

fn prompt(prompt_text: &str) -> String {
  let mut s = String::new();
  print!("{}", prompt_text);
  let _=stdout().flush();
  stdin().read_line(&mut s).expect("Sorry, can't read that.");
  if let Some('\n')=s.chars().next_back() {
    s.pop();
  }
  if let Some('\r')=s.chars().next_back() {
    s.pop();
  }
  return s;
}

fn main() {
  let hash_me = prompt("Enter what I should hash:\n");
  let result : String = hash_test(&hash_me); 
  println!("Hash: {}", result);
}
