use crate::fi::fi;
use crate::fi::bcd;
use crate::functions;

// add a mul_add (fused multiply and add)



impl fi { // could we borrow here?? --> prob yes
    pub fn add(self, num: Self) -> fi {
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
    pub fn sub(self, num: Self) -> fi {
        self.add(num.invert())
    }

    // pub fn mul(self, num: fi) -> fi { // division is required for this function (devide by a factor of 10^20 at the end)

    // }

    // pub fn div(self, num: fi) -> fi {

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
    if num1 >= num2 {
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
    if num1.sign == num2.sign {
        sign = false;
    } else {
        sign = true;
    }
    for i in 0..val1.len() {
        for j in 0..val2.len() {
            let index = i + j;
            if res.len() > index {
                if val1[i] == val2[i] && val1[i] {
                    res[index] ^= true;
                }
            } else {
                if val1[i] == val2[i] && val1[i] {
                    res.push(true);
                } else {
                    res.push(false);
                }
            }
        }
    }
    fi{sign: sign, value: res}
}

// fn gen_div(num1: fi, num2: fi) -> fi {
//     if num2.is_zero() {
//         panic!("You can't divide by 0. Make sure your dividend is not equal to 0.")
//     }
//     let mut q = 0;
//     let mut r = 0;
// }

