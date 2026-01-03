use crate::fi::{FiBin, FiLong};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg, Not};
use std::time::Instant; // TODO: remove

// TODO: decide whether the spruce_up() method should be called


impl Add for FiBin {
    type Output = Self;

    fn add(self, num: Self) -> Self {
        let sign1 = self.sign;
        let sign2 = num.sign;
        if sign1 == true && sign2 == true {
            bin_add(self, num).neg() // add -
        } else if sign1 && !sign2 {
            bin_sub(num, self)
        } else if !sign1 && sign2 {
            bin_sub(self, num)
        } else {
            bin_add(self, num)
        }
    }

}

impl AddAssign for FiBin {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for FiBin {
    type Output = Self;

    fn sub(self, num: Self) -> Self {
        self + -num // change to neg once implemented
    }
}

impl SubAssign for FiBin {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other; // Fix
    }
}

impl Mul for FiBin {
    type Output = Self;

    fn mul(self, num: Self) -> Self {
        let res = bin_mul(self, num);
        bin_div(res, FiBin{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, true]}).spruce_up()
    }
}

impl MulAssign for FiBin {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl Div for FiBin {
    type Output = Self;

    fn div(self, num: Self) -> Self {
        let dividend = bin_mul(self, FiBin{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, true]});
        bin_div(dividend, num).spruce_up()
    }
}

impl DivAssign for FiBin {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

impl Rem for FiBin {
    type Output = Self;

    fn rem(self, num: Self) -> Self {
        bin_rem(self, num)
    }
}

impl RemAssign for FiBin {
    fn rem_assign(&mut self, other: Self) {
        *self = self.clone() % other;
    }
}

impl Neg for FiBin {
    type Output = Self; 

    fn neg(self) -> Self::Output {
        FiBin{sign: !self.sign, value: self.value}
    }
}
// TODO: look for a better implementation
impl Not for FiBin {
    type Output = Self; 

    fn not(self) -> Self::Output {
        FiBin{sign: !self.sign, value: self.value}
    }
}

// inline??

fn bin_add(num1: FiBin, num2: FiBin) -> FiBin { 
    let mut res: Vec<bool> = Vec::new();
    let mut carry: bool = false;
    let mut val1: Vec<bool> = num1.value;
    let mut val2: Vec<bool> = num2.value;
    if val1.len() > val2.len() {
        for _i in val2.len()..val1.len() {
            val2.push(false);
        }
    } else if val2.len() > val1.len() {
        for _i in val1.len()..val2.len() {
            val1.push(false);
        }
    }
    for i in 0..val1.len() { 
        if val1[i] == val2[i] {
            res.push(carry);
            if val1[i] {
                carry = true;
            } else {
                carry = false;
            }  
        } else {
            res.push(!carry);
        } 
    }
    if carry {
        res.push(true);
    }
    FiBin{sign: false, value: res}

}

fn bin_sub(num1: FiBin, num2: FiBin) -> FiBin {
    let sign: bool;
    let large;
    let mut small;
    if num1.abs() >= num2.abs() {
        small = num2.value;
        large = num1.value;
        sign = num1.sign;
    } else {
        small = num1.value;
        large = num2.value;
        sign = num2.sign;
    }
    for _i in small.len()..=large.len() { // is there a better way?
        small.push(false);
    }
    let mut value: Vec<bool> = Vec::new();
    let mut borrow: bool = false;
    for i in 0..large.len() {
        if large[i] == small[i] {
            value.push(borrow);
        } else {
            value.push(!borrow);
            if small[i] {
                borrow = true;
            } else {
                borrow = false;
            }
        }
    }
    FiBin{sign: sign, value: value}
}

fn bin_mul(num1: FiBin, num2: FiBin) -> FiBin {
    let sign: bool;
    let val1: Vec<bool> = num1.value;
    let val2: Vec<bool> = num2.value;
    let mut res: Vec<bool> = Vec::new();
    let mut carries: Vec<bool> = vec![false; val1.len() + val2.len()];
    if num1.sign == num2.sign {
        sign = false;
    } else {
        sign = true;
    }
    for i in 0..val1.len() {
        for j in 0..val2.len() {
            let index = i + j;
            if index < res.len() {
                if val1[i] == val2[j] && val1[i] {
                    res[index] ^= !carries[index];
                    if !res[index] || carries[index] {
                        if index + 1 < res.len() {
                            carries[index + 1] = true;
                        } else {
                            res.push(true);
                        }
                    }
                } else {
                    res[index] ^= carries[index];
                    if !res[index] && carries[index] {
                        carries[index + 1] = true;
                    }
                }
            } else {
                if val1[i] == val2[j] && val1[i] {
                    res.push(!carries[index]);
                    if !res[index] && carries[index] {
                        // carries[index + 1] = true;
                        res.push(true);
                    }
                } else {
                    res.push(carries[index]);
                }
            }
            // println!("carries: {:?}", carries);
            carries[index] = false;
        }
    }
    // println!("carries: {:?}", carries);
    if carries.last() == Some(&true) {
        res.push(true);
    }
    FiBin{sign: sign, value: res}
}

fn bin_div(num1: FiBin, num2: FiBin) -> FiBin {
    let time = Instant::now();
    let sign;
    if num1.sign == num2.sign {
        sign = false;
    } else {
        sign = true;
    }
    // let d = num2.abs();
    if num2.is_zero() {
        panic!("You can't divide by 0. Make sure your dividend is not equal to 0.")
    }
    let mut q: Vec<bool> = vec![false; num1.len() + num2.len()];
    let mut r: FiBin = FiBin{sign: false, value: vec![false]};
    for i in (0..num1.len()).rev() {
        
        r.insert(0, num1.value[i]);
        if r >= num2.abs() {
            r -= num2.abs();
            q[i] = true;
        }
    }  
    let mut res = FiBin{sign: sign, value: q};
    let double = bin_mul(r, FiBin{sign: false, value: vec![false, true]});
    if double >= num2.abs() {
        res += FiBin{sign: sign, value: vec![true]};
    }
    res
}

// TODO: implement a float division

// fn bin_floor(num1: FiBin, num2: FiBin) -> FiBin {
//     let sign;
//     if num1.sign == num2.sign {
//         sign = false;
//     } else {
//         sign = true;
//     }
//     // let d = num2.abs();
//     if num2.is_zero() {
//         panic!("You can't divide by 0. Make sure your dividend is not equal to 0.")
//     }
//     let mut q: Vec<bool> = vec![false; num1.len() + num2.len()];
//     let mut r: FiBin = FiBin{sign: false, value: vec![false]};
//     for i in (0..num1.len()).rev() {
//         r.value.insert(0, num1.value[i]);
//         // println!("r: {:?}", r);
        
//         if r >= num2.abs() {
//             r -= num2.abs();
//             q[i] = true;
//         }
        
//     }   
//     let mut res = FiBin{sign: sign, value: q};
//     res
// }

fn bin_rem(num1: FiBin, num2: FiBin) -> FiBin {
    let sign;
    sign = num1.sign;
    // let d = num2.abs();
    if num2.is_zero() {
        panic!("You can't divide by 0. Make sure your dividend is not equal to 0.")
    }
    let mut r: FiBin = FiBin{sign: false, value: vec![false]};
    for i in (0..num1.len()).rev() {
        r.value.insert(0, num1.value[i]);
        // println!("r: {:?}", r.to_string());
        
        if r >= num2.abs() {
            r -= num2.abs();
        }
        
    }   
    // println!("{:?}", q);
    r.sign = sign;
    r
}


pub fn time_comparison(num1: FiBin, num2: FiBin) {
    let int_1 = num1.parse::<i128>().unwrap();
    let int_2 = num2.clone().parse::<i128>().unwrap();
    let long_1 = (num1.clone() / FiBin::decimals()).to_long();
    let long_2 = (num2.clone() / FiBin::decimals()).to_long();
    let mul = Instant::now();
    let p: FiBin = bin_mul(num1.clone(), num2.clone()); // should probably test the gen function as they are the main functions
    println!("FiBin: {:?}", mul.elapsed());
    let mul_long = Instant::now();
    let p_long: FiLong = long_mul(long_1.clone(), long_2.clone());
    println!("FiLong: {:?}", mul_long.elapsed());
    println!("mul: {:?}", p_long);
    let mul_int = Instant::now();
    let p_int: i128= int_1 * int_2; // should probably test the gen function as they are the main functions
    println!("i128: {:?}", mul_int.elapsed());
    let div = Instant::now();
    let q: FiBin = bin_div(p.clone(), num2.clone());
    println!("FiBin: {:?}", div.elapsed());
    let div_int = Instant::now();
    let q_long: FiLong = p_long / long_2;
    println!("FiLong: {:?}", div_int.elapsed());
    println!("long: {:?}", q_long);
    println!("long: {:?}", long_1);
    let div_int = Instant::now();
    let q_int: i128 = p_int / int_2;
    println!("i128: {:?}", div_int.elapsed());
    let val: FiBin = FiBin::from(u64::MAX);
    let conv_int = Instant::now();
    let conv: String = val.to_string();
    println!("conv: {:?}", conv_int.elapsed());
    let div = Instant::now();
    p / q;
    println!("div: {:?}", div.elapsed());
    let vec: Vec<bool> = vec![true; 2];
    let u8 = Instant::now();
    let val = small_conv(vec);
    println!("u8: {:?}", u8.elapsed());
    println!("{:?}", val);
    let num = u8::MAX;
    let vec = Instant::now();
    let val = small_reconv(num);
    println!("reconv: {:?}", vec.elapsed());
    println!("{:?}", val);
}

fn small_conv(vec: Vec<bool>) -> u8 {
    let mut res: u8 = 0;
    let lookup: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
    for i in 0..vec.len() {
        if vec[i] {
            res += lookup[i];
        }
    }
    res
}

fn small_reconv(input: u8) -> Vec<bool> {
    let mut num: u8 = input.clone();
    let mut res: Vec<bool> = Vec::new();
    while num > 1 {
        res.push((num & 1) == 1);
        num >>= 2;
    }
    res
}





































impl Add for FiLong {
    type Output = Self;

    fn add(self, num: Self) -> Self {
        let sign1 = self.sign;
        let sign2 = num.sign;
        if sign1 == true && sign2 == true {
            long_add(self, num).neg() // add -
        } else if sign1 && !sign2 {
            long_sub(num, self)
        } else if !sign1 && sign2 {
            long_sub(self, num)
        } else {
            long_add(self, num)
        }
    }

}

impl AddAssign for FiLong {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for FiLong {
    type Output = Self;

    fn sub(self, num: Self) -> Self {
        self + -num
    }
}

impl SubAssign for FiLong {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other; // Fix
    }
}

impl Mul for FiLong {
    type Output = Self;

    fn mul(self, num: Self) -> Self {
        let res = long_mul(self, num);
        long_div(res, FiLong{sign: false, value: vec![7766279631452241920, 5]}).spruce_up()
    }
}

impl MulAssign for FiLong {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl Div for FiLong {
    type Output = Self;

    fn div(self, num: Self) -> Self {
        println!("num: {:?}", self.clone());
        let dividend = long_mul(self, FiLong{sign: false, value: vec![7766279631452241920, 5]});
        println!("num: {:?}", dividend.clone());
        long_div(dividend, num).spruce_up()
    }
}

impl DivAssign for FiLong {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

impl Rem for FiLong {
    type Output = Self;

    fn rem(self, num: Self) -> Self {
        long_rem(self, num)
    }
}

impl RemAssign for FiLong {
    fn rem_assign(&mut self, other: Self) {
        *self = self.clone() % other;
    }
}

impl Neg for FiLong {
    type Output = Self; 

    fn neg(self) -> Self::Output {
        FiLong{sign: !self.sign, value: self.value}
    }
}

#[inline(always)]
pub fn low_bits(num: u128) -> u128 {
	(num << 64) >> 64
}

#[inline(always)]
pub fn high_bits(num: u128) -> u128 {
	num >> 64
}
pub fn long_add(num1: FiLong, num2: FiLong) -> FiLong {
    let mut carry: u128 = 0;
    let bigger: FiLong;
    let smaller: FiLong;
    if num1.clone().abs() >= num2.clone().abs() {
        bigger = num1.clone();
        smaller = num2.clone();
    } else {
        bigger = num2.clone();
        smaller = num1.clone();
    }
    let mut result: FiLong = FiLong::new(); // TODO: change
    for i in 0..bigger.len() {
        let res: u128;
        if smaller.len() <= i {
            res = bigger[i] as u128 + carry;
        } else {
            res = bigger[i] as u128 + smaller[i] as u128 + carry;
        }
        carry = high_bits(res); // clone
        result.push(low_bits(res) as u64);

    }
    if carry != 0{
        result.push(carry as u64);
    }
    result.sign = bigger.sign;
    result
}

pub fn long_sub(num1: FiLong, num2: FiLong) -> FiLong {
    let mut borrow: u128 = 0;
    let bigger: FiLong;
    let smaller: FiLong;
    if num1.clone().abs() >= num2.clone().abs() {
        bigger = num1.clone();
        smaller = num2.clone();
    } else {
        bigger = num2.clone();
        smaller = num1.clone();
    }
    let mut result: FiLong = FiLong::new();
    for i in 0..bigger.len() {
        if smaller[i] + borrow as u64 > bigger[i] {
            result.push(u64::MAX - (smaller[i] - bigger[i]) - borrow as u64 + 1);
            borrow = 1;
        } else {
            result.push(bigger[i] - smaller[i] - borrow as u64);
            borrow = 0;
        }
    }
    result.sign = bigger.sign;
    result
}

pub fn long_mul(num1: FiLong, num2: FiLong) -> FiLong {
    if num1.is_zero() || num2.is_zero() {
        return FiLong::new();
    }
    let num1 = num1.spruce_up(); // these two lines are important as the length of a FiLong doesn't necessarily correlate with the used digits
    let num2 = num2.spruce_up();
    let len = num1.len() + num2.len(); 
    let mut carries: Vec<u128> = vec![0; len];
    let mut result: FiLong = FiLong{sign: num1.sign ^ num2.sign, value: vec![0; len]}; 
    for i in 0..num1.len() {
        for j in 0..num2.len() {
            let res: u128 = num1[i] as u128 * num2[j] as u128 + carries[i + j];
            let calc = result[i + j] as u128 + low_bits(res);
            carries[i + j + 1] += high_bits(res) + high_bits(calc); // overflow?? --> should'nt be a problem, right? maybe provide evidence
            result[i + j] = low_bits(calc) as u64;
            carries[i + j] = 0;
        }
    }
    if carries[len - 2] != 0 {
        result[len - 1] = carries[len - 2] as u64;
    }
    result
}

pub fn long_div(num1: FiLong, num2: FiLong) -> FiLong {
    println!{"{:?}", num1};
    let sign;
    if num1.sign == num2.sign {
        sign = false;
    } else {
        sign = true;
    }
    // let d = num2.abs();
    if num2.is_zero() {
        panic!("You can't divide by 0. Make sure your dividend is not equal to 0.")
    } else if num1.is_zero() {
        return num1;
    }
    let sq1 = num1.spruce_up();
    let offset = sq1.clone().last().unwrap().leading_zeros() as usize; // fix the unwrap()
    let n: FiLong = sq1 << offset;
    let mut inverse: FiLong = n.reverse_bits();
    let mut q = FiLong{sign: sign, value: vec![0; num1.value.capacity()]};
    let mut r: FiLong = FiLong{sign: false, value: vec![0]};
    let num_bits = (inverse.len() * 64) - 1 - offset;
    let mut bit_mask = FiLong{sign: false, value: vec![1]} << num_bits;
    for _ in 0..num_bits{
        r <<= 1;
        r[0] |= inverse[0] & 1;
        inverse >>= 1;
        if r >= num2 {
            r -= num2.clone();
            q |= bit_mask.clone();
        }
        bit_mask >>= 1;
    }   
    
    r <<= 1;
    if r >= num2.abs() {
        q |= bit_mask;
    }
    q
}

pub fn long_rem(num1: FiLong, num2: FiLong) -> FiLong {
    println!("n1: {:?}, n2: {:?}", num1, num2);
    let sign;
    if num1.sign == num2.sign {
        sign = false;
    } else {
        sign = true;
    }
    // let d = num2.abs();
    if num2.is_zero() {
        panic!("You can't divide by 0. Make sure your dividend is not equal to 0.")
    } else if num1.is_zero() {
        return num1;
    }
    let sq1 = num1.spruce_up();
    let offset = sq1.clone().last().unwrap().leading_zeros() as usize;
    let n: FiLong = sq1.clone() << offset;
    let mut inverse: FiLong = n.reverse_bits();
    let mut r: FiLong = FiLong{sign: false, value: vec![0]};
    let num_bits = (inverse.len() * 64) - 1 - offset;
    for _ in 0..num_bits{
        r <<= 1;
        r |= FiLong{sign: false, value: vec![inverse[0] % 2]};
        inverse >>= 1;
        if r >= num2 {
            r -= num2.clone();
        }
    }   
    r.sign = sign;
    r
}