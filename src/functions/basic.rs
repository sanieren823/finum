
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
    pub fn invert_bits(mut self) -> fi {
        for i in 0..self.value.len() {
            self.value[i] ^= true;
        }
        self.invert();
        self
    }
    
    pub fn abs(&self) -> fi {
        fi{sign: false, value: self.value.clone()}
    }

    pub fn neg(&self) -> fi {
        fi{sign: true, value: self.value.clone()}
    }

    pub fn invert(&self) -> fi {
        fi{sign: !self.sign.clone(), value: self.value.clone()}
    }
    
    pub fn len(&self) -> usize {
        self.value.len()
    }

}