use crate::fi::fi;
use crate::fi::bcd;
use crate::conversion;
use crate::fi::Parsing;
use crate::functions;

// TODO: finish but after adding the basic arithmetic functions

impl Into<fi> for i8 {
    fn into(self) -> fi {
        let fixed_int = fi::new();
        if self < 0 {
            fixed_int.sign = true;
        }
        let mut num = self.abs();
        while num != 1 || num != 2 {
            fixed_int.value.push(num % 2);
            num /= 2;
        }
        const decimals = fi{sign: false, value: vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false, true, true, false, true, false, false, false, true, true, true, true, false, true, false, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, true, false, true]}
        fixed_int.value *= decimals;
        fixed_int
    }
    
}
impl Into<fi> for i16 {
    
}
impl Into<fi> for i32 {

}
impl Into<fi> for i64 {
    
}
impl Into<fi> for i128 {
    
}
impl Into<fi> for u8 {
    
}
impl Into<fi> for u16 {
    
}
impl Into<fi> for u32 {

}
impl Into<fi> for u64 {
    
}
impl Into<fi> for u128 {
    
}


impl From<i8> for fi {

}
impl From<i16> for fi {
    
}
impl From<i32> for fi {
    
}
impl From<i64> for fi {
    
}
impl From<i128> for fi {
    
}
impl From<u8> for fi {
    
}
impl From<u16> for fi {
    
}
impl From<u32> for fi {
    
}
impl From<u64> for fi {
    
}
impl From<u128> for fi {
    
}

