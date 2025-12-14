use std::cmp::Ordering;
use crate::fi::Fi;


impl PartialOrd for Fi {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fi {
    fn cmp(&self, other: &Self) -> Ordering {

        match self.sign.cmp(&other.sign) {
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => (),
        }
        let v1 = self.spruce_up();    
        let v2 = other.spruce_up();
        let len_diff: isize = v1.len() as isize - v2.len() as isize;

        let mut b = false; // fix
        if len_diff == 0 {
            let len = v1.len();
            if len == 0 {
                panic!("Make sure that your fi-object has a value.");
            }
            for i in 1..=len {
                if v1.value[len - i] != v2.value[len - i] {
                    b = v1.value[len - i];
                    break;
                } else if i == len {
                    return Ordering::Equal;
                }
            }
        } else {
            b = heaviside(&len_diff);
        }
        b ^= self.sign;
        if b {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq for Fi {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.sign == other.sign
    }
}

impl Eq for Fi {}

fn heaviside(num: &isize) -> bool {
    if *num < 0 {
        false
    } else {
        true
    }
}



impl Fi {
    fn pretty(&self) -> Fi {
        let mut valid = false;
        let mut output = Vec::new();
        for el in self.value.clone().iter().rev(){
            if *el {
                valid = true;
                output.push(*el);
            } else {
                if valid {
                    output.push(*el);
                } 
                
            }
        }
        output.reverse();
        Fi{sign: self.sign.clone(), value: output}
    }
}
