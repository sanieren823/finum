mod fi;
mod errors;
mod conversion;
use crate::fi::Parsing;
// use crate::conversion::int::ParseInt;
mod operations;
use crate::fi::{FiBin, FiLong};
use std::time::Instant;
use crate::operations::arithm::{Ceil, Floor, Round, RoundN, single_limb_div, single_limb_rem};
use crate::operations::math::{Factorial, Logarithm, PowInteger, PowReal, PowerOfTwo, Exponential};
fn main() {
    // let fixed = fi::fi{sign: false, value: vec![false, true, false, true, true, true, false, false, true]};
    // let string = "109019".to_string();
    let n1: FiBin = FiBin::from(String::from("14"));
    let n2: FiBin = FiBin::from(String::from("2.5"));
    let dividend: FiBin = FiBin::from(String::from("0.030102999566398119521373889472449"));
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
    println!("val: {:?}", FiLong::from(2u128.pow(65)).parse::<u128>().unwrap());
    let mut new1 = n1.to_long();
    let mut new2 = n2.to_long();
    println!("sld: {:?}", single_limb_rem(&FiLong::e(), &FiLong::hundredth()));
    println!("{:?}", (dividend.to_long() % FiLong::one()).to_bin().to_string());
    println!("round: {:?}", dividend.to_long());
    println!("{:?}, {:?}", new1, new2);
    println!("sub: {:?}", (&new1 / &new2).to_bin().to_string());
    println!("{:?}", FiLong::one() * 10u128);
    println!("{:?}", FiLong::one() / 10u128);
    println!("{:?}", FiBin::from(String::from("2.718281828459045235360287471352")).to_long());
    println!("{:?}", FiBin::from(String::from("5.12039898129")).to_long().pot().to_bin().to_string());
    let time = Instant::now();
    new1.log2();
    println!("log: {:?}", time.elapsed());
    println!("100: {:?}; 1/100: {:?}", FiLong::one() * 100, FiLong::one() / 100);
    // println!("{:?}", n1.nth_root(n2.clone()).to_string());
    operations::arithm::time_comparison(FiBin::from(String::from("0897563456342")), FiBin::from(String::from("1238975634532")));
    // println!("{}", (f1 / f2).to_string());
    // println!("{res}");
    // println!("{}", n1 < n2);
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
    let start = Instant::now();
    let pot = new2.clone().pot();
    println!("{:?}, {:?}", start.elapsed(), pot.to_bin().to_string());
    let start = Instant::now();
    let pow = new1.pow_r(new2.clone());
    println!("{:?}, {:?}", start.elapsed(), pow.to_bin().to_string());
}


#[cfg(test)]
mod test;

