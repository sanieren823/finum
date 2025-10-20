#[derive(Debug)]
pub struct fi {
    pub sign: bool,
    pub value: Vec<bool>, // 0 is the lsb
}

pub struct bcd {
    pub sign: bool,
    pub value: Vec<Vec<bool>>
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
}
