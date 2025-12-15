mod fi;
mod errors;
mod conversion;
use crate::fi::Parsing;
// use crate::conversion::int::ParseInt;
use crate::conversion::int::ParseIntGeneric;
mod functions;
use crate::fi::Fi as fixed_int;

fn main() {
    // let fixed = fi::fi{sign: false, value: vec![false, true, false, true, true, true, false, false, true]};
    // let string = "109019".to_string();
    let n1: fixed_int = (43 as i8).into();
    let n2: fixed_int = (2 as i8).into();
    let dividend: fixed_int = String::from("2.000000000000000001").parse_fi();
    let divisor: fixed_int = fixed_int{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]};
    let calc = dividend.clone() / divisor.clone();
    let f1: fixed_int = fixed_int{sign: false, value: vec![true, true, true]};
    let f2: fixed_int = ((i8::MAX as i16 + 12) as i16).into();
    let res = f2.parse::<i8>().unwrap();
    // println!("{:?}", fixed.bin_bcd().value);
    // println!("{:?}", fixed.bin_bcd().bcd_bin());
    // println!("{}", fixed.to_string());
    // println!("{}", fixed.bin_bcd().bcd_bin().to_string());
    // println!("{:?}", string.parse_fi());
    // println!("{:?}", string.parse_fi().to_string());
    // println!("{:?}", string.parse_bcd().to_string());
    // println!("{:?}", string.parse_bcd().to_string());
    println!("{:?}", n1.to_string());
    println!("{:?}", n2.to_string());
    println!("{:?}", calc.to_string());
    println!("{}", (f1 / f2).to_string());
    println!("{res}");
    // println!("{}", n1 < n2);
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
}


#[cfg(test)]
mod test;
