mod fi;
mod errors;
mod conversion;
use crate::fi::Parsing;
// use crate::conversion::int::ParseInt;
mod functions;
use crate::fi::{FiBin, FiLong};
use std::time::Instant;

fn main() {
    // let fixed = fi::fi{sign: false, value: vec![false, true, false, true, true, true, false, false, true]};
    // let string = "109019".to_string();
    let n1: FiBin = FiBin::from(String::from("14"));
    let n2: FiBin = FiBin::from(String::from("7"));
    let dividend: FiBin = FiBin::from(String::from("2.4999999999999999999"));
    let divisor: FiBin = FiBin{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]};
    let calc = dividend.clone() / divisor.clone();
    // let f1: FiBin = FiBin{sign: false, value: vec![true, true, true]};
    // let f2: FiBin = ((i8::MAX as i16 + 12) as i16).into();
    // let res = f2.parse::<i8>().unwrap();
    // println!("{:?}", fixed.bin_bcd().value);
    // println!("{:?}", fixed.bin_bcd().bcd_bin());
    // println!("{}", fixed.to_string());
    // println!("{}", fixed.bin_bcd().bcd_bin().to_string());
    // println!("{:?}", string.parse_fi());
    // println!("{:?}", string.parse_fi().to_string());
    // println!("{:?}", string.parse_bcd().to_string());
    // println!("{:?}", string.parse_bcd().to_string());
    let time = Instant::now();
    let new = n1.to_long();
    println!("conv: {:?}", time.elapsed());
    let time = Instant::now();
    new.to_bin();
    println!("reconvv: {:?}", time.elapsed());
    let new1 = n1.to_long();
    let mut new2 = n2.to_long();
    println!("{:?}", n1.to_long().to_bin().to_string());
    println!("{:?}", (n1.clone() / FiBin::decimals()).to_long() << 2);
    println!("{:?}", FiBin::decimals().to_long());
    println!("{:?}, {:?}", new1, new2);
    println!("sub: {:?}", (new1 * new2.clone()).to_bin().to_string());
    println!("correct: {:?}", (n1 * FiBin::decimals()).to_long());
    println!("dec: {:?}", FiLong::decimals().to_bin().to_string());
    println!("dec: {:?}", FiBin::from(1).to_long());
    println!("98: {:?}", (FiBin::from(98) * FiBin::decimals()).to_long());
    println!("{:?}", new2);
    new2 >>= 2;
    println!("{:?}", new);
    // println!("{:?}", n1.nth_root(n2.clone()).to_string());
    functions::arithm::time_comparison(FiBin::from(String::from("0897563456342")), FiBin::from(String::from("1238975634532")));
    // println!("{}", (f1 / f2).to_string());
    // println!("{res}");
    // println!("{}", n1 < n2);
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
}


#[cfg(test)]
mod test;
