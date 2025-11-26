mod fi;
mod conversion;
use crate::fi::Parsing;
mod functions;

fn main() {
    let fixed = fi::fi{sign: false, value: vec![false, true, false, true, true, true, false, false, true]};
    let string = "109019".to_string();
    let n1 = "10".to_string();
    let n2 = "7".to_string();
    let calc = n1.parse_fi().add(n2.parse_fi());
    // println!("{:?}", fixed.bin_bcd().value);
    // println!("{:?}", fixed.bin_bcd().bcd_bin());
    // println!("{}", fixed.to_string());
    // println!("{}", fixed.bin_bcd().bcd_bin().to_string());
    println!("{:?}", string.parse_fi());
    println!("{:?}", string.parse_fi().to_string());
    println!("{:?}", string.parse_bcd().to_string());
    println!("{:?}", string.parse_bcd().to_string());
    println!("{:?}", n1.parse_bcd().to_string());
    println!("{:?}", n2.parse_bcd().to_string());
    println!("{:?}", calc.bin_bcd());
    println!("{:?}", calc.to_string());
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
}
