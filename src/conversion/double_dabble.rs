use crate::fi::fi;
use crate::fi::bcd;


// matches a 4-bit in LE integer to its approriate value
#[inline(always)]
fn to_dec_le(input: &[bool]) -> usize {
    let bits: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    match bits {
        [false, false, false, false] => 0,
        [true, false, false, false] => 1,
        [false, true, false, false] => 2,
        [true, true, false, false] => 3,
        [false, false, true, false] => 4,
        [true, false, true, false] => 5,
        [false, true, true, false] => 6,
        [true, true, true, false] => 7,
        [false, false, false, true]  => 8,
        [true, false, false, true]  => 9,
        [false, true, false, true]  => 10,
        [true, true, false, true]  => 11,
        [false, false, true, true]  => 12,
        [true, false, true, true]  => 13,
        [false, true, true, true]  => 14,
        [true, true, true, true]  => 15,
    }
}

// matches a 4-bit in BE integer to its approriate value
#[inline(always)]
fn to_dec_be(input: &[bool]) -> usize {
    let mut bits: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    match bits {
        [false, false, false, false] => 0,
        [false, false, false, true] => 1,
        [false, false, true, false] => 2,
        [false, false, true, true] => 3,
        [false, true, false, false] => 4,
        [false, true, false, true] => 5,
        [false, true, true, false] => 6,
        [false, true, true, true] => 7,
        [true, false, false, false] => 8,
        [true, false, false, true] => 9,
        [true, false, true, false] => 10,
        [true, false, true, true] => 11,
        [true, true, false, false] => 12,
        [true, true, false, true] => 13,
        [true, true, true, false] => 14,
        [true, true, true, true] => 15,
    }
}


// matches are 4-bit in LE to the value if 3 was added
#[inline(always)]
fn plus_three(input: &[bool]) -> [bool; 4] {
    let bits: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    match bits {
        [false, false, false, false] => [true, true, false, false],
        [true, false, false, false] => [false, false, true, false],
        [false, true, false, false] => [true, false, true, false], 
        [true, true, false, false] => [false, true, true, false], 
        [false, false, true, false] => [true, true, true, false], 
        [true, false, true, false] => [false, false, false, true], 
        [false, true, true, false] => [true, false, false, true], 
        [true, true, true, false] => [false, true, false, true], 
        [false, false, false, true]  => [true, true, false, true],
        [true, false, false, true] => [false, false, true, true],
        [false, true, false, true] => [true, false, true, true], 
        [true, true, false, true] => [false, true, true, true],
        [false, false, true, true] => [true, true, true, true], 
        [true, false, true, true] => [false, false, false, false], 
        [false, true, true, true] => [true, false, false, false], 
        [true, true, true, true] => [false, true,  false, false],
    }
}

// matches are 4-bit in BE to the value if 3 was subtracted
#[inline(always)]
fn minus_three(input: &[bool]) -> [bool; 4] {
    let mut s = [false; 4];
    let mut bits: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    match bits {
        [false, false, false, false] => s = [true, true, false, true],
        [false, false, false, true] => s = [true, true, true, false], 
        [false, false, true, false] => s = [true, true, true, true],
        [false, false, true, true] => s = [false, false, false, false], 
        [false, true, false, false] => s = [false, false, false, true], 
        [false, true, false, true] => s = [false, false, true, false], 
        [false, true, true, false] => s = [false, false, true, true], 
        [false, true, true, true] => s = [false, true, false, false],
        [true, false, false, false] => s = [false, true, false, true],
        [true, false, false, true] => s = [false, true, true, false], 
        [true, false, true, false] => s = [false, true, true, true], 
        [true, false, true, true] => s = [true, false, false, false], 
        [true, true, false, false] => s = [true, false, false, true], 
        [true, true, false, true] => s = [true, false, true, false], 
        [true, true, true, false] => s = [true, false, true, true], 
        [true, true, true, true] => s = [true, true, false, false],
        
    }
    s
}



impl fi {
    // double-dabble is implemented for LE (as thefi is stored in LE)
    pub fn bin_bcd(&self) -> bcd {
        let length: usize = self.value.len();
        // defines output larger as the bcd encoding takes up more bits than a common binary encoding
        let mut output: Vec<bool> = vec![false; length + (length - 4) / 3 + 1]; // why +1: because index != length
        for index in 0..length {
            output[index] = self.value[index];
        }

        // double-dabble
        for i in 0..=(length - 4) {
            for j in 0..=(i / 3) {
                let current = length - i + j * 4;
                if to_dec_le(&output[current - 3..=current]) > 4 { // actually 4 going down from current (inverted because that's how slices work in rust)
                    let temp = plus_three(&output[current - 3..=current]);
                    let prev = output[current - 3..=current].to_vec();
                    output[current - 3..=current].copy_from_slice(&temp);
                }
                
            }
        }
        // bcd: works in BE; fi: works in LE; --> reverse
        output.reverse();
        let mut vec: Vec<Vec<bool>> = Vec::new();
        let mut arr = vec![false; 4];
        let num_last = output.len() % 4;

        // adds "0"/"false" bits so that bcd handing is easier
        if num_last > 0 {                                                 
            arr[(4 - num_last)..4].copy_from_slice(&output[0..num_last]);
            vec.push(arr.clone());
        }
        
        // split the vector after every 4 element for the bcd encoding
        for chunk in output[num_last..].chunks(4) {
            vec.push(chunk.to_vec());
        }
        // remove any unnecassary zeros to minimize future compute sizes
        while vec[0] == [false, false, false, false] {
            vec.remove(0);
        }
        
        bcd{sign: self.sign, value: vec}
    }
}


impl bcd {
    // reverse double-dabble is implemented for BE (as the bcd is stored in BE)
    pub fn bcd_bin(&self) -> fi {
        // flatten the two-dimensional vector
        let mut flat: Vec<bool> = Vec::new();
        for i in 0..self.value.len() {
            for j in 0..self.value[i].len() {
                flat.push(self.value[i][j])
            }
        }
        let mut output: Vec<bool> = vec![false; flat.len()]; // why +1: because index != length
        // flat.reverse();
        let mut res: Vec<bool> = Vec::new();
        // map the bits of the bcd encoding to a vector
        for index in 0..flat.len() {
            output[index] = flat[index];
        }

        // removes all unnecesasry "false" (mainly a problem for low decimals that are fed into the function directly from a string)
        while output[0] == false {
            output.remove(0);
        }
        // readd those that are neccesary for the algorithm
        for _ in 0..(4 - output.len() % 4) {
            output.insert(0, false);
        }


        // reverse double-dabble
        let mut length: usize = output.len();
        for i in 0..=(length - 4) {
            for j in 1..=(length / 4) {
                let current = length - j * 4;

                if current + 4 <= output.len() {
                    if to_dec_be(&output[current..current + 4]) > 7 {
                        let temp = minus_three(&output[current..current + 4]);
                        output[current..current + 4].copy_from_slice(&temp);
                    }
                    
                }
            }
            let last = output.pop();
            res.push(last.unwrap());
            length = output.len();
        }
        
        // removes any unnecassary "0"/"false" bits at the end of the number
        if !res.last().unwrap() {

            for el in res.clone().iter().rev() {
                if *el {
                    break;
                } else {
                    res.pop();
                }
            }
        }
        fi{sign: self.sign, value: res}        
    } 
}
  


