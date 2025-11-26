
// sign-magnitude
#[derive(Debug)]
pub struct fi {
    pub sign: bool, // true: neg; false: pos
    pub value: Vec<bool>, // 0 is the lsb
}

#[derive(Debug)]
pub struct bcd {
    pub sign: bool,
    pub value: Vec<Vec<bool>>
}

pub trait Parsing {
    fn parse_fi(&self) -> fi;

    fn parse_bcd(&self) -> bcd;
}


impl fi {
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
        fi{sign: false, value: Vec::new()}
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
