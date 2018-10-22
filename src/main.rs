//This project was inspired by https://github.com/jkusner/CACBarcode/blob/master/cacbarcode.py

extern crate base_custom; use base_custom::BaseCustom;

extern crate chrono; use chrono::prelude::*;
extern crate time; use time::Duration;

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
  match data.len() {
    18 => return decode_code39(data),
    88 | 89 => return decode_pdf217(data),
    _ => return format!("Incorrect barcode length: {}. Make sure to include all spaces.", data.len()),
  }
}

fn decode_pdf217(data: String) -> String {
  let mut out = Vec::new(); //(Key, Value)
  out.push(("Barcode type", "PDF217".to_string()));

  let base32 = BaseCustom::<String>::new("0123456789ABCDEFGHIJKLMNOPQRSTUV", None);
  let base_time = Utc.ymd(1000, 1, 1);
  
  let mut data_chars = data.chars();

  //Version
  let version = data_chars.next().unwrap();
  match version {
    '1' | 'N' => out.push(("Barcode version", version.to_string())),
    _ => return format!("Unknown barcode version {}", version),
  }
  
  //Personal Designator Identifier (Base 32)
  let pdi = data_chars.by_ref().take(6).collect::<String>();
  out.push(("Personal Designator Identifier", base32.decimal(pdi).to_string()));
  
  //Personal Designator Type
  out.push(("Personal Designator Type", lookup_pdt(data_chars.next().unwrap())));

  //Electronic Data Interchange Person Identifier (base 32)
  let edipi = data_chars.by_ref().take(7).collect::<String>();
  out.push(("Electronic Data Interchange Person Identifier", base32.decimal(edipi).to_string()));

  //First Name
  out.push(("First Name", data_chars.by_ref().take(20).collect::<String>()));

  //Last Name
  out.push(("Last Name", data_chars.by_ref().take(26).collect::<String>()));

  //Date of Birth
  let days = data_chars.by_ref().take(4).collect::<String>().parse::<i64>();
  out.push(("Date of Birth", (base_time + Duration::days(days)).format("%a, %e %b %Y")));
  
  //Personnel Category Code
  out.push(("Personnel Category Code", loop_ppc(data_chars.next().unwrap())));
  
  //Branch
  out.push(("Branch", lookup_branch(data_chars.next().unwrap())));
  
  //Personnel Entitlement Condition Type
  let pect = (data_chars.next().unwrap(), data_chars.next().unwrap())
  out.push(("Personnel Entitlement Condition Type", lookup_pect()));

  //Rank
  out.push(("Rank", data_chars.by_ref().take(6).collect::<String>())); 

  //Pay Plan Code
  out.push(("Pay Plan Code", data_chars.by_ref().take(2).collect::<String>())); 

  //Pay Plan Grade Code
  out.push(("Pay Plan Grade Code", data_chars.by_ref().take(2).collect::<String>())); 

  //Card Issue Date
  let days = data_chars.by_ref().take(4).collect::<String>().parse::<i64>();
  out.push(("Card Issue Date", (base_time + Duration::days(days)).format("%a, %e %b %Y")));
  
  //Card Expiration Date
  let days = data_chars.by_ref().take(4).collect::<String>().parse::<i64>();
  out.push(("Card Expiration Date", (base_time + Duration::days(days)).format("%a, %e %b %Y")));
  
  //Card Instance Identifier (Random)
  out.push(("Card Instance Identifier (Random)", data_chars.next().unwrap().to_string()));

  if data.len() == 89 {
    //Middle Initial
    let initial = data_chars.next().unwrap();
    out.push(("Middle Initial", initial.to_string()));
  }
  
  out.iter().map(|(key, val)| format!("{}: {}\n", key, val)).collect::<String>()
}
    
fn decode_code39(data: String) -> String {
  let mut out = Vec::new(); //(Key, Value)
  out.push(("Barcode type", "Code39".to_string()));
  
  //Version
  let version = data_chars.next().unwrap();
  match version {
    '1' => out.push(("Barcode version", version.to_string())),
    _ => return format!("Unknown barcode version {}", version),
  }

  //Personal Designator Identifier (Base 32)
  let pdi = data_chars.by_ref().take(6).collect::<String>();
  out.push(("Personal Designator Identifier", base32.decimal(pdi).to_string()));
  
  //Personal Designator Type
  out.push(("Personal Designator Type", lookup_pdt(data_chars.next().unwrap())));

  //Electronic Data Interchange Person Identifier (base 32)
  let edipi = data_chars.by_ref().take(7).collect::<String>();
  out.push(("Electronic Data Interchange Person Identifier", base32.decimal(edipi).to_string()));

  //Personnel Category Code
  out.push(("Personnel Category Code", loop_ppc(data_chars.next().unwrap())));
  
  //Branch
  out.push(("Branch", lookup_branch(data_chars.next().unwrap())));
  
  //Card Instance Identifier (Random)
  out.push(("Card Instance Identifier (Random)", data_chars.next().unwrap().to_string()));
  
  out.iter().map(|(key, val)| format!("{}: {}\n", key, val)).collect::<String>()
}
    
fn lookup_ppt(ppt: char) -> String {
  match pdt {
    'S' => "Social Security Number (SSN)".to_string(),
    'N' => "9 digits, not valid SSN".to_string(),
    'P' => "Special code before SSNs".to_string(),
    'D' => "Temporary Identifier Number (TIN)".to_string(),
    'F' => "Foreign Identifier Number (FIN)".to_string(),
    'T' => "Test (858 series)".to_string(),
    'I' => "Individual Taxpayer Identification Number".to_string(),
    _ => format!("Unknown Type {}", pdt),
  }
}
    
fn lookup_ppc(ppc: char) -> String {
  match ppc {
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
  }
}

fn lookup_branch(branch: char) -> String {
  match branch {
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
  }
}

fn lookup_pect(pect: (char, char)) -> String {
  match pect {
    ('0', '1') => "On Active Duty. Segment condition.",
    ('0', '2') => "Mobilization. Segment condition.",
    ('0', '3') => "On appellate leave. Segment condition.",
    ('0', '4') => "Military prisoner. Segment condition.",
    ('0', '5') => "POW/MIA. Segment condition.",
    ('0', '6') => "Separated from Selected Reserve. Event condition.",
    ('0', '7') => "Declared permanently disabled after temporary disability period. Event condition.",
    ('0', '8') => "On non-CONUS assignment. Segment condition.",
    ('0', '9') => "Living in Guam or Puerto Rico. Segment condition.",
    ('1', '0') => "Living in government quarters. Segment condition.",
    ('1', '1') => "Death determined to be related to an injury, illness, or disease while on Active duty for training or while traveling to or from a place of duty. Event condition.",
    ('1', '2') => "Discharged due to misconduct involving family member abuse. (Sponsors who are eligible for retirement.) Segment condition.",
    ('1', '3') => "Granted retired pay. Event condition.",
    ('1', '4') => "DoD sponsored in U.S. (foreign military). Segment condition.",
    ('1', '5') => "DoD non-sponsored in U.S. (foreign military). Segment condition.",
    ('1', '6') => "DoD sponsored overseas. Segment condition.",
    ('1', '7') => "Deserter. Segment condition.",
    ('1', '8') => "Discharged due to misconduct involving family member abuse. (Sponsors who are not eligible for retirement.) Segment condition.",
    ('1', '9') => "Reservist who dies after receiving their 20 year letter. Event condition.",
    ('2', '0') => "Transitional assistance (TA-30). Segment condition.",
    ('2', '1') => "Transitional assistance (TA-Res). Segment condition.",
    ('2', '2') => "Transitional assistance (TA-60). Segment condition.",
    ('2', '3') => "Transitional assistance (TA-120). Segment condition.",
    ('2', '4') => "Transitional assistance (SSB program). Segment condition.",
    ('2', '5') => "Transitional assistance (VSI program). Segment condition.",
    ('2', '6') => "Transitional assistance (composite). Segment condition.",
    ('2', '7') => "Senior Executive Service (SES).",
    ('2', '8') => "Emergency Essential - overseas only.",
    ('2', '9') => "Emergency Essential - CONUS.",
    ('3', '0') => "Emergency Essential - CONUS in living quarters, living on base, and not drawing a basic allowance for quarters, serving in an emergency essential capacity.",
    ('3', '1') => "Reserve Component TA-120 Reserve Component Transition Assistance TA 120 (Jan 1, 2002 or later).",
    ('3', '2') => "On MSC owned and operated vessels Deployed to foreign countries on Military Sealift Command owned and operated vessels. Segment condition.",
    ('3', '3') => "Guard/Reserve Alert Notification Period.",
    ('3', '4') | ('3', '5') => "Reserve Component TA-180 - 180 days TAMPS for reserve return from named contingencies.",
    ('3', '6') | ('3', '7') => "TA-180 - 180 days TAMP for involuntary separation.",
    ('3', '8') => "Living in Government Quarters in Guam or Puerto Rico, Living on base and not drawing an allowance for quarters in Guam or Puerto Rico.",
    ('3', '9') => "Reserve Component TA-180 - TAMP - Mobilized for Contingency.",
    ('4', '0') => "TA-180 TAMP - SPD Code Separation.",
    ('4', '1') => "TA-180 - TAMP - Stop/Loss Separation.",
    ('4', '2') => "DoD Non-Sponsored Overseas - Foreign Military personnel serving OCONUS not sponsored by DoD.",
    _ => format!("Unknown Type {}", pect),
  }
}
