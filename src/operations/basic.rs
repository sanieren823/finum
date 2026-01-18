
use crate::finum::{FiBin, FiLong};
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
    #[inline(always)]
    pub fn new() -> Self {
        FiLong{sign: false, value: Vec::new()}
    }

    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self {
        FiLong{sign: false, value: Vec::with_capacity(capacity)}
    }

    #[inline(always)]
    pub fn resize(&mut self, new_len: usize, value: u64) {
        self.value.resize(new_len, value);
    }

    #[inline(always)]
    pub fn abs(self) -> Self {
        FiLong{sign: false, value: self.value}
    }

    #[inline(always)]
    pub fn absolute(&self) -> FiLong { // this function is somewhat a dublicate of the .abs() but i need it to work with references so there it is
        FiLong{sign: false, value: self.value.clone()}
    }

    #[inline(always)]
    pub fn inverse(&self) -> FiLong {
        FiLong::one() / self
    }

    #[inline(always)]
    pub fn bits(&self) -> u32 {
        let len = self.spruce_up().len();
        self[len - 1].ilog2() + 64u32 * (len as u32 - 1)
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.value.len()
    }

    #[inline(always)]
    pub fn push(&mut self, other: u64) {
        self.value.push(other);
    }

    #[inline(always)]
    pub fn insert(&mut self, index: usize, elem: u64) {
        self.value.insert(index, elem);
    }

    #[inline(always)]
    pub fn remove(&mut self, index: usize) -> u64 {
        self.value.remove(index)
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<u64> {
        self.value.pop()
    }
    
    #[inline(always)]
    pub fn last(self) -> Option<u64> {
        self.value.last().copied()
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.spruce_up().value == Vec::new()
    }

    #[inline(always)]
    pub fn is_integer(&self) -> bool {
        (self % FiLong::one()).is_zero()
    }

    #[inline(always)]
    pub fn iter(&self) -> Iter<'_, u64> {
        self.value.iter()
    }

    #[inline(always)]
    pub fn reverse(&mut self) {
        self.value.reverse();
    }

    #[inline(always)]
    pub fn reverse_bits(self) -> Self {
        let mut res: FiLong = FiLong::new();
        for el in self.iter().rev() {
            res.push(el.reverse_bits());
        }
        res
    }

    #[inline(always)]
    pub fn decimals() -> Self {
        FiLong {sign: false, value: vec![13399722918938673152, 7145508105175220139, 29]}
    }
    
    #[inline(always)]
    pub fn decimal_part(&self) -> Self {
        self % FiLong::one()
    }

    #[inline(always)]
    pub fn value_of_sign(self, s: &bool) -> Self {
        FiLong{sign: *s, value: self.value}
    }

    #[inline(always)]
    pub fn gen_increment(&mut self, s: bool) {
        *self += FiLong{sign: s, value: vec![7766279631452241920, 5]};
    }

    #[inline(always)]
    pub fn gen_decrement(&mut self, s: bool) {
        *self -= FiLong{sign: s, value: vec![7766279631452241920, 5]};
    }

    #[inline(always)]
    pub fn one() -> Self {
        FiLong{sign: false, value: vec![7766279631452241920, 5]}
    }

    #[inline(always)]
    pub fn neg_one() -> Self {
        FiLong{sign: true, value: vec![7766279631452241920, 5]}
    }

    #[inline(always)]
    pub fn two() -> Self {
        FiLong{sign: false, value: vec![15532559262904483840, 10]}
    }

    #[inline(always)]
    pub fn neg_two() -> Self {
        FiLong{sign: true, value: vec![15532559262904483840, 10]}
    }

    #[inline(always)]
    pub fn three() -> Self {
        FiLong{sign: false, value: vec![4852094820647174144, 16]}
    }
    
    #[inline(always)]
    pub fn neg_three() -> Self {
        FiLong{sign: true, value: vec![4852094820647174144, 16]}
    }

    #[inline(always)]
    pub fn four() -> Self {
        FiLong{sign: false, value: vec![12618374452099416064, 21]}
    }

    #[inline(always)]
    pub fn neg_four() -> Self {
        FiLong{sign: true, value: vec![12618374452099416064, 21]}
    }

    #[inline(always)]
    pub fn five() -> Self {
        FiLong{sign: false, value: vec![1937910009842106368, 27]}
    }

    #[inline(always)]
    pub fn neg_five() -> Self {
        FiLong{sign: true, value: vec![1937910009842106368, 27]}
    }

    #[inline(always)]
    pub fn six() -> Self {
        FiLong{sign: false, value: vec![9704189641294348288, 32]}
    }

    #[inline(always)]
    pub fn neg_six() -> Self {
        FiLong{sign: true, value: vec![9704189641294348288, 32]}
    }

    #[inline(always)]
    pub fn seven() -> Self {
        FiLong{sign: false, value: vec![17470469272746590208, 37]}
    }

    #[inline(always)]
    pub fn neg_seven() -> Self {
        FiLong{sign: true, value: vec![17470469272746590208, 37]}
    }

    #[inline(always)]
    pub fn eight() -> Self {
        FiLong{sign: false, value: vec![6790004830489280512, 43]}
    }

    #[inline(always)]
    pub fn neg_eight() -> Self {
        FiLong{sign: true, value: vec![6790004830489280512, 43]}
    }

    #[inline(always)]
    pub fn nine() -> Self {
        FiLong{sign: false, value: vec![14556284461941522432, 48]}
    }

    #[inline(always)]
    pub fn neg_nine() -> Self {
        FiLong{sign: true, value: vec![14556284461941522432, 48]}
    }

    #[inline(always)]
    pub fn ten() -> Self {
        FiLong{sign: false, value: vec![3875820019684212736, 54]}
    }

    #[inline(always)]
    pub fn neg_ten() -> Self {
        FiLong{sign: true, value: vec![3875820019684212736, 54]}
    }

    #[inline(always)]
    pub fn tenth() -> Self {
        FiLong{sign: false, value: vec![10000000000000000000]}
    }

    #[inline(always)]
    pub fn hundred() -> Self {
        FiLong{sign: false, value: vec![1864712049423024128, 542]}
    }

    #[inline(always)]
    pub fn hundredth() -> Self {
        FiLong{sign: false, value: vec![1000000000000000000]}
    }

    #[inline(always)]
    pub fn thousand() -> Self {
        FiLong{sign: false, value: vec![200376420520689664, 5421]}
    }

    #[inline(always)]
    pub fn thousandth() -> Self {
        FiLong{sign: false, value: vec![100000000000000000]}
    }

    #[inline(always)]
    pub fn million() -> Self {
        FiLong{sign: false, value: vec![15908979783594147840, 5421010]}
    }

    #[inline(always)]
    pub fn millionth() -> Self {
        FiLong{sign: false, value: vec![100000000000000]}
    }

    #[inline(always)]
    pub fn billion() -> Self {
        FiLong{sign: false, value: vec![7886392056514347008, 5421010862]}
    }

    #[inline(always)]
    pub fn billionth() -> Self {
        FiLong{sign: false, value: vec![100000000000]}
    }

    #[inline(always)]
    pub fn trillion() -> Self {
        FiLong{sign: false, value: vec![9632337040368467968, 5421010862427]}
    }

    #[inline(always)]
    pub fn trillionth() -> Self {
        FiLong{sign: false, value: vec![100000000]}
    }

    #[inline(always)]
    pub fn pi() -> Self {
        FiLong{sign: false, value: vec![564616105916946374, 17]}
    }

    #[inline(always)]
    pub fn two_pi() -> Self {
        FiLong{sign: false, value: vec![1129232211833892748, 34]}
    }

    #[inline(always)]
    pub fn pi_div_two() -> Self {
        FiLong{sign: false, value: vec![9505680089813248995, 8]}
    }

    #[inline(always)]
    pub fn pi_div_three() -> Self {
        FiLong{sign: false, value: vec![12486034751112016535, 5]}
    }

    #[inline(always)]
    pub fn pi_div_four() -> Self {
        FiLong{sign: false, value: vec![4752840044906624498, 4]}
    }

    #[inline(always)]
    pub fn pi_div_five() -> Self {
        FiLong{sign: false, value: vec![7491620850667209921, 3]}
    }

    #[inline(always)]
    pub fn pi_div_six() -> Self {
        FiLong{sign: false, value: vec![15466389412410784076, 2]}
    }

    #[inline(always)]
    pub fn pi_div_eight() -> Self {
        FiLong{sign: false, value: vec![2376420022453312249, 2]}
    }

    #[inline(always)]
    pub fn pi_squared() -> Self {
        FiLong{sign: false, value: vec![9283004202329626235, 53]}
    }

    #[inline(always)]
    pub fn pi_recip() -> Self {
        FiLong{sign: false, value: vec![13384244544669515538, 1]}
    }

    #[inline(always)]
    pub fn e() -> Self {
        FiLong{sign: false, value: vec![13573765813970800912, 14]}
    }

    #[inline(always)]
    pub fn half_e() -> Self {
        FiLong {sign: false, value: vec![6786882906985400456, 7] }
    }

    #[inline(always)]
    pub fn e_squared() -> Self {
        FiLong{sign: false, value: vec![1035846944682958083, 40]}
    }

    #[inline(always)]
    pub fn e_recip() -> Self {
        FiLong{sign: false, value: vec![18341200043434680544, 1]}
    }

    #[inline(always)]
    pub fn ln2() -> Self {
        FiLong{sign: false, value: vec![13974485834865876094, 3]}
    }

    #[inline(always)]
    pub fn ln3() -> Self {
        FiLong{sign: false, value: vec![17627508498263211060, 5]}
    }

    #[inline(always)]
    pub fn ln5() -> Self {
        FiLong{sign: false, value: vec![13369838653733624532, 8]}
    }

    #[inline(always)]
    pub fn ln10() -> Self {
        FiLong{sign: false, value: vec![8897580414889949010, 12]}
    }

    #[inline(always)]
    pub fn one_half() -> Self {
        FiLong{sign: false, value: vec![13106511852580896768, 2]}
    }

    #[inline(always)]
    pub fn one_third() -> Self {
        FiLong{sign: false, value: vec![14886589259623781717, 1]}
    }

    #[inline(always)]
    pub fn one_quarter() -> Self {
        FiLong{sign: false, value: vec![6553255926290448384, 1]}
    }

    #[inline(always)]
    pub fn one_fifth() -> Self {
        FiLong{sign: false, value: vec![1553255926290448384, 1]}
    }

    #[inline(always)]
    pub fn one_eighth() -> Self {
        FiLong{sign: false, value: vec![12500000000000000000]}
    }

    #[inline(always)]
    pub fn sqrt2() -> Self {
        FiLong{sign: false, value: vec![12294147721342643568, 7]}
    }

    #[inline(always)]
    pub fn sqrt3() -> Self {
        FiLong{sign: false, value: vec![7184384093501764809, 9]}
    }

        #[inline(always)]
    pub fn sqrt5() -> Self {
        FiLong{sign: false, value: vec![15532559262904483840, 10]}
    }

    #[inline(always)]
    pub fn sqrt10() -> Self {
        FiLong{sign: false, value: vec![2633116763775555728, 17]}
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