mod fi;
mod conversion;
use crate::fi::Parsing;

fn main() {
    let fixed = fi::fi{sign: false, value: vec![false, true, false, true, true, true, false, false, true]};
    let string = "123".to_string();
    println!("{:?}", fixed.bin_bcd().value);
    println!("{:?}", fixed.bin_bcd().bcd_bin());
    println!("{}", fixed.to_string());
    println!("{}", fixed.bin_bcd().bcd_bin().to_string());
    println!("{:?}", string.parse_fi());
    println!("{:?}", string.parse_fi().to_string());
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
}
