use crate::fi::Fi;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign, Neg, Not};

// add a mul_add (fused multiply and add)


impl Add for Fi {
    type Output = Self;

    fn add(self, num: Self) -> Self {
        let sign1 = self.sign;
        let sign2 = num.sign;
        if sign1 == true && sign2 == true {
            gen_add(self, num).neg() // add -
        } else if sign1 && !sign2 {
            gen_sub(num, self)
        } else if !sign1 && sign2 {
            gen_sub(self, num)
        } else {
            gen_add(self, num)
        }
    }

}

impl AddAssign for Fi {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl Sub for Fi {
    type Output = Self;

    fn sub(self, num: Self) -> Self {
        self + num.invert() // change to neg once implemented
    }
}

impl SubAssign for Fi {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other; // Fix
    }
}

impl Mul for Fi {
    type Output = Self;

    fn mul(self, num: Self) -> Self {
        let res = gen_mul(self, num);
        gen_div(res, Fi{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, true]})
    }
}

impl MulAssign for Fi {
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl Div for Fi {
    type Output = Self;

    fn div(self, num: Self) -> Self {
        let dividend = gen_mul(self, Fi{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, true]});
        gen_div(dividend, num)
    }
}

impl DivAssign for Fi {
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

impl Rem for Fi {
    type Output = Self;

    fn rem(self, num: Self) -> Self {
        gen_rem(self, num)
    }
}

impl RemAssign for Fi {
    fn rem_assign(&mut self, other: Self) {
        *self = self.clone() % other;
    }
}

impl Neg for Fi {
    type Output = Self; 

    fn neg(self) -> Self::Output {
        Fi{sign: !self.sign, value: self.value}
    }
}
// TODO: look for a better implementation
impl Not for Fi {
    type Output = Self; 

    fn not(self) -> Self::Output {
        Fi{sign: !self.sign, value: self.value}
    }
}

// inline??

fn gen_add(num1: Fi, num2: Fi) -> Fi { 
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
    Fi{sign: false, value: res}

}

fn gen_sub(num1: Fi, num2: Fi) -> Fi {
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
    Fi{sign: sign, value: value}
}

fn gen_mul(num1: Fi, num2: Fi) -> Fi {
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
    Fi{sign: sign, value: res}
}

fn gen_div(num1: Fi, num2: Fi) -> Fi {
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
    let mut r: Fi = Fi{sign: false, value: vec![false]};
    for i in (0..num1.len()).rev() {
        r.value.insert(0, num1.value[i]);
        // println!("r: {:?}", r);
        
        if r >= num2.abs() {
            r -= num2.abs();
            q[i] = true;
        }
        
    }   
    let mut res = Fi{sign: sign, value: q};
    let double = gen_mul(r, Fi{sign: false, value: vec![false, true]});
    if double >= num2.abs() {
        res += Fi{sign: sign, value: vec![true]};
    }
    res
}

// fn gen_floor(num1: Fi, num2: Fi) -> Fi {
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
//     let mut r: Fi = Fi{sign: false, value: vec![false]};
//     for i in (0..num1.len()).rev() {
//         r.value.insert(0, num1.value[i]);
//         // println!("r: {:?}", r);
        
//         if r >= num2.abs() {
//             r -= num2.abs();
//             q[i] = true;
//         }
        
//     }   
//     let mut res = Fi{sign: sign, value: q};
//     res
// }

fn gen_rem(num1: Fi, num2: Fi) -> Fi {
    let sign;
    sign = num1.sign;
    // let d = num2.abs();
    if num2.is_zero() {
        panic!("You can't divide by 0. Make sure your dividend is not equal to 0.")
    }
    let mut q: Vec<bool> = vec![false; num1.len() + num2.len()];
    let mut r: Fi = Fi{sign: false, value: vec![false]};
    for i in (0..num1.len()).rev() {
        r.value.insert(0, num1.value[i]);
        // println!("r: {:?}", r.to_string());
        
        if r >= num2.abs() {
            r -= num2.abs();
            q[i] = true;
        }
        
    }   
    // println!("{:?}", q);
    r.sign = sign;
    r
}


