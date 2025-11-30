
use crate::fi::fi;

// abs: --> positive; neg: --> negative; invert --> inverts --> postive if negative/negative if positive
// invert_bits


// // match?
// pub fn is_zero(&self) -> bool {
//     for el in self {
//         if el.count_zeros() > 0 {
//             true
//         }
//     }
//     false
// }


// pub fn count_zeros(&self) -> u32 {

// }

// pub fn count_ones(&self) -> u32 {
    
// }

// pub fn leading_zeros(&self) -> u32 {
    
// }

// pub fn leading_ones(&self) -> u32 {
    
// }

// pub fn trailing_zeros(&self) -> u32 {
    
// }

// pub fn trailing_ones(&self) -> u32 {
    
// }

// pub fn isolate_most_significant_one(&self) -> Self {

// }

// pub fn isolate_least_significant_one(&self) -> Self {
    
// }

impl fi {
    pub fn invert_bits(mut self) -> Self { // remove mut
        for i in 0..self.value.len() {
            self.value[i] ^= true;
        }
        self.invert();
        self
    }
    
    pub fn abs(&self) -> Self {
        fi{sign: false, value: self.value.clone()}
    }

    pub fn neg(&self) -> Self {
        fi{sign: true, value: self.value.clone()}
    }

    // dublicate of Neg
    pub fn invert(&self) -> Self {
        fi{sign: !self.sign.clone(), value: self.value.clone()}
    }
    
    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn is_zero(&self) -> bool {
        self.spruce_up().value == Vec::new()
    }

    pub fn spruce_up(&self) -> Self {
        let mut valid = false;
        let mut output = Vec::new();
        for el in self.value.iter().rev() {
            if *el {
                valid = true;
                output.push(*el)
            } else {
                if valid {
                    output.push(*el);
                }
            }
        }
        output.reverse();
        fi{sign: self.sign, value: output}
    }

}

impl Clone for fi {
    fn clone(&self) -> Self {
        fi{sign: self.sign.clone(), value: self.value.clone()}
    }
}