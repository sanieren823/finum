use crate::fi::fi;
use crate::fi::bcd;
use crate::functions;





impl fi {
    pub fn add(self, num: fi) -> fi {
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
    pub fn sub(self, num: fi) -> fi {
        self.add(num.invert())
    }
}

fn gen_add(num1: fi, num2: fi) -> fi { // incorrect
    let len: usize;
    let mut res: Vec<bool> = Vec::new();
    let mut carry: bool = false;
    let mut val1: Vec<bool> = num1.value;
    let mut val2: Vec<bool> = num2.value;
    if val1.len() > val2.len() {
        len = val1.len();
        for i in val2.len()..len {
            val2.push(false);
        }
    } else {
        len = val2.len();
        for i in val1.len()..len {
            val1.push(false);
        }
    }
    for i in 0..len { 
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
    // figure out what the larger one is -> normal subtraction + adjust the sign approriately
    let mut sign: bool;
    let mut biggest;
    let mut smallest;
    if num1.value.len() > num2.value.len() {
        smallest = num2.value;
        biggest = num1.value;
        sign = num1.sign;
    } else {
        smallest = num1.value;
        biggest = num2.value;
        sign = num2.sign;
    }
    for i in smallest.len()..=biggest.len() {
        smallest.push(false);
    }
    let mut value: Vec<bool> = Vec::new();
    let mut borrow: bool = false;
    for i in 0..biggest.len() {
        if biggest[i] == smallest[i] && !borrow {
            value.push(false);
        } else if borrow {
            if biggest[i] == smallest[i] {
                value.push(true); 
            } else  {
                if biggest[i] {
                    borrow = false;
                }
                value.push(false);

            }
        } else {
            if smallest[i] {
                borrow = true;
            }
            value.push(true); 
        }
    }
    fi{sign: sign, value: value}
}


