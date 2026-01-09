
use crate::fi::{FiBin, FiLong};
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

impl FiBin {
    pub fn invert_bits(mut self) -> Self { // remove mut
        for i in 0..self.value.len() {
            self.value[i] ^= true;
        }
        self.invert();
        self
    }
    
    pub fn abs(&self) -> Self {
        FiBin{sign: false, value: self.value.clone()}
    }

    pub fn neg(&self) -> Self {
        FiBin{sign: true, value: self.value.clone()}
    }

    // dublicate of Neg
    pub fn invert(&self) -> Self {
        FiBin{sign: !self.sign.clone(), value: self.value.clone()}
    }
    
    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn is_zero(&self) -> bool {
        self.spruce_up().value == Vec::new()
    }

    pub fn is_integer(&self) -> bool {
        (self.clone() % FiBin::from(1)).is_zero()
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
        FiBin{sign: self.sign, value: output}
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
    pub fn pop(&mut self, index: usize) -> Option<bool> {
        self.value.pop()
    }

    pub fn iter(&self) -> Iter<'_, bool> {
        self.value.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, bool> {
        self.value.iter_mut()
    }
    pub fn decimals() -> Self { // TODO: find a better name
        FiBin{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]}
    }
    pub fn one() -> Self {
        FiBin{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, true]}
    }
    // TODO: implement method for every number between -10 and 10 + sqrt(2) + sqrt(3) + sqrt(5) + e + pi + e^2 + pi^2 + e/2 + pi/2 + pi/3 + pi/4 + 2pi + ln(2) + ln(3) + ln(5)

}

impl FiLong {
    pub fn new() -> Self {
        FiLong{sign: false, value: Vec::new()}
    }
    pub fn abs(self) -> Self {
        FiLong{sign: false, value: self.value}
    }
    pub fn absolute(&self) -> FiLong { // this function is somewhat a dublicate of the .abs() but i need it to work with references so there it is
        FiLong{sign: false, value: self.value.clone()}
    }
    pub fn spruce_up(&self) -> Self {
        let mut output: Vec<u64> = self.value.clone();
        for el in self.value.iter().rev() {
            if *el != 0 {
                break;
            }
            output.pop();
        }
        FiLong{sign: self.sign, value: output}
    }

    pub fn len(&self) -> usize {
        self.value.len()
    }

    pub fn push(&mut self, other: u64) {
        self.value.push(other);
    }

    pub fn insert(&mut self, index: usize, elem: u64) {
        self.value.insert(index, elem);
    }

    pub fn remove(&mut self, index: usize) -> u64 {
        self.value.remove(index)
    }

    pub fn pop(&mut self, index: usize) -> Option<u64> {
        self.value.pop()
    }

    pub fn last(self) -> Option<u64> {
        self.value.last().copied()
    }

    pub fn is_zero(&self) -> bool {
        self.spruce_up().value == Vec::new()
    }

    pub fn is_integer(&self) -> bool {
        self % FiLong::one() == FiLong::new()
    }

    pub fn iter(&self) -> Iter<'_, u64> {
        self.value.iter()
    }

    pub fn reverse(&mut self) {
        self.value.reverse();
    }

    pub fn reverse_bits(self) -> Self {
        let mut res: FiLong = FiLong::new();
        for el in self.iter().rev() {
            res.push(el.reverse_bits());
        }
        res
    }

    pub fn decimals() -> Self {
        FiLong {sign: false, value: vec![13399722918938673152, 7145508105175220139, 29]}
    }

    pub fn one() -> Self {
        FiLong{sign: false, value: vec![7766279631452241920, 5]}
    }

    pub fn two() -> Self {
        FiLong{sign: false, value: vec![15532559262904483840, 10]}
    }

    pub fn thousand() -> Self {
        FiLong {sign: false, value: vec![200376420520689664, 5421]}
    }

    pub fn thousandth() -> Self {
        FiLong {sign: false, value: vec![100000000000000000]}
    }

    pub fn billion() -> Self {
        FiLong {sign: false, value: vec![7886392056514347008, 5421010862]}
    }

    pub fn billionth() -> Self {
        FiLong {sign: false, value: vec![100000000000]}
    }

    pub fn e() -> Self {
        FiLong {sign: false, value: vec![13573765813970800912, 14]}
    }
}


impl Clone for FiBin {
    fn clone(&self) -> Self {
        FiBin{sign: self.sign.clone(), value: self.value.clone()}
    }
}

impl Index<usize> for FiBin {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for FiBin {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl Clone for FiLong {
    fn clone(&self) -> Self {
        FiLong{sign: self.sign.clone(), value: self.value.clone()}
    }
}

impl Index<usize> for FiLong {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for FiLong {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}