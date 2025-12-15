use crate::fi::Fi;
use crate::errors::FiError;
use crate::errors::FiErrorKind;
use crate::fi::bcd;
use crate::conversion;
use crate::fi::Parsing;
use crate::functions;


pub trait ParseInt<T> {
    fn parse(&self) -> Result<T, FiError>;
}

pub trait ParseIntGeneric<T> {
    fn parse<S>(&self) -> Result<S, FiError>;
}


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
    fn neg(self) -> Self;
}
// impl Numeric for i8 {}
// impl Numeric for i16 {}
// impl Numeric for i32 {}
// impl Numeric for i64 {}
// impl Numeric for i128 {}
// impl Numeric for u8 {}
// impl Numeric for u16 {}
// impl Numeric for u32 {}
// impl Numeric for u64 {}
// impl Numeric for u128 {}


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
            fn neg(self) -> Self {
                self.wrapping_neg()
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

macro_rules! impl_from_for_fi {
    ($type:ty) => {
        impl From<$type> for Fi {
            fn from(val: $type) -> Fi {
                let mut fixed_int = Fi::new();
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
                    fixed_int.value.push(res); // implement push
                    num /= 2;
                    if num != 0 {
                    }

                }
                let decimals: Fi = Fi{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]};
                fixed_int *= decimals;
                fixed_int.spruce_up()
            }
        }
    };
}



// TODO
// macro_rules! impl_parse_for_fi {
//     ($type:ty) => {
//         impl ParseInt<$type> for Fi {
//             fn parse(&self) -> Result<$type, FiError> {
//                 let divisor: Fi = Fi{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]};
//                 let bits: Fi = (self.clone() / divisor).spruce_up();
//                 if bits.len() > <$type>::BITS as usize {
//                     return Err(FiError::new(FiErrorKind::NumberTooLarge, "The type you're trying to parse the fixed integer does NOT support numbers this large."));
//                 }
//                 let mut res: $type = 0;
//                 for i in 0..bits.len() {
//                     res += (2 as $type).pow(i as u32) * match_u8(&bits[i as usize]) as $type;
//                 }
//                 if self.sign {
//                     if <$type>::MIN != 0 {
//                         res = res.wrapping_neg();
//                     } else {
//                         return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "You can't assign a negative number to an unsigned type. Make sure the fixed interger is greater than zero before parsing into an unsigned number."));
//                     }
                    
//                 }
//                 Ok(res)
//             }
//         }
//     };
// }

// macro_rules! impl_parse_for_fi_generic {
//     ($type:ty) => {
//         impl ParseIntGeneric<$type> for Fi {
            
//         }
//     };
// }


// TODO: polish (might even refactor)
impl Fi {
    pub fn parse<S: Numeric + std::fmt::Debug>(&self) -> Result<S, FiError>
    where
        S: Numeric
    {
        let divisor: Fi = Fi{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, true, true, false, true, false, true, false, true, true, true, true, true, false, false, true, true, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, false, true, false, false, true, false, false, true, false, true, false, false, true, true, true, false, true, false, true, true, false, false, false, false, true, true, true, false, false, false, true, true, true, true, true, false, false, true, false, true, false, false, true, true, false, false, false, true, true, false, true, false, true, true, true]};
        let bits: Fi = (self.clone() / divisor).spruce_up();
        if (bits.len() + match_u8(&!<S>::MIN.is_zero()) as usize) > <S>::BITS as usize {
            return Err(FiError::new(FiErrorKind::NumberTooLarge, "The type you're trying to parse the fixed integer does NOT support numbers this large."));
        }
        let mut res: S = S::new();
        for i in 0..bits.len() {
            res = res.add(S::two().pow(i as u32).mul(S::u8_to_numeric(match_u8(&bits[i as usize]))));
        }
        if self.sign {
            if !<S>::MIN.is_zero() {
                res = res.neg();
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



impl_from_for_fi!(i8);
impl_from_for_fi!(i16);
impl_from_for_fi!(i32);
impl_from_for_fi!(i64);
impl_from_for_fi!(i128);
impl_from_for_fi!(u8);
impl_from_for_fi!(u16);
impl_from_for_fi!(u32);
impl_from_for_fi!(u64);
impl_from_for_fi!(u128);

// impl_parse_for_fi!(i8);
// impl_parse_for_fi!(i16);
// impl_parse_for_fi!(i32);
// impl_parse_for_fi!(i64);
// impl_parse_for_fi!(i128);
// impl_parse_for_fi!(u8);
// impl_parse_for_fi!(u16);
// impl_parse_for_fi!(u32);
// impl_parse_for_fi!(u64);
// impl_parse_for_fi!(u128);


// impl_parse_for_fi_generic!(i8);
// impl_parse_for_fi_generic!(i16);
// impl_parse_for_fi_generic!(i32);
// impl_parse_for_fi_generic!(i64);
// impl_parse_for_fi_generic!(i128);
// impl_parse_for_fi_generic!(u8);
// impl_parse_for_fi_generic!(u16);
// impl_parse_for_fi_generic!(u32);
// impl_parse_for_fi_generic!(u64);
// impl_parse_for_fi_generic!(u128);