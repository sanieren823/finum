use crate::fi::Fi;
use crate::fi::bcd;
use crate::conversion;
use crate::fi::Parsing;
use crate::functions;


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

macro_rules! impl_from_for_int {
    ($type:ty) => {
        impl From<Fi> for $type {
            fn from(val: Fi) -> $type {
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