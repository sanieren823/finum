use crate::finum::{FiBin, FiLong};
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg, Not};
use crate::operations::math::PowInteger;

pub trait Floor {
    type Output;

    fn floor(self) -> Self::Output;
}

pub trait Ceil {
    type Output;

    fn ceil(self) -> Self::Output;
}

pub trait Round {
    type Output;

    fn round(self) -> Self::Output;
}

pub trait RoundN<Rhs = Self> {
    type Output;

    fn round_n(self, rhs: Rhs) -> Self::Output;
}

impl Add for FiBin {
    type Output = FiBin;

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
    type Output = FiBin;

    fn sub(self, num: Self) -> Self {
        self + -num // change to neg once implemented
    }
}

impl SubAssign for FiBin {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl Mul for FiBin {
    type Output = FiBin;

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
    type Output = FiBin;

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
    type Output = FiBin;

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
    type Output = FiBin; 

    fn neg(self) -> Self::Output {
        FiBin{sign: !self.sign, value: self.value}
    }
}

// Implemented as Nothing/Something (something = 1)
impl Not for FiBin {
    type Output = FiBin; 

    fn not(self) -> Self::Output {
        if self.is_zero() {
            FiBin::one()
        } else {
            FiBin::new()
        }
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

impl Add<FiLong> for FiLong {
    type Output = FiLong;

    fn add(self, num: FiLong) -> Self::Output {
        &self + &num
    }

}

impl Add<&FiLong> for FiLong {
    type Output = FiLong;

    fn add(self, num: &FiLong) -> Self::Output {
        &self + num
    }

}

impl Add<FiLong> for &FiLong {
    type Output = FiLong;

    fn add(self, num: FiLong) -> Self::Output {
        self + &num
    }

}

impl Add<&FiLong> for &FiLong {
    type Output = FiLong;

    fn add(self, num: &FiLong) -> Self::Output {
        let sign1 = self.sign;
        let sign2 = num.sign;
        if sign1 == true && sign2 == true {
            long_add(self, num) 
        } else if sign1 && !sign2 {
            long_sub(num, self)
        } else if !sign1 && sign2 {
            long_sub(self, num)
        } else {
            long_add(self, num)
        }
    }

}

impl AddAssign<FiLong> for FiLong {
    fn add_assign(&mut self, other: FiLong) {
        *self += &other;
    }
}

impl AddAssign<&FiLong> for FiLong {
    fn add_assign(&mut self, other: &FiLong) {
        let sign1 = self.sign;
        let sign2 = other.sign;
        if sign1 == true && sign2 == true {
            *self = long_add(self, other);
        } else if sign1 && !sign2 {
            *self = long_sub(other, self);
        } else if !sign1 && sign2 {
            *self = long_sub(self, other);
        } else {
            *self = long_add(self, other);
        }
    }
}

impl Sub<FiLong> for FiLong {
    type Output = FiLong;

    fn sub(self, num: FiLong) -> Self::Output {
        &self - &num
    }
}

impl Sub<&FiLong> for FiLong {
    type Output = FiLong;

    fn sub(self, num: &FiLong) -> Self::Output {
        &self - num
    }
}

impl Sub<FiLong> for &FiLong {
    type Output = FiLong;

    fn sub(self, num: FiLong) -> Self::Output {
        self - &num
    }
}

impl Sub<&FiLong> for &FiLong {
    type Output = FiLong;

    fn sub(self, num: &FiLong) -> Self::Output {
        let sign1 = self.sign;
        let sign2 = !num.sign;
        if sign1 == true && sign2 == true {
            long_add(self, &-num)
        } else if sign1 && !sign2 {
            long_sub(&-num, self)
        } else if !sign1 && sign2 {
            long_sub(self, &-num)
        } else {
            long_add(self, &-num)
        }
    }
}


impl SubAssign<FiLong> for FiLong {
    fn sub_assign(&mut self, other: FiLong) {
        *self -= &other;
    }
}

impl SubAssign<&FiLong> for FiLong {
    fn sub_assign(&mut self, other: &FiLong) {
        let sign1 = self.sign;
        let sign2 = !other.sign;
        if sign1 == true && sign2 == true {
            *self = long_add(self, &-other);
        } else if sign1 && !sign2 {
            *self = long_sub(&-other, self);
        } else if !sign1 && sign2 {
            *self = long_sub(self, &-other);
        } else {
            *self = long_add(self, &-other);
        }
    }
}

impl Mul<FiLong> for FiLong {
    type Output = FiLong;

    fn mul(self, num: FiLong) -> Self::Output {
        &self * &num
    }
}

impl Mul<&FiLong> for FiLong {
    type Output = FiLong;

    fn mul(self, num: &FiLong) -> Self::Output {
        &self * num
    }
}

impl Mul<FiLong> for &FiLong {
    type Output = FiLong;

    fn mul(self, num: FiLong) -> Self::Output {
        self * &num
    }
}

impl Mul<&FiLong> for &FiLong {
    type Output = FiLong;

    fn mul(self, num: &FiLong) -> Self::Output {
        let res = long_mul(self, num);
        let sqrt_10_pow_20 = FiLong{sign: false, value: vec![10000000000]};
        let factor = single_limb_div(&res, &sqrt_10_pow_20).spruce_up();
        single_limb_div(&factor, &sqrt_10_pow_20).spruce_up()
    }
}

impl MulAssign<FiLong> for FiLong {
    fn mul_assign(&mut self, other: FiLong) {
        *self *= &other;
    }
}

impl MulAssign<&FiLong> for FiLong {
    fn mul_assign(&mut self, other: &FiLong) {
        let res = long_mul(self, other);
        let sqrt_10_pow_20 = FiLong{sign: false, value: vec![10000000000]};
        let factor = single_limb_div(&res, &sqrt_10_pow_20).spruce_up();
        *self = single_limb_div(&factor, &sqrt_10_pow_20).spruce_up();
    }
}

impl Div<FiLong> for FiLong {
    type Output = FiLong;

    fn div(self, num: FiLong) -> Self::Output {
        &self / &num
    }
}

impl Div<&FiLong> for FiLong {
    type Output = FiLong;

    fn div(self, num: &FiLong) -> Self::Output {
        &self / num
    }
}

impl Div<FiLong> for &FiLong {
    type Output = FiLong;

    fn div(self, num: FiLong) -> Self::Output {
        self / &num
    }
}

impl Div<&FiLong> for &FiLong {
    type Output = FiLong;

    fn div(self, num: &FiLong) -> Self::Output {
        let dividend = long_mul(&self, &FiLong{sign: false, value: vec![7766279631452241920, 5]});
        if num.absolute() > dividend.absolute() {
            return FiLong::new();
        }
        match num.len() {
            0 => panic!("You cannot divide by 0 in any cases."),
            1 => single_limb_div(&dividend, &num).spruce_up(),
            _=> algorithm_d_div(&dividend, &num).spruce_up(),
        }
    }
}


impl DivAssign<FiLong> for FiLong {
    fn div_assign(&mut self, other: Self) {
        *self /= &other;
    }
}

impl DivAssign<&FiLong> for FiLong {
    fn div_assign(&mut self, other: &FiLong) {
        let dividend = long_mul(&self, &FiLong{sign: false, value: vec![7766279631452241920, 5]});
        if other.absolute() > dividend.absolute() {
            *self = FiLong::new();
        }
        *self = match other.len() {
            0 => panic!("You cannot divide by 0 in any cases."),
            1 => single_limb_div(&dividend, &other).spruce_up(),
            _=> algorithm_d_div(&dividend, &other).spruce_up(),
        }
    }
}

impl Rem<FiLong> for FiLong {
    type Output = FiLong;

    fn rem(self, num: FiLong) -> Self::Output {
        &self % &num
    }
}

impl Rem<&FiLong> for FiLong {
    type Output = FiLong;

    fn rem(self, num: &FiLong) -> Self::Output {
        &self % num
    }
}

impl Rem<FiLong> for &FiLong {
    type Output = FiLong;

    fn rem(self, num: FiLong) -> Self::Output {
        self % &num
    }
}

impl Rem<&FiLong> for &FiLong {
    type Output = FiLong;

    fn rem(self, num: &FiLong) -> Self::Output {
        match num.len() {
            0 => self.clone(),
            1 => single_limb_rem(&self, &num).spruce_up(),
            _=> algorithm_d_rem(&self, &num).spruce_up(),
        }
    }
}

impl RemAssign<FiLong> for FiLong {
    fn rem_assign(&mut self, other: Self) {
        *self %= &other
    }
}

impl RemAssign<&FiLong> for FiLong {
    fn rem_assign(&mut self, other: &FiLong) {
        *self = match other.len() {
            0 => self.clone(),
            1 => single_limb_rem(&self, &other).spruce_up(),
            _=> algorithm_d_rem(&self, &other).spruce_up(),
        }
    }
}

impl Neg for FiLong {
    type Output = FiLong; 

    fn neg(self) -> Self::Output {
        FiLong{sign: !self.sign, value: self.value}
    }
}

impl Neg for &FiLong {
    type Output = FiLong; 

    fn neg(self) -> Self::Output {
        FiLong{sign: !self.sign, value: self.value.clone()}
    }
}

#[inline(always)]
fn low_bits(num: u128) -> u128 {
	(num << 64) >> 64
}

#[inline(always)]
fn high_bits(num: u128) -> u128 {
	num >> 64
}
fn long_add(num1: &FiLong, num2: &FiLong) -> FiLong {
    let mut carry: u128 = 0;
    let bigger: &FiLong;
    let smaller: &FiLong;
    if num1.absolute() >= num2.absolute() { // assigns the key variables after comparing the sizes of the parameters
        bigger = num1;
        smaller = num2;
    } else {
        bigger = num2;
        smaller = num1;
    }
    let mut result: FiLong = FiLong{sign: false, value: Vec::with_capacity(bigger.len() + 1)};
    for i in 0..bigger.len() { // standard carry-addition
        let res: u128;
        if smaller.len() <= i {
            res = bigger[i] as u128 + carry;
        } else {
            res = bigger[i] as u128 + smaller[i] as u128 + carry;
        }
        carry = high_bits(res);
        result.push(low_bits(res) as u64);

    }
    if carry != 0 { // chechs whether the last calculation produced a number larger than the u64 limit and adds it to the output if that's the case
        result.push(carry as u64);
    }
    result.sign = bigger.sign;
    result
}

fn long_sub(num1: &FiLong, num2: &FiLong) -> FiLong {
    let mut borrow: u128 = 0;
    let bigger: &FiLong;
    let smaller: &FiLong;
    if num1.absolute() >= num2.absolute() { // assigns the key variables after comparing the sizes of the parameters
        bigger = num1;
        smaller = num2;
    } else {
        bigger = num2;
        smaller = num1;
    }
    let mut result: FiLong = FiLong{sign: false, value: Vec::with_capacity(bigger.len())};
    for i in 0..bigger.len() { // standard borrow-subtraction
        if smaller.len() <= i {
            if borrow as u64 > bigger[i] {
                result.push(u64::MAX);
            } else {
                result.push(bigger[i] - borrow as u64);
                borrow = 0;
            }
        } else if smaller[i] + borrow as u64 > bigger[i] {
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

fn long_mul(num1: &FiLong, num2: &FiLong) -> FiLong {
    if num1.is_zero() || num2.is_zero() {
        return FiLong::new();
    }
    let num1 = num1.spruce_up(); // these two lines are important as the length of a FiLong doesn't necessarily correlate with the used digits
    let num2 = num2.spruce_up();
    let len = num1.len() + num2.len(); 
    let mut result: FiLong = FiLong{sign: num1.sign ^ num2.sign, value: Vec::with_capacity(len)}; 
    result.resize(len, 0);
    for i in 0..num1.len() { // somewhat standard multiplication
        let mut carry: u128 = 0;
        for j in 0..num2.len() {
            let prod: u128 = num1[i] as u128 * num2[j] as u128;
            let sum: u128 = result[i + j] as u128 + prod + carry;
            result[i + j] = sum as u64;
            carry = sum >> 64;
        }
        result[i + num2.len()] = carry as u64;
    }
    // if carries[len - 1] != 0 { // checks if there's a carry in the last operation
    //     result[len - 1] = carries[len - 1] as u64;
    // }
    result.spruce_up()
}


// regular long division --> fully tested stays for redundancy purposes
// fn long_div(num1: &FiLong, num2: &FiLong) -> FiLong {
//     let sign; // "calculates" the sign of the result
//     if num1.sign == num2.sign {
//         sign = false;
//     } else {
//         sign = true;
//     }
//     if num2.is_zero() { // checks if either input is zero
//         panic!("You can't divide by 0. Make sure your dividend is not equal to 0.") // i know it's not proper error handeling but it's an internal function that's not meant to be used by anyone + run time?
//     } else if num1.is_zero() {
//         return FiLong::new();
//     }
//     let dividend = num1.spruce_up();
//     let offset = dividend[dividend.len() - 1].leading_zeros() as usize;
//     let n: FiLong = dividend << offset;
//     let mut inverse: FiLong = n.reverse_bits(); // in normal long division you iterate through the number/vector/bits from end to start. for some reason i wanted to avoid that which is why i calculated the inverse number (i think i belived that the num_bits - i would be less efficient than just computing the inverse given that the run time scales with size)
//     let mut q = FiLong{sign: sign, value: vec![0; num1.value.capacity()]};
//     let mut r: FiLong = FiLong{sign: false, value: vec![0]};
//     let num_bits = (inverse.len() * 64) - offset;
//     let mut bit_mask = FiLong{sign: false, value: vec![1]} << num_bits;
//     for _ in 0..num_bits{ // standard long division
//         bit_mask >>= 1;
//         r <<= 1;
//         r[0] |= inverse[0] & 1;
//         inverse >>= 1;
//         if r >= num2.absolute() {
//             r -= num2.absolute();
//             q |= &bit_mask;
//         }
        
//     }
//     r <<= 1;
//     if r >= num2.absolute() { // rounds if necessary
//         return long_add(&q, &bit_mask);
//     }
//     q.spruce_up()
// }

// fn long_rem(num1: &FiLong, num2: &FiLong) -> FiLong {
//     if num2.is_zero() { // special cases
//         return num1.clone();
//     } else if num1.is_zero() {
//         return FiLong::new();
//     }
//     let dividend = num1.spruce_up(); // i think it's still called dividend for a modulu operation
//     let offset = dividend[dividend.len() - 1].leading_zeros() as usize;
//     let n: FiLong = dividend << offset;
//     let mut inverse: FiLong = n.reverse_bits();
//     let mut r: FiLong = FiLong{sign: false, value: vec![0]};
//     let num_bits = (inverse.len() * 64) - offset;
//     for _ in 0..num_bits{ // long division
//         r <<= 1;
//         r[0] |= inverse[0] & 1;
//         inverse >>= 1;
//         if r >= num2.absolute() {
//             r -= num2.absolute();
//         }
//     }   
//     r.sign = num1.sign;
//     r.spruce_up()
// }


impl Floor for FiLong {
    type Output = FiLong;

    fn floor(self) -> Self::Output {
        if self.spruce_up().len() <= 1 {
            return FiLong::new();
        }
        let binary = algorithm_d_floor(&self, &FiLong::one());
        long_mul(&binary, &FiLong::one())
    }
}

impl Floor for &FiLong {
    type Output = FiLong;

    fn floor(self) -> Self::Output {
        if self.spruce_up().len() <= 1 {
            return FiLong::new();
        }
        let binary = algorithm_d_floor(self, &FiLong::one());
        long_mul(&binary, &FiLong::one())
    }
}

impl Ceil for FiLong {
    type Output = FiLong;

    fn ceil(self) -> Self::Output {
        let len = self.spruce_up().len();
        match len {
            0 => FiLong::new(),
            1 => FiLong::one(),
            _ => { 
                let binary = algorithm_d_ceil(&self, &FiLong::one());
                long_mul(&binary, &FiLong::one())
            }
        }
    }
}

impl Ceil for &FiLong {
    type Output = FiLong;

    fn ceil(self) -> Self::Output {
        let len = self.spruce_up().len();
        match len {
            0 => FiLong::new(),
            1 => FiLong::one(),
            _ => { 
                let binary = algorithm_d_ceil(self, &FiLong::one());
                long_mul(&binary, &FiLong::one())
            }
        }
        
    }
}

impl Round for FiLong {
    type Output = FiLong;

    fn round(self) -> Self::Output {
        if self.spruce_up().len() <= 1 {
            return FiLong::new();
        }
        let binary = algorithm_d_div(&self, &FiLong::one());
        long_mul(&binary, &FiLong::one())
    }
}

impl Round for &FiLong {
    type Output = FiLong;

    fn round(self) -> Self::Output {
        if self.spruce_up().len() <= 1 {
            return FiLong::new();
        }
        let binary = algorithm_d_div(self, &FiLong::one());
        long_mul(&binary, &FiLong::one())
    }
}

impl RoundN<usize> for FiLong {
    type Output = FiLong;

    fn round_n(self, num: usize) -> Self::Output {
        if num >= 20 {
            self
        } else if num == 0 {
            self.round()
        } else {
            let factor: FiLong = FiLong::ten().pow_int(20 - num) / FiLong::decimals();
            let binary = match factor.len() {
                0 => panic!("You cannot divide by 0 in any cases."),
                1 => single_limb_div(&self, &factor).spruce_up(),
                _=> algorithm_d_div(&self, &factor).spruce_up(),
            };
            long_mul(&binary, &factor)
        }
    }
}
impl RoundN<&usize> for FiLong {
    type Output = FiLong;

    fn round_n(self, num: &usize) -> Self::Output {
        if *num >= 20 {
            self
        } else if *num == 0 {
            self.round()
        } else {
            let factor: FiLong = FiLong::ten().pow_int(20 - num) / FiLong::decimals();
            let binary = match factor.len() {
                0 => panic!("You cannot divide by 0 in any cases."),
                1 => single_limb_div(&self, &factor).spruce_up(),
                _=> algorithm_d_div(&self, &factor).spruce_up(),
            };
            long_mul(&binary, &factor)
        }
    }
}

impl RoundN<usize> for &FiLong {
    type Output = FiLong;

    fn round_n(self, num: usize) -> Self::Output {
        if num >= 20 {
            self.clone()
        } else if num == 0 {
            self.round()
        } else {
            let factor: FiLong = FiLong::ten().pow_int(20 - num) / FiLong::decimals();
            let binary = match factor.len() {
                0 => panic!("You cannot divide by 0 in any cases."),
                1 => single_limb_div(self, &factor).spruce_up(),
                _=> algorithm_d_div(self, &factor).spruce_up(),
            };
            long_mul(&binary, &factor)
        }
    }
}

impl RoundN<&usize> for &FiLong {
    type Output = FiLong;

    fn round_n(self, num: &usize) -> Self::Output {
        if *num >= 20 {
            self.clone()
        } else if *num == 0 {
            self.round()
        } else {
            let factor: FiLong = FiLong::ten().pow_int(20 - num) / FiLong::decimals();
            let binary = match factor.len() {
                0 => panic!("You cannot divide by 0 in any cases."),
                1 => single_limb_div(self, &factor).spruce_up(),
                _=> algorithm_d_div(self, &factor).spruce_up(),
            };
            long_mul(&binary, &factor)
        }
    }
}


macro_rules! base_types_arithmetic {
    ($type:ty) => {
        impl Add<$type> for FiLong {
            type Output = FiLong;

            fn add(self, num: $type) -> Self::Output {
                self + FiLong::from(num)
            }
        }
        impl Add<&$type> for FiLong {
            type Output = FiLong;

            fn add(self, num: &$type) -> Self::Output {
                self + FiLong::from(*num)
            }
        }
        impl Add<$type> for &FiLong {
            type Output = FiLong;

            fn add(self, num: $type) -> Self::Output {
                self + FiLong::from(num)
            }
        }
        impl Add<&$type> for &FiLong {
            type Output = FiLong;

            fn add(self, num: &$type) -> Self::Output {
                self + FiLong::from(*num)
            }  
        }
        impl AddAssign<$type> for FiLong {
            fn add_assign(&mut self, num: $type) {
                *self += FiLong::from(num);
            }
        }
        impl AddAssign<&$type> for FiLong {
            fn add_assign(&mut self, num: &$type) {
                *self += FiLong::from(*num);
            }  
        }
        impl Sub<$type> for FiLong {
            type Output = FiLong;

            fn sub(self, num: $type) -> Self::Output {
                self - FiLong::from(num)
            }
        }
        impl Sub<&$type> for FiLong {
            type Output = FiLong;

            fn sub(self, num: &$type) -> Self::Output {
                self - FiLong::from(*num)
            }
        }
        impl Sub<$type> for &FiLong {
            type Output = FiLong;

            fn sub(self, num: $type) -> Self::Output {
                self - FiLong::from(num)
            }
        }
        impl Sub<&$type> for &FiLong {
            type Output = FiLong;

            fn sub(self, num: &$type) -> Self::Output {
                self - FiLong::from(*num)
            }  
        }
        impl SubAssign<$type> for FiLong {
            fn sub_assign(&mut self, num: $type) {
                *self -= FiLong::from(num);
            }
        }
        impl SubAssign<&$type> for FiLong {
            fn sub_assign(&mut self, num: &$type) {
                *self -= FiLong::from(*num);
            }  
        }
        impl Mul<$type> for FiLong {
            type Output = FiLong;

            fn mul(self, num: $type) -> Self::Output {
                self * FiLong::from(num)
            }
        }
        impl Mul<&$type> for FiLong {
            type Output = FiLong;

            fn mul(self, num: &$type) -> Self::Output {
                self * FiLong::from(*num)
            }
        }
        impl Mul<$type> for &FiLong {
            type Output = FiLong;

            fn mul(self, num: $type) -> Self::Output {
                self * FiLong::from(num)
            }
        }
        impl Mul<&$type> for &FiLong {
            type Output = FiLong;

            fn mul(self, num: &$type) -> Self::Output {
                self * FiLong::from(*num)
            }  
        }
        impl MulAssign<$type> for FiLong {
            fn mul_assign(&mut self, num: $type) {
                *self *= FiLong::from(num);
            }
        }
        impl MulAssign<&$type> for FiLong {
            fn mul_assign(&mut self, num: &$type) {
                *self *= FiLong::from(*num);
            }  
        }
        impl Div<$type> for FiLong {
            type Output = FiLong;

            fn div(self, num: $type) -> Self::Output {
                self / FiLong::from(num)
            }
        }
        impl Div<&$type> for FiLong {
            type Output = FiLong;

            fn div(self, num: &$type) -> Self::Output {
                self / FiLong::from(*num)
            }
        }
        impl Div<$type> for &FiLong {
            type Output = FiLong;

            fn div(self, num: $type) -> Self::Output {
                self / FiLong::from(num)
            }
        }
        impl Div<&$type> for &FiLong {
            type Output = FiLong;

            fn div(self, num: &$type) -> Self::Output {
                self / FiLong::from(*num)
            }  
        }
        impl DivAssign<$type> for FiLong {
            fn div_assign(&mut self, num: $type) {
                *self /= FiLong::from(num);
            }
        }
        impl DivAssign<&$type> for FiLong {
            fn div_assign(&mut self, num: &$type) {
                *self /= FiLong::from(*num);
            }  
        }
        impl Rem<$type> for FiLong {
            type Output = FiLong;

            fn rem(self, num: $type) -> Self::Output {
                self % FiLong::from(num)
            }
        }
        impl Rem<&$type> for FiLong {
            type Output = FiLong;

            fn rem(self, num: &$type) -> Self::Output {
                self % FiLong::from(*num)
            }
        }
        impl Rem<$type> for &FiLong {
            type Output = FiLong;

            fn rem(self, num: $type) -> Self::Output {
                self % FiLong::from(num)
            }
        }
        impl Rem<&$type> for &FiLong {
            type Output = FiLong;

            fn rem(self, num: &$type) -> Self::Output {
                self % FiLong::from(*num)
            }  
        }
        impl RemAssign<$type> for FiLong {
            fn rem_assign(&mut self, num: $type) {
                *self %= FiLong::from(num);
            }
        }
        impl RemAssign<&$type> for FiLong {
            fn rem_assign(&mut self, num: &$type) {
                *self %= FiLong::from(*num);
            }  
        }
        impl Add<FiLong> for $type {
            type Output = FiLong;

            fn add(self, num: FiLong) -> Self::Output {
                FiLong::from(self) + num
            }  
        }
        impl Add<&FiLong> for $type {
            type Output = FiLong;

            fn add(self, num: &FiLong) -> Self::Output {
                FiLong::from(self) + num
            }  
        }
        impl Add<FiLong> for &$type {
            type Output = FiLong;

            fn add(self, num: FiLong) -> Self::Output {
                FiLong::from(*self) + num
            }  
        }
        impl Add<&FiLong> for &$type {
            type Output = FiLong;

            fn add(self, num: &FiLong) -> Self::Output {
                FiLong::from(*self) + num
            }  
        }
        impl Sub<FiLong> for $type {
            type Output = FiLong;

            fn sub(self, num: FiLong) -> Self::Output {
                FiLong::from(self) - num
            }  
        }
        impl Sub<&FiLong> for $type {
            type Output = FiLong;

            fn sub(self, num: &FiLong) -> Self::Output {
                FiLong::from(self) - num
            }  
        }
        impl Sub<FiLong> for &$type {
            type Output = FiLong;

            fn sub(self, num: FiLong) -> Self::Output {
                FiLong::from(*self) - num
            }  
        }
        impl Sub<&FiLong> for &$type {
            type Output = FiLong;

            fn sub(self, num: &FiLong) -> Self::Output {
                FiLong::from(*self) - num
            }  
        }
        impl Mul<FiLong> for $type {
            type Output = FiLong;

            fn mul(self, num: FiLong) -> Self::Output {
                FiLong::from(self) * num
            }  
        }
        impl Mul<&FiLong> for $type {
            type Output = FiLong;

            fn mul(self, num: &FiLong) -> Self::Output {
                FiLong::from(self) * num
            }  
        }
        impl Mul<FiLong> for &$type {
            type Output = FiLong;

            fn mul(self, num: FiLong) -> Self::Output {
                FiLong::from(*self) * num
            }  
        }
        impl Mul<&FiLong> for &$type {
            type Output = FiLong;

            fn mul(self, num: &FiLong) -> Self::Output {
                FiLong::from(*self) * num
            }  
        }
        impl Div<FiLong> for $type {
            type Output = FiLong;

            fn div(self, num: FiLong) -> Self::Output {
                FiLong::from(self) / num
            }  
        }
        impl Div<&FiLong> for $type {
            type Output = FiLong;

            fn div(self, num: &FiLong) -> Self::Output {
                FiLong::from(self) / num
            }  
        }
        impl Div<FiLong> for &$type {
            type Output = FiLong;

            fn div(self, num: FiLong) -> Self::Output {
                FiLong::from(*self) / num
            }  
        }
        impl Div<&FiLong> for &$type {
            type Output = FiLong;

            fn div(self, num: &FiLong) -> Self::Output {
                FiLong::from(*self) / num
            }  
        }
        impl Rem<FiLong> for $type {
            type Output = FiLong;

            fn rem(self, num: FiLong) -> Self::Output {
                FiLong::from(self) % num
            }  
        }
        impl Rem<&FiLong> for $type {
            type Output = FiLong;

            fn rem(self, num: &FiLong) -> Self::Output {
                FiLong::from(self) % num
            }  
        }
        impl Rem<FiLong> for &$type {
            type Output = FiLong;

            fn rem(self, num: FiLong) -> Self::Output {
                FiLong::from(*self) % num
            }  
        }
        impl Rem<&FiLong> for &$type {
            type Output = FiLong;

            fn rem(self, num: &FiLong) -> Self::Output {
                FiLong::from(*self) % num
            }  
        }
    };
}

base_types_arithmetic!(isize);
base_types_arithmetic!(i8);
base_types_arithmetic!(i16);
base_types_arithmetic!(i32);
base_types_arithmetic!(i64);
base_types_arithmetic!(i128);
base_types_arithmetic!(usize);
base_types_arithmetic!(u8);
base_types_arithmetic!(u16);
base_types_arithmetic!(u32);
base_types_arithmetic!(u64);
base_types_arithmetic!(u128);
base_types_arithmetic!(f32);
base_types_arithmetic!(f64);













pub fn single_limb_div(num1: &FiLong, num2: &FiLong) -> FiLong { // remove pub
    let sign; // "calculates" the sign of the result
    if num1.sign == num2.sign {
        sign = false;
    } else {
        sign = true;
    }
    if num2[0] == 1 {
        return num1.clone();
    }
    let len = num1.len();
    if len == 0 {
        return FiLong::new();
    }
    let mut res = FiLong{sign: sign, value: Vec::with_capacity(len)};
    res.resize(len, 0);
    let mut carry: u128 = num1[len - 1] as u128;
    for i in (0..len - 1).rev() {
        let num = carry * 2u128.pow(64) + num1[i] as u128;
        let div = num / num2[0] as u128;
        let rem = num % num2[0] as u128;
        res[i + 1] += high_bits(div) as u64;
        res[i] += low_bits(div) as u64;
        carry = rem;
    }
    if carry * 2 > num2[0] as u128 {
        res += FiLong::smallest_val();
    }
    res
}

fn single_limb_rem(num1: &FiLong, num2: &FiLong) -> FiLong { // remove pub
    let sign; // "calculates" the sign of the result
    if num1.sign == num2.sign {
        sign = false;
    } else {
        sign = true;
    }
    if num2[0] == 1 {
        return FiLong::one();
    }
    let len = num1.len();
    let mut carry: u128 = num1[len - 1] as u128;
    for i in (0..len - 1).rev() {
        let num = carry * 2u128.pow(64) + num1[i] as u128;
        let rem = num % num2[0] as u128;
        carry = rem;
    }
    FiLong{sign: sign, value: vec![carry as u64]}
}


// https://skanthak.hier-im-netz.de/division.html
fn algorithm_d_div(num1: &FiLong, num2: &FiLong) -> FiLong { 
    // important constants
    let n = num2.len();
    let m = num1.len() - n;
    const B: u128 = u64::MAX as u128 + 1;

    // result handeling
    let sign = num1.sign ^ num2.sign;
    let mut res = FiLong::with_capacity(m);
    res.sign = sign;
    res.resize(m + 1, 0);
    
    // initial multiplication
    let s = num2[n - 1].leading_zeros();
    let mut v: Vec<u64> = Vec::with_capacity(n);
    for i in 0..n {
        let high_bits = num2.value[i] << s;
        let low_bits = if i > 0 {
            num2.value[i - 1] >> (64 - s)
        } else {
            0
        };
        v.push(high_bits | low_bits);
    }
    let mut u: Vec<u64> = Vec::with_capacity(m + n + 1);
    for i in 0..num1.len() {
        let high_bits = num1.value[i] << s;
        let low_bits = if i > 0 {
            num1.value[i - 1] >> (64 - s)
        } else {
            0
        };
        u.push(high_bits | low_bits);
    }
    u.push(num1.value[num1.len() - 1] >> (64 - s)); 

    let mut q: u128;
    let mut r: u128;
    for j in (0..=m).rev() {
        q = (u[n + j] as u128 * B + u[n + j - 1] as u128) / v[n - 1] as u128; 
        r = (u[n + j] as u128 * B + u[n + j - 1] as u128) % v[n - 1] as u128;
        loop {
            if (q >= B) || q * v[n - 2] as u128 > (r * B + u[n + j - 2] as u128) {
                q -= 1;
                
                r += v[n - 1] as u128;
                if r > B {
                    break;
                }
            } else {
                break;
            }
        }
        let mut product = v.clone();
        multiply(&mut product, q, 0);
        // checks if value would be negative
        if compare(&u, &product, j, n) {
            q -= 1;
            product = v.clone();
            multiply(&mut product, q, 0);
        }
        res[j] = q as u64;
        // subtraction logic
        let mut borrow: u64 = 0;
        for i in 0..=n {
            let ui = u[i + j] as u128;
            let pi = product[i] as u128 + borrow as u128;
            if ui >= pi {
                u[i + j] = (ui - pi) as u64;
                borrow = 0;
            } else {
                u[i + j] = ((ui + B) - pi) as u64;
                borrow = 1;
            }
        }
    }
    let mut rem: Vec<u64> = Vec::with_capacity(u.capacity());
    rem.resize(u.len(), 0);
    for i in (0..u.len()).rev() {
        rem[i] |= u[i] >> s;
        if i > 0 {
            rem[i - 1] |= u[i] << (64 - s);
        }
    }
    

    let fi_rem = FiLong{sign: false, value: rem}.spruce_up();
    if fi_rem * FiLong::two() > num2.absolute() {
        res += FiLong::smallest_val();
    }
    res
}

fn algorithm_d_floor(num1: &FiLong, num2: &FiLong) -> FiLong { 
    // important constants
    let n = num2.len();
    let m = num1.len() - n;
    const B: u128 = u64::MAX as u128 + 1;

    // result handeling
    let sign = num1.sign ^ num2.sign;
    let mut res = FiLong::with_capacity(m);
    res.sign = sign;
    res.resize(m + 1, 0);
    
    // initial multiplication
    let s = num2[n - 1].leading_zeros();
    let mut v: Vec<u64> = Vec::with_capacity(n);
    for i in 0..n {
        let high_bits = num2.value[i] << s;
        let low_bits = if i > 0 {
            num2.value[i - 1] >> (64 - s)
        } else {
            0
        };
        v.push(high_bits | low_bits);
    }
    let mut u: Vec<u64> = Vec::with_capacity(m + n + 1);
    for i in 0..num1.len() {
        let high_bits = num1.value[i] << s;
        let low_bits = if i > 0 {
            num1.value[i - 1] >> (64 - s)
        } else {
            0
        };
        u.push(high_bits | low_bits);
    }
    u.push(num1.value[num1.len() - 1] >> (64 - s)); 

    let mut q: u128;
    let mut r: u128;
    for j in (0..=m).rev() {
        q = (u[n + j] as u128 * B + u[n + j - 1] as u128) / v[n - 1] as u128; 
        r = (u[n + j] as u128 * B + u[n + j - 1] as u128) % v[n - 1] as u128;
        loop {
            if (q >= B) || q * v[n - 2] as u128 > (r * B + u[n + j - 2] as u128) {
                q -= 1;
                
                r += v[n - 1] as u128;
                if r > B {
                    break;
                }
            } else {
                break;
            }
        }
        let mut product = v.clone();
        multiply(&mut product, q, 0);
        // checks if value would be negative
        if compare(&u, &product, j, n) {
            q -= 1;
            product = v.clone();
            multiply(&mut product, q, 0);
        }
        res[j] = q as u64;
        // subtraction logic
        let mut borrow: u64 = 0;
        for i in 0..=n {
            let ui = u[i + j] as u128;
            let pi = product[i] as u128 + borrow as u128;
            if ui >= pi {
                u[i + j] = (ui - pi) as u64;
                borrow = 0;
            } else {
                u[i + j] = ((ui + B) - pi) as u64;
                borrow = 1;
            }
        }
    }
    res
}

fn algorithm_d_ceil(num1: &FiLong, num2: &FiLong) -> FiLong { 
    // important constants
    let n = num2.len();
    let m = num1.len() - n;
    const B: u128 = u64::MAX as u128 + 1;

    // result handeling
    let sign = num1.sign ^ num2.sign;
    let mut res = FiLong::with_capacity(m);
    res.sign = sign;
    res.resize(m + 1, 0);
    
    // initial multiplication
    let s = num2[n - 1].leading_zeros();
    let mut v: Vec<u64> = Vec::with_capacity(n);
    for i in 0..n {
        let high_bits = num2.value[i] << s;
        let low_bits = if i > 0 {
            num2.value[i - 1] >> (64 - s)
        } else {
            0
        };
        v.push(high_bits | low_bits);
    }
    let mut u: Vec<u64> = Vec::with_capacity(m + n + 1);
    for i in 0..num1.len() {
        let high_bits = num1.value[i] << s;
        let low_bits = if i > 0 {
            num1.value[i - 1] >> (64 - s)
        } else {
            0
        };
        u.push(high_bits | low_bits);
    }
    u.push(num1.value[num1.len() - 1] >> (64 - s)); 

    let mut q: u128;
    let mut r: u128;
    for j in (0..=m).rev() {
        q = (u[n + j] as u128 * B + u[n + j - 1] as u128) / v[n - 1] as u128; 
        r = (u[n + j] as u128 * B + u[n + j - 1] as u128) % v[n - 1] as u128;
        loop {
            if (q >= B) || q * v[n - 2] as u128 > (r * B + u[n + j - 2] as u128) {
                q -= 1;
                
                r += v[n - 1] as u128;
                if r > B {
                    break;
                }
            } else {
                break;
            }
        }
        let mut product = v.clone();
        multiply(&mut product, q, 0);
        // checks if value would be negative
        if compare(&u, &product, j, n) {
            q -= 1;
            product = v.clone();
            multiply(&mut product, q, 0);
        }
        res[j] = q as u64;
        // subtraction logic
        let mut borrow: u64 = 0;
        for i in 0..=n {
            let ui = u[i + j] as u128;
            let pi = product[i] as u128 + borrow as u128;
            if ui >= pi {
                u[i + j] = (ui - pi) as u64;
                borrow = 0;
            } else {
                u[i + j] = ((ui + B) - pi) as u64;
                borrow = 1;
            }
        }
    }
    let mut rem: Vec<u64> = Vec::with_capacity(u.capacity());
    rem.resize(u.len(), 0);
    for i in (0..u.len()).rev() {
        rem[i] |= u[i] >> s;
        if i > 0 {
            rem[i - 1] |= u[i] << (64 - s);
        }
    }
    

    let fi_rem = FiLong{sign: false, value: rem}.spruce_up();
    if !fi_rem.is_zero() {
        res += FiLong::smallest_val();
    }
    res
}

// https://skanthak.hier-im-netz.de/division.html
fn algorithm_d_rem(num1: &FiLong, num2: &FiLong) -> FiLong { 
    // important constants
    let n = num2.len();
    let m = num1.len() - n;
    const B: u128 = u64::MAX as u128 + 1;
    
    // initial multiplication
    let s = num2[n - 1].leading_zeros();
    let mut v: Vec<u64> = Vec::with_capacity(n);
    for i in 0..n {
        let high_bits = num2.value[i] << s;
        let low_bits = if i > 0 {
            num2.value[i - 1] >> (64 - s)
        } else {
            0
        };
        v.push(high_bits | low_bits);
    }
    let mut u: Vec<u64> = Vec::with_capacity(m + n + 1);
    for i in 0..num1.len() {
        let high_bits = num1.value[i] << s;
        let low_bits = if i > 0 {
            num1.value[i - 1] >> (64 - s)
        } else {
            0
        };
        u.push(high_bits | low_bits);
    }
    u.push(num1.value[num1.len() - 1] >> (64 - s)); 

    let mut q: u128;
    let mut r: u128;
    for j in (0..=m).rev() {
        q = (u[n + j] as u128 * B + u[n + j - 1] as u128) / v[n - 1] as u128; 
        r = (u[n + j] as u128 * B + u[n + j - 1] as u128) % v[n - 1] as u128;
        loop {
            if (q >= B) || q * v[n - 2] as u128 > (r * B + u[n + j - 2] as u128) {
                q -= 1;
                
                r += v[n - 1] as u128;
                if r > B {
                    break;
                }
            } else {
                break;
            }
        }
        let mut product = v.clone();
        multiply(&mut product, q, 0);
        // checks if value would be negative
        if compare(&u, &product, j, n) {
            q -= 1;
            product = v.clone();
            multiply(&mut product, q, 0);
        }
        // subtraction logic
        let mut borrow: u64 = 0;
        for i in 0..=n {
            let ui = u[i + j] as u128;
            let pi = product[i] as u128 + borrow as u128;
            if ui >= pi {
                u[i + j] = (ui - pi) as u64;
                borrow = 0;
            } else {
                u[i + j] = ((ui + B) - pi) as u64;
                borrow = 1;
            }
        }
    }

    // bit shifting algorithm
    let mut result: Vec<u64> = Vec::with_capacity(u.capacity());
    result.resize(u.len(), 0);
    for i in (0..u.len()).rev() {
        result[i] |= u[i] >> s;
        if i > 0 {
            result[i - 1] |= u[i] << (64 - s);
        }
    }
    FiLong{sign: num1.sign, value: result}
}

fn multiply(vec: &mut Vec<u64>, factor: u128, start: usize) {
    let mut carry: u128 = 0;
    for i in start..vec.len() {
        let double_long = vec[i] as u128 * factor + carry;
        vec[i] = low_bits(double_long) as u64;
        carry = high_bits(double_long);
    }
    vec.push(carry as u64);
}


fn compare(val1: &Vec<u64>, val2: &Vec<u64>, summand: usize, n: usize) -> bool {
    for i in (0..=n).rev() {
        if val2[i] > val1[i + summand] {
            return true;
        } else if val2[i] < val1[i + summand] {
            return false;
        }
    }
    false
}
