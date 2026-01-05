use crate::fi::{FiBin, FiLong};
use crate::errors::FiError;
use crate::errors::FiErrorKind;

// TODO: add a function that converts every number to it's usual binary representaiotn in FiBin --> 6 --> 110 (not a 40 digit long vec)
trait Numeric {
    const BITS: u32;
    const MIN: Self;
    const MAX: Self;

    fn new() -> Self;
    fn two() -> Self;
    fn u8_to_numeric(val: u8) -> Self;
    fn pow(self, n: u32) -> Self;
    fn add(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn is_zero(self) -> bool;
    fn neg(&mut self);
    fn to_u128(self) -> u128;
    fn add_u128(&self, num: u128) -> Self;
}


macro_rules! impl_numeric {
    ($type:ty) => {
        impl Numeric for $type {
            const BITS: u32 = <$type>::BITS;
            const MIN: $type = <$type>::MIN;
            const MAX: $type = <$type>::MAX;

            fn new() -> $type {
                0
            }
            fn two() -> $type {
                2
            }
            fn u8_to_numeric(val: u8) -> $type {
                val as $type
            }
            fn pow(self, n: u32) -> Self {
                self.pow(n)
            }
            fn add(self, other: Self) -> Self {
                self + other
            }
            fn mul(self, other: Self) -> Self {
                self * other
            }
            fn is_zero(self) -> bool {
                self == 0
            }
            fn neg(&mut self) {
                *self = self.wrapping_neg();
            }
            fn to_u128(self) -> u128 {
                self as u128
            }
            fn add_u128(&self, num: u128) -> Self {
                self + num as Self
            }
        }
    }
}

impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(i128);
impl_numeric!(u8);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);
impl_numeric!(u128);

macro_rules! impl_from_for_fibin {
    ($type:ty) => {
        impl From<$type> for FiBin {
            fn from(val: $type) -> FiBin {
                let mut fixed_int = FiBin::new();
                if val < 0 { 
                    fixed_int.sign = true;
                }
                let mut num: $type = val.abs_diff(0) as $type;
                while num != 0 {
                    let bit: $type = num % 2;
                    let res = match bit {
                        0 => false,
                        1 => true,
                        _ => panic!("Numbers can only be converted to bool if they are either 0 or 1."),
                    };
                    fixed_int.push(res);
                    num /= 2;
                    if num != 0 {
                    }

                }
                let decimals: FiBin = FiBin{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]};
                fixed_int *= decimals;
                fixed_int.spruce_up()
            }
        }
    };
}

macro_rules! impl_from_for_filong {
    ($type:ty) => {
        impl From<$type> for FiLong {
            fn from(val: $type) -> FiLong {
                let mut fixed_int = FiLong::new();
                if val < 0 { 
                    fixed_int.sign = true;
                }
                let num: u128 = val.abs_diff(0) as u128;
                if num == 0 {
                    return FiLong::new();
                }
                fixed_int.push(low_bits(num) as u64);
                fixed_int.push(high_bits(num) as u64);
                fixed_int *= FiLong::decimals();
                fixed_int.spruce_up()
            }
        }
    };
}

// stole these functions from arithm.rs

#[inline(always)]
fn low_bits(num: u128) -> u128 {
	(num << 64) >> 64
}

#[inline(always)]
fn high_bits(num: u128) -> u128 {
	num >> 64
}

// TODO: polish (might even refactor)
impl FiBin {
    pub fn parse<S: Numeric + std::fmt::Debug>(&self) -> Result<S, FiError>
    where
        S: Numeric
    {
        let divisor: FiBin = FiBin{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]};
        let bits: FiBin = (self.clone() / divisor).spruce_up(); // converts the number to an integer
        if (bits.len() + match_u8(&!<S>::MIN.is_zero()) as usize) > <S>::BITS as usize { // checks if the numbes is larger than the type allows
            return Err(FiError::new(FiErrorKind::NumberTooLarge, "The type you're trying to parse the fixed integer does NOT support numbers this large."));
        }
        let mut res: S = S::new();
        for i in 0..bits.len() { // this loop actually parses the number
            res = res.add(S::two().pow(i as u32).mul(S::u8_to_numeric(match_u8(&bits[i as usize])))); 
        }
        if self.sign {
            if !<S>::MIN.is_zero() { // basically checks if a type is signed
                res.neg();
            } else {
                return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "You can't assign a negative number to an unsigned type. Make sure the fixed interger is greater than zero before parsing into an unsigned number."));
            }
            
        }
        Ok(res)
    }
}

impl FiLong {
    pub fn parse<S: Numeric + std::fmt::Debug>(&self) -> Result<S, FiError>
    where
        S: Numeric
    {
        let bits: FiLong = (self.clone() / FiLong::decimals()).spruce_up(); // converts the number to an integer
        let num: u128 = match bits.len() {
            0 => return Ok(S::new()),
            1 => bits[0] as u128,
            2 => bits[0] as u128 + bits[1] as u128 * 2u128.pow(64),
            _ => return Err(FiError::new(FiErrorKind::NumberTooLarge, "The type you're trying to parse the fixed integer does NOT support numbers this large.")),
        };
        let mut res: S = S::new();
        if num > <S>::MAX.to_u128() {
            return Err(FiError::new(FiErrorKind::NumberTooLarge, "The type you're trying to parse the fixed integer does NOT support numbers this large."));
        } else {
            res = res.add_u128(num);
        }
        
        if self.sign {
            if !<S>::MIN.is_zero() { // basically checks if a type is signed
                res.neg();
            } else {
                return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "You can't assign a negative number to an unsigned type. Make sure the fixed interger is greater than zero before parsing into an unsigned number."));
            }
            
        }
        Ok(res)
    }
}



fn match_u8(bit: &bool) -> u8 {
    match bit {
        true => 1,
        false => 0,
    }
}



impl_from_for_fibin!(i8);
impl_from_for_fibin!(i16);
impl_from_for_fibin!(i32);
impl_from_for_fibin!(i64);
impl_from_for_fibin!(i128);
impl_from_for_fibin!(u8);
impl_from_for_fibin!(u16);
impl_from_for_fibin!(u32);
impl_from_for_fibin!(u64);
impl_from_for_fibin!(u128);

impl_from_for_filong!(i8);
impl_from_for_filong!(i16);
impl_from_for_filong!(i32);
impl_from_for_filong!(i64);
impl_from_for_filong!(i128);
impl_from_for_filong!(u8);
impl_from_for_filong!(u16);
impl_from_for_filong!(u32);
impl_from_for_filong!(u64);
impl_from_for_filong!(u128);
