use crate::finum::{FiBin, FiLong};
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

// FiLong

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

        let mut output = Vec::with_capacity(self.len() + full + 1);
        output.extend(vec![0; full]);

        if exact == 0 {
            output.extend_from_slice(&self.value);
            return FiLong { sign: self.sign, value: output };
        }
        output
            .extend((0..self.len())
                .map(|i| {
                        let mut val: u64 = self[i] << exact;
                        if i > 0 {
                            val |= self[i - 1] >> (64 - exact);
                        }
                        val
                    }));
        let last = self[self.len() - 1] >> (64 - exact);
        if last != 0 { // if you remove this if clause the compute time suddenly jumps to 7µs compared to around 1µs; it seems odd because this clause is only there for performance reasons; i checked both parts (the push, the last computation) and they take no longer than 100ns seperately (or together)
            output.push(last);
        }
        FiLong { sign: self.sign, value: output }
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
        let full = rhs / 64;
        let exact = rhs % 64;
        let mut output = Vec::with_capacity(self.len() + full + 1);
        output.extend(vec![0; full]);

        if exact == 0 {
            output.extend_from_slice(&self.value);
            *self = FiLong {sign: self.sign, value: output}
        } else {
            output
                .extend((0..self.len())
                    .map(|i| {
                            let mut val: u64 = self[i] << exact;
                            if i > 0 {
                                val |= self[i - 1] >> (64 - exact);
                            }
                            val
                        }));
            let last = self[self.len() - 1] >> (64 - exact);
            if last != 0 { // if you remove this if clause the compute time suddenly jumps to 7µs compared to around 1µs; it seems odd because this clause is only there for performance reasons; i checked both parts (the push, the last computation) and they take no longer than 100ns seperately (or together)
                output.push(last);
            }
            *self = FiLong {sign: self.sign, value: output};
        }
    }
}

impl ShlAssign<usize> for FiLong {

    fn shl_assign(&mut self, rhs: usize) {
        *self <<= &rhs;
    }
}

impl Shr<&usize> for &FiLong {
    type Output = FiLong;


    fn shr(self, rhs: &usize) -> Self::Output {
        if *rhs >= self.len() * 64 {
            FiLong::new()
        } else {
            let full = rhs / 64;
            let exact = rhs % 64;
            if exact == 0 {
                return FiLong{sign: self.sign, value: self.value[full..].to_vec()};
            }
            let output: Vec<u64> = (full..self.len())
                .map(|i| {
                    let mut val = self[i] >> exact;
                    if i + 1 < self.len() {
                        val |= self[i + 1] << (64 - exact);
                    }
                    val
                })
                .collect();
            FiLong{sign: self.sign, value: output}
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
        if *rhs >= self.len() * 64 {
            *self = FiLong::new();
        } else {
            let full = rhs / 64;
            let exact = rhs % 64;
            if exact == 0 {
                *self = FiLong{sign: self.sign, value: self.value[full..].to_vec()};
            } else {
                let output: Vec<u64> = (full..self.len())
                    .map(|i| {
                        let mut val = self[i] >> exact;
                        if i + 1 < self.len() {
                            val |= self[i + 1] << (64 - exact);
                        }
                        val
                    })
                    .collect();
                *self = FiLong{sign: self.sign, value: output};
            }
        }
    }
}

impl ShrAssign<usize> for FiLong {

    fn shr_assign(&mut self, rhs: usize) {
        *self >>= &rhs;
    }
}
