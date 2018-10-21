//This project was inspired by https://github.com/jkusner/CACBarcode/blob/master/cacbarcode.py

fn main() {
  if std::env::args().count() > 1 {
    println!("For security, the barcodes should only be passed via stdin, not as arguments.");
    std::process::exit(1);
  }

  println!("Common Access Cards have two barcodes.");
  println!("One the front (PDF417), and one the back (Code39).");
  println!("Get an application that can read a PDF417 barcode.");
  println!("Copy and paste it into here, and I will decode it.");
  println!("The decoded info will only be presented here, and will not be saved.");
  println!();
  
  use std::io::prelude::*;
  let stdin = std::io::stdin();
  for line in stdin.lock().lines() {
    println!("{}", decode(line.unwrap()));
  }
}

fn decode(line: String) -> String {
  //PDF217
  "test".to_string()
}