
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
