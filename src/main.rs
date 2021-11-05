use std::io::{stdin,stdout,Write};

mod keygenerator;
use keygenerator::hash_string;
mod blockchain;
use blockchain::{Block, Printer};

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
  let block: Block = Block::new(
      "home".to_string(), "dest".to_string(), 1337
  );
  let result : String = block.print(); 
  println!("Block:\n{}", result);
  block.print();
}
