//This project was inspired by https://github.com/jkusner/CACBarcode/blob/master/cacbarcode.py

extern crate base_custom;
use base_custom::BaseCustom;
/*
fn main() {
  println!("{}", base32.decimal("TONMLL"));
}*/

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

fn decode(data: String) -> String {

  let mut out = Vec::new(); //(Key, Value)

  //PDF217
  match data.len() {
    88 | 89 => out.push(("Barcode type", "PDF217".to_string())),
    _ => return format!("Incorrect barcode length: {}. Make sure to include all spaces.", data.len()),
  }

  let base32 = BaseCustom::<String>::new("0123456789ABCDEFGHIJKLMNOPQRSTUV", None);
  
  let mut data_chars = data.chars();

  //Version
  let version = data_chars.next().unwrap();
  match version {
    '1' | 'N' => out.push(("Barcode version", version.to_string())),
    _ => return format!("Unknown barcode version {}", version),
  }
  
  //Personal Designator Identifier
  let pdi = data_chars.by_ref().take(6).collect::<String>();
  out.push(("Personal Designator Identifier", base32.decimal(pdi).to_string()));
  
  //Personal Designator Type
  let pdt = data_chars.next().unwrap();
  out.push(("Personal Designator Type", match pdt {
    'S' => "Social Security Number (SSN)".to_string(),
    'N' => "9 digits, not valid SSN".to_string(),
    'P' => "Special code before SSNs".to_string(),
    'D' => "Temporary Identifier Number (TIN)".to_string(),
    'F' => "Foreign Identifier Number (FIN)".to_string(),
    'T' => "Test (858 series)".to_string(),
    'I' => "Individual Taxpayer Identification Number".to_string(),
    _ => format!("Unknown Type {}", pdt),
  }));

  //Electronic Data Interchange Person Identifier
  let edipi = data_chars.by_ref().take(7).collect::<String>();
  out.push(("Electronic Data Interchange Person Identifier", base32.decimal(edipi).to_string()));

  //First Name
  out.push(("First Name", data_chars.by_ref().take(20).collect::<String>()));

  //Last Name
  out.push(("Last Name", data_chars.by_ref().take(26).collect::<String>()));

  //Read Date
  let _date = data_chars.by_ref().take(4).collect::<String>();
  
  //Personnel Category Code
  let pcc = data_chars.next().unwrap();
  out.push(("Personnel Category Code", match pcc {
    'A' => "Active Duty member".to_string(),
    'B' => "Presidential Appointee".to_string(),
    'C' => "DoD civil service employee".to_string(),
    'D' => "100% disabled American veteran".to_string(),
    'E' => "DoD contract employee".to_string(),
    'F' => "Former member".to_string(),
    'N' | 'G' => "National Guard member".to_string(),
    'H' => "Medal of Honor recipient".to_string(),
    'I' => "Non-DoD Civil Service Employee".to_string(),
    'J' => "Academy student".to_string(),
    'K' => "non-appropriated fund (NAF) DoD employee".to_string(),
    'L' => "Lighthouse service".to_string(),
    'M' => "Non-Government agency personnel".to_string(),
    'O' => "Non-DoD contract employee".to_string(),
    'Q' => "Reserve retiree not yet eligible for retired pay".to_string(),
    'R' => "Retired Uniformed Service member eligible for retired pay".to_string(),
    'V' | 'S' => "Reserve member".to_string(),
    'T' => "Foreign military member".to_string(),
    'U' => "Foreign national employee".to_string(),
    'W' => "DoD Beneficiary".to_string(),
    'Y' => "Retired DoD Civil Service Employees".to_string(),
    _ => format!("Unknown Type {}", pcc),
  }));
  
  //Branch
  let branch = data_chars.next().unwrap();
  out.push(("Branch", match branch {
  'A' => "USA".to_string(),
  'C' => "USCG".to_string(),
  'D' => "DOD".to_string(),
  'F' => "USAF".to_string(),
  'H' => "USPHS".to_string(),
  'M' => "USMC".to_string(),
  'N' => "USN".to_string(),
  'O' => "NOAA".to_string(),
  '1' => "Foreign Army".to_string(),
  '2' => "Foreign Navy".to_string(),
  '3' => "Foreign Marine Corps".to_string(),
  '4' => "Foreign Air Force".to_string(),
  'X' => "Other".to_string(),
  _ => format!("Unknown Type {}", pcc),
  }));
  
  //Personnel Entitlement Condition Type
  let pect = (data_chars.next().unwrap(), data_chars.next().unwrap())
  out.push(("Personnel Entitlement Condition Type", match pect {
  ('0', '1') => "On Active Duty. Segment condition.",
  _ => format!("Unknown Type {}", pect),
  }));

  out.iter().map(|(key, val)| format!("{}: {}\n", key, val)).collect::<String>()
  
}