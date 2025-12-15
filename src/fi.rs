use std::fmt;


// sign-magnitude
#[derive(Debug)]
pub struct Fi {
    pub sign: bool, // true: neg; false: pos
    pub value: Vec<bool>, // 0 is the lsb
}

#[derive(Debug)]
pub struct bcd {
    pub sign: bool,
    pub value: Vec<Vec<bool>>
}

pub trait Parsing {
    
    fn parse_fi(&self) -> Fi;

    fn parse_bcd(&self) -> bcd;

    
}
// TODO: implement Display for as a readable chain of bits in BE (signed)



impl fmt::Display for Fi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string: String = String::new();
        for el in self.value.iter() {
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

impl Fi {
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
       Fi{sign: false, value: Vec::new()}
    }
}

impl bcd {
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
        bcd{sign: false, value: Vec::new()}
    }
}

// impl Default for fi {
//     fn default() -> fi {
//         fi{sign: false, value: Vec::new()}
//     }
// }
