use crate::fi::{FiBin, FiLong};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

// TODO: write tests for all bitwise functions



impl BitAnd for FiBin {
    type Output = Self;


    fn bitand(self, rhs: Self) -> Self::Output {
        let mut new = FiBin::new();
        new.sign = self.sign & rhs.sign;
        let mut smallest: FiBin;
        let biggest: FiBin; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(false);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] & biggest[i]);
        }
        new
    }
}

impl BitAndAssign for FiBin {

    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.clone() & rhs
    }
}

impl BitOr for FiBin {
    type Output = Self;


    fn bitor(self, rhs: Self) -> Self::Output {
        let mut new = FiBin::new();
        new.sign = self.sign | rhs.sign;
        let mut smallest: FiBin;
        let biggest: FiBin; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(false);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] | biggest[i]);
        }
        new
    }
}

impl BitOrAssign for FiBin {

    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.clone() | rhs
    }
}

impl BitXor for FiBin {
    type Output = Self;


    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut new = FiBin::new();
        new.sign = self.sign ^ rhs.sign;
        let mut smallest: FiBin;
        let biggest: FiBin; 
        if self.len() >= rhs.len() {
            smallest = rhs;
            biggest = self;
        } else {
            smallest = self;
            biggest = rhs;
        }
        for _ in smallest.len()..biggest.len() {
            smallest.push(false);
        }
        for i in 0..biggest.len() {
            new.push(smallest[i] ^ biggest[i]);
        }
        new
    }
}

impl BitXorAssign for FiBin {

    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.clone() ^ rhs
    }
}


impl Shl<usize> for FiBin {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut fi = self;
        for _ in 0..rhs {
            fi.insert(0, false);
        }
        fi
    }
}

impl ShlAssign<usize> for FiBin {
    fn shl_assign(&mut self, rhs: usize) {
        *self = self.clone() << rhs;
    }
}


impl Shr<usize> for FiBin {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut fi = self;
        for i in 0..rhs {
            if i >= fi.len() {
                break;
            }
            fi.remove(0);
        }
        fi
    }
}

impl ShrAssign<usize> for FiBin {
    fn shr_assign(&mut self, rhs: usize) {
        *self = self.clone() >> rhs;
    }
}



















// impl BitAnd for FiLong {
//     type Output = Self;


//     fn bitand(self, rhs: Self) -> Self::Output {
//         let mut new = FiLong::new();
//         new.sign = self.sign & rhs.sign;
//         let mut smallest: FiLong;
//         let biggest: FiLong; 
//         if self.len() >= rhs.len() {
//             smallest = rhs;
//             biggest = self;
//         } else {
//             smallest = self;
//             biggest = rhs;
//         }
//         for _ in smallest.len()..biggest.len() {
//             smallest.push(0);
//         }
//         for i in 0..biggest.len() {
//             new.push(smallest[i] & biggest[i]);
//         }
//         new
//     }
// }


// impl BitOr for FiLong {
//     type Output = Self;


//     fn bitor(self, rhs: Self) -> Self::Output {
//         let mut new = FiLong::new();
//         new.sign = self.sign & rhs.sign;
//         let mut smallest: FiLong;
//         let biggest: FiLong; 
//         if self.len() >= rhs.len() {
//             smallest = rhs;
//             biggest = self;
//         } else {
//             smallest = self;
//             biggest = rhs;
//         }
//         for _ in smallest.len()..biggest.len() {
//             smallest.push(0);
//         }
//         for i in 0..biggest.len() {
//             new.push(smallest[i] | biggest[i]);
//         }
//         new
//     }
// }

// impl BitOrAssign for FiLong {

//     fn bitor_assign(&mut self, rhs: Self) {
//         *self = self.clone() | rhs
//     }
// }

// impl BitXor for FiLong {
//     type Output = Self;


//     fn bitxor(self, rhs: Self) -> Self::Output {
//         let mut new = FiLong::new();
//         new.sign = self.sign & rhs.sign;
//         let mut smallest: FiLong;
//         let biggest: FiLong; 
//         if self.len() >= rhs.len() {
//             smallest = rhs;
//             biggest = self;
//         } else {
//             smallest = self;
//             biggest = rhs;
//         }
//         for _ in smallest.len()..biggest.len() {
//             smallest.push(0);
//         }
//         for i in 0..biggest.len() {
//             new.push(smallest[i] ^ biggest[i]);
//         }
//         new
//     }
// }

// impl BitXorAssign for FiLong {

//     fn bitxor_assign(&mut self, rhs: Self) {
//         *self = self.clone() ^ rhs
//     }
// }


// impl Shl<usize> for FiLong {
//     type Output = Self;

//     fn shl(self, rhs: usize) -> Self::Output {
//         let full = rhs / 64;
//         let exact = rhs % 64;
//         let mut output: Vec<u64> = Vec::with_capacity(self.len() + full + 1);
//         for _ in 0..full {
//             output.push(0);
//         }
//         output.push(0);
//         for i in 0..self.len() {
//             let this = self[i] << exact;
//             if exact == 0 {
//                 output[i + full] |= this;
//                 break;
//             }
            
//             let next = self[i] >> (64 - exact);
//             output[i + full] |= this;
//             if next > 0  || i + 1 < self.len() {
//                 output.push(next);
//             }
            
            
//         }
//         FiLong{sign: self.sign, value: output}
//     }
// }

// impl ShlAssign<usize> for FiLong {
//     fn shl_assign(&mut self, rhs: usize) {
//         *self = self.clone() << rhs;
//     }
// }


// impl Shr<usize> for FiLong {
//     type Output = Self;

//     fn shr(self, rhs: usize) -> Self::Output {
//         if rhs >= self.len() * 64 {
//             FiLong::new()
//         } else {
//             let mut fi = self;
//             let full = rhs / 64;
//             let exact = rhs % 64;
//             for _ in 0..full {
//                 fi.remove(0);
//             }
//             let mut output: Vec<u64> = Vec::with_capacity(fi.value.capacity());
//             for i in 0..fi.len() {
//                 if exact == 0 {
//                     break;
//                 }
//                 let this = fi[i] >> exact;
//                 let prev = fi[i] << (64 - exact);
//                 output.push(this);
//                 if i > 0 {
//                     output[i - 1] |= prev;
//                 }
//             }
//             FiLong{sign: fi.sign, value: output}
//         }
//     }
// }

// impl ShrAssign<usize> for FiLong {
//     fn shr_assign(&mut self, rhs: usize) {
//         *self = self.clone() >> rhs;
//     }
// }

impl BitAnd<&FiLong> for &FiLong {
    type Output = FiLong;


    fn bitand(self, rhs: &FiLong) -> Self::Output {
        let mut new = FiLong::new();
        new.sign = self.sign & rhs.sign;
        let (smaller, bigger) = if self.len() >= rhs.len() {
            (rhs, self)
        } else {
            (self, rhs)
        };
        
        for i in 0..smaller.len() {
            new.push(smaller[i] & bigger[i]);
        }
        for _ in smaller.len()..bigger.len() {
            new.push(0);
        }
        new
    }
}

impl BitAnd<&FiLong> for FiLong {
    type Output = FiLong;


    fn bitand(self, rhs: &FiLong) -> Self::Output {
        &self & rhs
    }
}

impl BitAnd<FiLong> for &FiLong {
    type Output = FiLong;


    fn bitand(self, rhs: FiLong) -> Self::Output {
        self & &rhs
    }
}

impl BitAnd<FiLong> for FiLong {
    type Output = FiLong;


    fn bitand(self, rhs: FiLong) -> Self::Output {
        &self & &rhs
    }
}

impl BitAndAssign<&FiLong> for FiLong {

    fn bitand_assign(&mut self, rhs: &FiLong) {
        *self = self.clone() & rhs;
    }
}

impl BitAndAssign<FiLong> for FiLong {

    fn bitand_assign(&mut self, rhs: FiLong) {
        *self = self.clone() & rhs;
    }
}



impl BitOr<&FiLong> for &FiLong {
    type Output = FiLong;


    fn bitor(self, rhs: &FiLong) -> Self::Output {
       let mut new = FiLong::new();
        new.sign = self.sign | rhs.sign;
        let (smaller, bigger) = if self.len() >= rhs.len() {
            (rhs, self)
        } else {
            (self, rhs)
        };
        
        for i in 0..smaller.len() {
            new.push(smaller[i] | bigger[i]);
        }
        for i in smaller.len()..bigger.len() {
            new.push(bigger[i]);
        }
        new
    }
}

impl BitOr<&FiLong> for FiLong {
    type Output = FiLong;


    fn bitor(self, rhs: &FiLong) -> Self::Output {
        &self | rhs
    }
}

impl BitOr<FiLong> for &FiLong {
    type Output = FiLong;


    fn bitor(self, rhs: FiLong) -> Self::Output {
        self | &rhs
    }
}

impl BitOr<FiLong> for FiLong {
    type Output = FiLong;


    fn bitor(self, rhs: FiLong) -> Self::Output {
        &self | &rhs
    }
}

impl BitOrAssign<&FiLong> for FiLong {

    fn bitor_assign(&mut self, rhs: &FiLong) {
        *self = self.clone() | rhs;
    }
}

impl BitOrAssign<FiLong> for FiLong {

    fn bitor_assign(&mut self, rhs: FiLong) {
        *self = self.clone() | rhs;
    }
}


impl BitXor<&FiLong> for &FiLong {
    type Output = FiLong;


    fn bitxor(self, rhs: &FiLong) -> Self::Output {
       let mut new = FiLong::new();
        new.sign = self.sign ^ rhs.sign;
        let (smaller, bigger) = if self.len() >= rhs.len() {
            (rhs, self)
        } else {
            (self, rhs)
        };
        
        for i in 0..smaller.len() {
            new.push(smaller[i] ^ bigger[i]);
        }
        for i in smaller.len()..bigger.len() {
            new.push(bigger[i] ^ 0);
        }
        new
    }
}

impl BitXor<&FiLong> for FiLong {
    type Output = FiLong;


    fn bitxor(self, rhs: &FiLong) -> Self::Output {
        &self ^ rhs
    }
}

impl BitXor<FiLong> for &FiLong {
    type Output = FiLong;


    fn bitxor(self, rhs: FiLong) -> Self::Output {
        self ^ &rhs
    }
}

impl BitXor<FiLong> for FiLong {
    type Output = FiLong;


    fn bitxor(self, rhs: FiLong) -> Self::Output {
        &self ^ &rhs
    }
}

impl BitXorAssign<&FiLong> for FiLong {

    fn bitxor_assign(&mut self, rhs: &FiLong) {
        *self = self.clone() ^ rhs;
    }
}

impl BitXorAssign<FiLong> for FiLong {

    fn bitxor_assign(&mut self, rhs: FiLong) {
        *self = self.clone() ^ rhs;
    }
}

impl Shl<&usize> for &FiLong {
    type Output = FiLong;


    fn shl(self, rhs: &usize) -> Self::Output {
        let full = rhs / 64;
        let exact = rhs % 64;
        let mut output: Vec<u64> = Vec::with_capacity(self.len() + full + 1);
        for _ in 0..full {
            output.push(0);
        }
        output.push(0);
        for i in 0..self.len() {
            let this = self[i] << exact;
            if exact == 0 {
                output[i + full] |= this;
                break;
            }
            
            let next = self[i] >> (64 - exact);
            output[i + full] |= this;
            if next > 0  || i + 1 < self.len() {
                output.push(next);
            }
            
            
        }
        FiLong{sign: self.sign, value: output}
    }
}

impl Shl<&usize> for FiLong {
    type Output = FiLong;


    fn shl(self, rhs: &usize) -> Self::Output {
        &self << rhs
    }
}

impl Shl<usize> for &FiLong {
    type Output = FiLong;


    fn shl(self, rhs: usize) -> Self::Output {
        self << &rhs
    }
}

impl Shl<usize> for FiLong {
    type Output = FiLong;


    fn shl(self, rhs: usize) -> Self::Output {
        &self << &rhs
    }
}

impl ShlAssign<&usize> for FiLong {

    fn shl_assign(&mut self, rhs: &usize) {
        *self = self.clone() << rhs;
    }
}

impl ShlAssign<usize> for FiLong {

    fn shl_assign(&mut self, rhs: usize) {
        *self = self.clone() << rhs;
    }
}

impl Shr<&usize> for &FiLong {
    type Output = FiLong;


    fn shr(self, rhs: &usize) -> Self::Output {
        if *rhs >= self.len() * 64 {
            FiLong::new()
        } else {
            let mut fi = self.clone();
            let len = self.len();
            let full = rhs / 64;
            let exact = rhs % 64;
            let mut output: Vec<u64> = Vec::with_capacity(len - full);
            for i in full..len {
                output.push(self[i]);
            }
            
            for i in 0..output.len() {
                if exact == 0 {
                    break;
                }
                let prev = output[i] << (64 - exact);
                output[i] >>= exact;
                if i > 0 {
                    output[i - 1] |= prev;
                }
            }
            FiLong{sign: fi.sign, value: output}
        }
    }
}

impl Shr<&usize> for FiLong {
    type Output = FiLong;


    fn shr(self, rhs: &usize) -> Self::Output {
        &self >> rhs
    }
}

impl Shr<usize> for &FiLong {
    type Output = FiLong;


    fn shr(self, rhs: usize) -> Self::Output {
        self >> &rhs
    }
}

impl Shr<usize> for FiLong {
    type Output = FiLong;


    fn shr(self, rhs: usize) -> Self::Output {
        &self >> &rhs
    }
}

impl ShrAssign<&usize> for FiLong {

    fn shr_assign(&mut self, rhs: &usize) {
        *self = self.clone() >> rhs;
    }
}

impl ShrAssign<usize> for FiLong {

    fn shr_assign(&mut self, rhs: usize) {
        *self = self.clone() >> rhs;
    }
}
