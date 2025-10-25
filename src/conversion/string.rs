use crate::fi::fi;
use crate::fi::bcd;
use crate::conversion;
use crate::fi::Parsing;


impl fi {
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        if self.sign {
            string.push('-');
        }
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
                _ => panic!("Wrong Input. Function input differs from the expected input. Expected Input type: vec (len = 4)"), // TODO: raise error
            };
            string.push(el_char);
            
        }
        let len = string.chars().count();
        if len > 21 { // check if constant 20 is correct
            string.insert(len - 20, '.') // TODO: figure out what constant is needed
        } else {
            for num in 0..=20-len {
                string.insert(0, '0')  ;
            }
            string.insert(1, '.');
        }
        string
        
    }

}

impl Parsing for String {
    fn parse_fi(&self) -> fi {
        let mut bcd = bcd::new();
        let mut count: usize = 0;
        let mut first: bool = false;
        let len = self.chars().count();
        if self.chars().nth(0) == Some('-') {
            bcd.sign = true;
        } else {
            bcd.sign = false;
        }
        for el in self.chars() {
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
                _ => panic!("Invalid String! Make sure your string only contains numerical characters (0 - 9; '-'; '.')"),

            };
            if next_bcd == [true, true, true, true] {
                if first {
                    panic!("Make sure your string only contains ONE decimal point.")
                }
                first = true; 
                count = len;
                
            } else {
                bcd.value.push(next_bcd.to_vec());
                count += 1;
            }
        }
        let decimals = count - len;  
        // println!("{}", decimals);
        if decimals > 20 {
            for i in 0..decimals - 21 { // plus one because the last one will get special handeling
                bcd.value.pop();
            }
            // TODO: improve the algorithm (or fix it)
            let last = bcd.value.last().unwrap();
            let mut carry: bool = check(last);
            bcd.value.pop();
            let mut index = 0;
            let len = bcd.value.len(); // changed to actual length
            while carry {
                index += 1;
                let last_of_slice = bcd.value[..len - index].last().unwrap();
                carry = check(last_of_slice);
                let temp = plus_one(bcd.value[len - index - 1].clone());
                bcd.value[len - index - 1] = temp;
            }
        } else {
            for i in 0..20-decimals {
                bcd.value.push([false, false, false, false].to_vec())
            }
        }
        println!("{:?}", bcd.value);
        bcd.bcd_bin()
    }

    fn parse_bcd(&self) -> bcd {
        bcd::new()
    }
}

#[inline(always)]
fn check(bits: &Vec<bool>) -> bool {
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

#[inline(always)]
fn plus_one(input: Vec<bool>) -> Vec<bool> {
    let s: [bool; 4];
    match input[..] {
        [false, false, false, false] => s = [false, false, true, true],
        [false, false, false, true] => s = [false, true, false, false],
        [false, false, true, false] => s = [false, true, false, true],
        [false, false, true, true] => s = [false, true, true, false],
        [false, true, false, false] => s = [false, true, true, true],
        [false, true, false, true] => s = [true, false, false, false],
        [false, true, true, false] => s = [true, false, false, true],
        [false, true, true, true] => s = [true, false, true, false],
        [true, false, false, false] => s = [true, false, true, true],
        [true, false, false, true] => s = [true, true, false, false],
        [true, false, true, false] => s = [true, true, false, true],
        [true, false, true, true] => s = [true, true, true, false],
        [true, true, false, false] => s = [true, true, true, true],
        [true, true, false, true] => s = [false, false, false, false],
        [true, true, true, false] => s = [false, false, false, true],
        [true, true, true, true] => s = [false, false, true, false],
        _ => panic!("Value error. The vector length should be exactly 4."), // TODO: figure out what's missing
    }
    s.to_vec()
}
