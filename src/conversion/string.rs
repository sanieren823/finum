use crate::fi::{FiBin, FiBcd, FiLong};

impl FiBin {
    pub fn to_string(&self) -> String {
        // adds the "-" sing if the number is negative
        let mut string = String::new();
        let base: usize;
        if self.sign {
            string.push('-');
            base = 1;
        } else {
            base = 0;
        }
        // matches the 4-bit upon converting to the string digits
        for el in self.bin_bcd().value.iter() {
            let el_char = match el[..] {
                [false, false, false, false] => '0',
                [false, false, false, true]  => '1',
                [false, false, true, false]  => '2',
                [false, false, true, true]   => '3',
                [false, true, false, false]  => '4',
                [false, true, false, true]   => '5',
                [false, true, true, false]   => '6',
                [false, true, true, true]    => '7',
                [true,  false, false, false] => '8',
                [true,  false, false, true]  => '9',
                [true,  false, true, false]  => panic!("Algorithm error. Expected result: 10"), // raise error if large than 10?
                [true,  false, true, true]   => panic!("Algorithm error. Expected result: 11"),
                [true,  true,  false, false] => panic!("Algorithm error. Expected result: 12"),
                [true,  true,  false, true]  => panic!("Algorithm error. Expected result: 13"),
                [true,  true,  true, false]  => panic!("Algorithm error. Expected result: 14"),
                [true,  true,  true, true]   => panic!("Algorithm error. Expected result: 15"),
                _ => panic!("Wrong Input. Function input differs from the expected input. Expected Input type: vec (len = 4)"),
            };
            string.push(el_char);
            
        }
        // inserts the decimal point (and adds zeros if required)
        let len = string.chars().count();
        if len > 20 {
            string.insert(len - 20, '.')
        } else {
            for _num in 0..=20-len {
                string.insert(0 + base, '0')  ;
            }
            string.insert(1 + base, '.');
        }
        string
        
    }
}

impl FiLong {
    pub fn to_string(&self) -> String {
        self.to_bin().to_string()
    }
}
// impl FiBcd {
//     pub fn to_string(&self) -> String {
//         // adds the "-" sing if the number is negative
//         let mut string = String::new();
//         if self.sign {
//             string.push('-');
//         }
//         // matches the 4-bit (BE) to string digits 
//         for el in self.value.iter() {
//             let el_char = match el[..] {
//                 [false, false, false, false] => '0',
//                 [false, false, false, true]  => '1',
//                 [false, false, true, false]  => '2',
//                 [false, false, true, true]   => '3',
//                 [false, true, false, false]  => '4',
//                 [false, true, false, true]   => '5',
//                 [false, true, true, false]   => '6',
//                 [false, true, true, true]    => '7',
//                 [true,  false, false, false] => '8',
//                 [true,  false, false, true]  => '9',
//                 [true,  false, true, false]  => panic!("Algorithm error. Expected result: 10"), // raise error if larger than 10?
//                 [true,  false, true, true]   => panic!("Algorithm error. Expected result: 11"),
//                 [true,  true,  false, false] => panic!("Algorithm error. Expected result: 12"),
//                 [true,  true,  false, true]  => panic!("Algorithm error. Expected result: 13"),
//                 [true,  true,  true, false]  => panic!("Algorithm error. Expected result: 14"),
//                 [true,  true,  true, true]   => panic!("Algorithm error. Expected result: 15"),
//                 _ => panic!("Wrong Input. Function input differs from the expected input. Expected Input type: vec (len = 4)"), 
//             };
//             string.push(el_char);
            
//         }
//         // inserts the decimal point (and adds zeros if required)
//         let len = string.chars().count();
//         if len > 20 {
//             string.insert(len - 20, '.')
//         } else {
//             for _num in 0..=20-len {
//                 string.insert(0, '0')  ;
//             }
//             string.insert(1, '.');
//         }
//         string
        
//     }

// }

// impl Parsing for String {
//     // given that the parse_bcd function is inline there should not be any difference in runtime compared to new function (99% of the lines would be identical)
//     #[inline(always)]
//     fn parse_fi(&self) -> FiBin { 
//         self.parse_bcd().bcd_bin()
//     }

//     #[inline(always)]
//     fn parse_bcd(&self) -> FiBcd {
//         let mut bcd = FiBcd::new();
//         let mut count: usize = 0;
//         let mut first: bool = false;
//         let len = self.chars().count();
//         // checks if the string represents a negative nubmer
//         if self.chars().nth(0) == Some('-') {
//             bcd.sign = true;
//         } else {
//             bcd.sign = false;
//         }
//         // matches the digits to their 4-bit encoding (BE)
//         for el in self.chars() {
//             let next_bcd = match el {
//                 '0' => [false, false, false, false],
//                 '1' => [false, false, false, true],
//                 '2' => [false, false, true, false],
//                 '3' => [false, false, true, true],
//                 '4' => [false, true, false, false],
//                 '5' => [false, true, false, true],
//                 '6' => [false, true, true, false],
//                 '7' => [false, true, true, true],
//                 '8' => [true,  false, false, false],
//                 '9' => [true,  false, false, true],
//                 '.' => [true, true, true, true], 
//                 '-' => [true, true, true, false],
//                 _ => panic!("Invalid String! Make sure your string only contains numerical characters (0 - 9; '-'; '.')"),

//             };
//             // handles decimal points
//             if next_bcd == [true, true, true, true] {
//                 // panics if there are more than one decimal point
//                 if first {
//                     panic!("Make sure your string only contains ONE decimal point.")
//                 }
//                 first = true; 
//                 count = len;
                
//             } else if next_bcd == [true, true, true, false] {
//                 count += 1;
//             } else {
//                 bcd.value.push(next_bcd.to_vec());
//                 count += 1;
//             }
//         }
//         let decimals: usize = count - len;  
//         // rounds if there are more than 20 decimals
//         if decimals > 20 {
//             for _i in 0..decimals - 21 { // plus one since the last 4-bit will get special handeling
//                 bcd.value.pop();
//             }
//             // checks if the 4-bit at index 20 after the decimal point will need a carry
//             let last = bcd.value.last().unwrap();
//             let mut carry: bool = check_round(last);
//             bcd.value.pop();
//             let mut index = 0;
//             let len = bcd.value.len();
//             // as long as there's a carry (the last 4-bit is "9") the vector is updated
//             while carry {
//                 let last_of_slice = bcd.value[..len - index].last().unwrap();
//                 carry = check_carry(last_of_slice);
//                 let temp = plus_one(bcd.value[len - index - 1].clone());
//                 bcd.value[len - index - 1] = temp;
//                 index += 1;
//             }
//         } else {
//             // adds missing decimals 
//             for _i in 0..20-decimals {
//                 bcd.value.push([false, false, false, false].to_vec())
//             }
//         }
//         bcd
//     }
// }

// checks whether the 4-bit would get rounded up/down (BE)
#[inline(always)]
fn check_round(bits: &Vec<bool>) -> bool {
    match bits[..] {
        [false, false, false, false] => false,
        [false, false, false, true] => false,
        [false, false, true, false] => false,
        [false, false, true, true] => false,
        [false, true, false, false] => false,
        [false, true, false, true] => true,
        [false, true, true, false] => true,
        [false, true, true, true] => true,
        [true, false, false, false] => true,
        [true, false, false, true] => true,
        _ => panic!("Value error. The number has to be between 0 - 9"),
    }
}

// checks whether we need a carry bit (if the last 4-bit is "9") (BE/LE)
#[inline(always)]
fn check_carry(bits: &Vec<bool>) -> bool {
    match bits[..] {
        [true, false, false, true] => true,
        _ => false,
    }
}


// adds 1 to the 4-bit (BE)
#[inline(always)]
fn plus_one(input: Vec<bool>) -> Vec<bool> {
    let s: [bool; 4];
    match input[..] {
        [false, false, false, false] => s = [false, false, false, true],
        [false, false, false, true]  => s = [false, false, true, false],
        [false, false, true, false]  => s = [false, false, true, true],
        [false, false, true, true]   => s = [false, true, false, false],
        [false, true, false, false]  => s = [false, true, false, true],
        [false, true, false, true]   => s = [false, true, true, false],
        [false, true, true, false]   => s = [false, true, true, true],
        [false, true, true, true]    => s = [true, false, false, false],
        [true, false, false, false]  => s = [true, false, false, true],
        [true, false, false, true]   => s = [false, false, false, false],

        _ => panic!("Value error. The vector should be a 4-bit integer between 0 - 9."), // TODO: figure out what's missing
    }
    s.to_vec()
}


impl From<String> for FiBin {
    fn from(val: String) -> FiBin {
        let mut bcd = FiBcd::new();
        let mut count: usize = 0;
        let mut first: bool = false;
        let len = val.chars().count();
        // checks if the string represents a negative nubmer
        if val.chars().nth(0) == Some('-') {
            bcd.sign = true;
        } else {
            bcd.sign = false;
        }
        // matches the digits to their 4-bit encoding (BE)
        for el in val.chars() {
            let next_bcd = match el {
                '0' => [false, false, false, false],
                '1' => [false, false, false, true],
                '2' => [false, false, true, false],
                '3' => [false, false, true, true],
                '4' => [false, true, false, false],
                '5' => [false, true, false, true],
                '6' => [false, true, true, false],
                '7' => [false, true, true, true],
                '8' => [true,  false, false, false],
                '9' => [true,  false, false, true],
                '.' => [true, true, true, true], 
                '-' => [true, true, true, false],
                _ => panic!("Invalid String! Make sure your string only contains numerical characters (0 - 9; '-'; '.')"),

            };
            // handles decimal points
            if next_bcd == [true, true, true, true] {
                // panics if there are more than one decimal point
                if first {
                    panic!("Make sure your string only contains ONE decimal point.")
                }
                first = true; 
                count = len;
                
            } else if next_bcd == [true, true, true, false] {
                count += 1;
            } else {
                bcd.value.push(next_bcd.to_vec());
                count += 1;
            }
        }
        let decimals: usize = count - len;  
        // rounds if there are more than 20 decimals
        if decimals > 20 {
            for _i in 0..decimals - 21 { // plus one since the last 4-bit will get special handeling
                bcd.value.pop();
            }
            // checks if the 4-bit at index 20 after the decimal point will need a carry
            let last = bcd.value.last().unwrap();
            let mut carry: bool = check_round(last);
            bcd.value.pop();
            let mut index = 0;
            let len = bcd.value.len();
            // as long as there's a carry (the last 4-bit is "9") the vector is updated
            while carry {
                let last_of_slice = bcd.value[..len - index].last().unwrap();
                carry = check_carry(last_of_slice);
                let temp = plus_one(bcd.value[len - index - 1].clone());
                bcd.value[len - index - 1] = temp;
                index += 1;
            }
        } else {
            // adds missing decimals 
            for _i in 0..20-decimals {
               bcd.value.push([false, false, false, false].to_vec())
            }
        }
        bcd.bcd_bin()
    }
}

impl From<&String> for FiBin {
    fn from(val: &String) -> FiBin {
        let mut bcd = FiBcd::new();
        let mut count: usize = 0;
        let mut first: bool = false;
        let len = val.chars().count();
        // checks if the string represents a negative nubmer
        if val.chars().nth(0) == Some('-') {
            bcd.sign = true;
        } else {
            bcd.sign = false;
        }
        // matches the digits to their 4-bit encoding (BE)
        for el in val.chars() {
            let next_bcd = match el {
                '0' => [false, false, false, false],
                '1' => [false, false, false, true],
                '2' => [false, false, true, false],
                '3' => [false, false, true, true],
                '4' => [false, true, false, false],
                '5' => [false, true, false, true],
                '6' => [false, true, true, false],
                '7' => [false, true, true, true],
                '8' => [true,  false, false, false],
                '9' => [true,  false, false, true],
                '.' => [true, true, true, true], 
                '-' => [true, true, true, false],
                _ => panic!("Invalid String! Make sure your string only contains numerical characters (0 - 9; '-'; '.')"),

            };
            // handles decimal points
            if next_bcd == [true, true, true, true] {
                // panics if there are more than one decimal point
                if first {
                    panic!("Make sure your string only contains ONE decimal point.")
                }
                first = true; 
                count = len;
                
            } else if next_bcd == [true, true, true, false] {
                count += 1;
            } else {
                bcd.value.push(next_bcd.to_vec());
                count += 1;
            }
        }
        let decimals: usize = count - len;  
        // rounds if there are more than 20 decimals
        if decimals > 20 {
            for _i in 0..decimals - 21 { // plus one since the last 4-bit will get special handeling
                bcd.value.pop();
            }
            // checks if the 4-bit at index 20 after the decimal point will need a carry
            let last = bcd.value.last().unwrap();
            let mut carry: bool = check_round(last);
            bcd.value.pop();
            let mut index = 0;
            let len = bcd.value.len();
            // as long as there's a carry (the last 4-bit is "9") the vector is updated
            while carry {
                let last_of_slice = bcd.value[..len - index].last().unwrap();
                carry = check_carry(last_of_slice);
                let temp = plus_one(bcd.value[len - index - 1].clone());
                bcd.value[len - index - 1] = temp;
                index += 1;
            }
        } else {
            // adds missing decimals 
            for _i in 0..20-decimals {
               bcd.value.push([false, false, false, false].to_vec())
            }
        }
        bcd.bcd_bin()
    }
}

impl From<&str> for FiBin {
    fn from(val: &str) -> FiBin {
        let mut bcd = FiBcd::new();
        let mut count: usize = 0;
        let mut first: bool = false;
        let len = val.chars().count();
        // checks if the string represents a negative nubmer
        if val.chars().nth(0) == Some('-') {
            bcd.sign = true;
        } else {
            bcd.sign = false;
        }
        // matches the digits to their 4-bit encoding (BE)
        for el in val.chars() {
            let next_bcd = match el {
                '0' => [false, false, false, false],
                '1' => [false, false, false, true],
                '2' => [false, false, true, false],
                '3' => [false, false, true, true],
                '4' => [false, true, false, false],
                '5' => [false, true, false, true],
                '6' => [false, true, true, false],
                '7' => [false, true, true, true],
                '8' => [true,  false, false, false],
                '9' => [true,  false, false, true],
                '.' => [true, true, true, true], 
                '-' => [true, true, true, false],
                _ => panic!("Invalid String! Make sure your string only contains numerical characters (0 - 9; '-'; '.')"),

            };
            // handles decimal points
            if next_bcd == [true, true, true, true] {
                // panics if there are more than one decimal point
                if first {
                    panic!("Make sure your string only contains ONE decimal point.")
                }
                first = true; 
                count = len;
                
            } else if next_bcd == [true, true, true, false] {
                count += 1;
            } else {
                bcd.value.push(next_bcd.to_vec());
                count += 1;
            }
        }
        let decimals: usize = count - len;  
        // rounds if there are more than 20 decimals
        if decimals > 20 {
            for _i in 0..decimals - 21 { // plus one since the last 4-bit will get special handeling
                bcd.value.pop();
            }
            // checks if the 4-bit at index 20 after the decimal point will need a carry
            let last = bcd.value.last().unwrap();
            let mut carry: bool = check_round(last);
            bcd.value.pop();
            let mut index = 0;
            let len = bcd.value.len();
            // as long as there's a carry (the last 4-bit is "9") the vector is updated
            while carry {
                let last_of_slice = bcd.value[..len - index].last().unwrap();
                carry = check_carry(last_of_slice);
                let temp = plus_one(bcd.value[len - index - 1].clone());
                bcd.value[len - index - 1] = temp;
                index += 1;
            }
        } else {
            // adds missing decimals 
            for _i in 0..20-decimals {
               bcd.value.push([false, false, false, false].to_vec())
            }
        }
        bcd.bcd_bin()
    }
}


impl From<String> for FiLong {
    fn from(val: String) -> FiLong {
        FiBin::from(val).to_long()
    }
}

impl From<&String> for FiLong {
    fn from(val: &String) -> FiLong {
        FiBin::from(val).to_long()
    }
}

impl From<&str> for FiLong {
    fn from(val: &str) -> FiLong {
        FiBin::from(val).to_long()
    }
}
