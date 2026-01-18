use std::fmt;



// sign-magnitude
#[derive(Debug)]
pub struct FiBin {
    pub sign: bool, // true: neg; false: pos
    pub value: Vec<bool>, // 0 is the lsb
}

#[derive(Debug)]
pub struct FiLong {
    pub sign: bool,
    pub value: Vec<u64>, // research on u128
}

#[derive(Debug)]
pub struct FiBcd { // should it really be public?
    pub sign: bool,
    pub value: Vec<Vec<bool>>,
}

#[derive(Debug)]
pub struct FiBytes {
    pub sign: bool,
    pub value: Vec<u8>,
}



pub trait Parsing {
    
    fn parse_fi(&self) -> FiBin;

    fn parse_bcd(&self) -> FiBcd;

    
}

// Implemented in LE as you would read it 
impl fmt::Display for FiBin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = String::new();
        for el in self.value.iter().rev() {
            string += &match_u8(el).to_string();
        }
        let sign: String;
        if self.sign {
            sign = String::from("-");
        } else {
            sign = String::new();
        }
        write!(f, "{}{}", sign, string)
    }
}


fn match_u8(bit: &bool) -> u8 {
    match bit {
        true => 1,
        false => 0,
    }
}

impl FiBin {
    // #[inline(always)]

    // #[inline(always)]
    // pub fn leading(&self) -> u128 {
    //     // TODO
    // }

    // #[inline(always)]
    // pub fn trailing(&self) -> u128 {
    //     // TODO
    // }
    pub const fn new() -> Self {
       FiBin{sign: false, value: Vec::new()}
    }
}

impl FiBcd {
    // #[inline(always)]

    // #[inline(always)]
    // pub fn leading(&self) -> u128 {
    //     // TODO
    // }

    // #[inline(always)]
    // pub fn trailing(&self) -> u128 {
    //     // TODO
    // }
    pub const fn new() -> Self {
        FiBcd{sign: false, value: Vec::new()}
    }
}

// impl Default for fi {
//     fn default() -> fi {
//         fi{sign: false, value: Vec::new()}
//     }
// }
