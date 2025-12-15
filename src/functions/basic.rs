
use crate::fi::Fi;
use std::ops::{Index, IndexMut};
use core::slice::{Iter, IterMut};

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

impl Fi {
    pub fn invert_bits(mut self) -> Self { // remove mut
        for i in 0..self.value.len() {
            self.value[i] ^= true;
        }
        self.invert();
        self
    }
    
    pub fn abs(&self) -> Self {
        Fi{sign: false, value: self.value.clone()}
    }

    pub fn neg(&self) -> Self {
        Fi{sign: true, value: self.value.clone()}
    }

    // dublicate of Neg
    pub fn invert(&self) -> Self {
        Fi{sign: !self.sign.clone(), value: self.value.clone()}
    }
    
    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn is_zero(&self) -> bool {
        self.spruce_up().value == Vec::new()
    }

    pub fn spruce_up(&self) -> Self {
        let mut valid: bool = false;
        let mut output: Vec<bool> = Vec::new();
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
        Fi{sign: self.sign, value: output}
    }
    // implement insert and remove
    pub fn push(&mut self, other: bool) {
        self.value.push(other);
    }

    pub fn insert(&mut self, index: usize, elem: bool) {
        self.value.insert(index, elem);
    }

    pub fn remove(&mut self, index: usize) -> bool {
        self.value.remove(index)
    }

    pub fn iter(&self) -> Iter<'_, bool> {
        self.value.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, bool> {
        self.value.iter_mut()
    }

}

impl Clone for Fi {
    fn clone(&self) -> Self {
        Fi{sign: self.sign.clone(), value: self.value.clone()}
    }
}

impl Index<usize> for Fi {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Fi {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

