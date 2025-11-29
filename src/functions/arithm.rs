use crate::fi::fi;
use crate::fi::bcd;
use crate::functions;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Rem;
use std::ops::RemAssign;

// add a mul_add (fused multiply and add)


impl Add for fi {
    type Output = Self;

    fn add(self, num: Self) -> Self {
        let sign1 = self.sign;
        let sign2 = num.sign;
        let mut res: fi = fi::new();
        if sign1 == true && sign2 == true {
            res = gen_add(self, num).neg();
        } else if sign1 && !sign2 {
            res = gen_sub(num, self);
        } else if !sign1 && sign2 {
            res = gen_sub(self, num);
        } else {
            res = gen_add(self, num);
        }
        res
    }

}

impl AddAssign for fi {
    fn add_assign(&mut self, other: Self) {
        *self = self.clone().add(other);
    }
}

impl Sub for fi {
    type Output = Self;

    fn sub(self, num: Self) -> Self {
        println!("yoo");
        
        self + num.invert() // change to neg once implemented
    }
}

impl SubAssign for fi {
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone().sub(other); // fix
    }
}

impl fi { // could we borrow here?? --> prob yes
    pub fn add(self, num: Self) -> Self {
        let sign1 = self.sign;
        let sign2 = num.sign;
        let mut res: fi = fi::new();
        if sign1 == true && sign2 == true {
            res = gen_add(self, num).neg();
        } else if sign1 && !sign2 {
            res = gen_sub(num, self);
        } else if !sign1 && sign2 {
            res = gen_sub(self, num);
        } else {
            res = gen_add(self, num);
        }
        res
    }

    // you are urged to use the add function; this function is slower and might be used incase the user doesn't care about the impact/doesn't read the documentation/is interested in a logical in having an easily readable code
    pub fn sub(self, num: Self) -> Self {

        self.add(num.invert())
    }

    pub fn mul(self, num: Self) -> Self { // division is required for this function (devide by a factor of 10^20 at the end)
        let res = gen_mul(self, num);
        gen_div(res, fi{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, true]})
    }

    pub fn div(self, num: fi) -> fi {
        let dividend = gen_mul(self, fi{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, true]});
        gen_div(dividend, num)
    }

    // pub fn rem(self, num: Self) -> Self {

    // }
}


// inline??


fn gen_add(num1: fi, num2: fi) -> fi { 
    let mut res: Vec<bool> = Vec::new();
    let mut carry: bool = false;
    let mut val1: Vec<bool> = num1.value;
    let mut val2: Vec<bool> = num2.value;
    if val1.len() > val2.len() {
        for i in val2.len()..val1.len() {
            val2.push(false);
        }
    } else if val2.len() > val1.len() {
        for i in val1.len()..val2.len() {
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
    fi{sign: false, value: res}

}

fn gen_sub(num1: fi, num2: fi) -> fi {
    let mut sign: bool;
    let mut large;
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
    for i in small.len()..=large.len() {
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
    fi{sign: sign, value: value}
}

fn gen_mul(num1: fi, num2: fi) -> fi {
    let sign: bool;
    let mut val1: Vec<bool> = num1.value;
    let mut val2: Vec<bool> = num2.value;
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
    fi{sign: sign, value: res}
}

fn gen_div(num1: fi, num2: fi) -> fi {
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
    let mut r: fi = fi{sign: false, value: vec![false]};
    for i in (0..num1.len()).rev() {
        r.value.insert(0, num1.value[i]);
        // println!("r: {:?}", r);
        
        if r >= num2.abs() {
            r -= num2.abs();
            q[i] = true;
        }
        
    }   
    let mut res = fi{sign: sign, value: q};
    let double = gen_mul(r, fi{sign: false, value: vec![false, true]});
    if double >= res.abs() {
        res += fi{sign: sign, value: vec![true]};
    }
    res
}


// fn gen_rem(num1: fi, num2: fi) -> fi {
//     let sign;
//     if num1.sign == num2.sign {
//         sign = false;
//     } else {
//         sign = true;
//     }
//     let d = num2.abs();
//     if num2.is_zero() {
//         panic!("You can't divide by 0. Make sure your dividend is not equal to 0.")
//     }
//     let mut q: Vec<bool> = vec![false];
//     let mut r: fi = fi{vec![false];
//     for i in (0..num1.len()).rev() {
//         r.value.insert(0, n.value[i]);
//         if r < d {
//             r -= d;
//             q.insert(0, true);
//         }
//     }
//     let mut res = fi{sign: sign, value: r.value}; // figure out what to do with the sign
//     res
// }

