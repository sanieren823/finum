
use crate::fi::fi;
use crate::fi::bcd;

struct u4([bool; 4]);



// #[inline(always)]
// fn ceil_usize(input: f64) -> usize {
//     input.ceil() as usize
// }

// #[inline(always)]
// fn floor_usize(input: f64) -> usize {
//     input.floor() as usize
// } 
#[inline(always)]
fn to_dec(input: &[bool]) -> usize {
    let mut reversed: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    reversed.reverse();
    match reversed {
        [false, false, false, false] => 0,
        [false, false, false, true]  => 1,
        [false, false, true, false]  => 2,
        [false, false, true, true]   => 3,
        [false, true, false, false]  => 4,
        [false, true, false, true]   => 5,
        [false, true, true, false]   => 6,
        [false, true, true, true]    => 7,
        [true,  false, false, false] => 8,
        [true,  false, false, true]  => 9,
        [true,  false, true, false]  => 10,
        [true,  false, true, true]   => 11,
        [true,  true,  false, false] => 12,
        [true,  true,  false, true]  => 13,
        [true,  true,  true, false]  => 14,
        [true,  true,  true, true]   => 15,
        _ => 0,
    }
}


// should be more efficient for u4
#[inline(always)]
fn plus_three(input: &[bool]) -> [bool; 4] {
    let mut s = [false; 4];
    let mut reversed: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    reversed.reverse();
    match reversed {
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
        _ => s = [false, false, true, true], // TODO: figure out what's missing
    }
    s.reverse();
    s
}

#[inline(always)]
fn minus_three(input: &[bool]) -> [bool; 4] {
    let mut s = [false; 4];
    let mut reversed: [bool; 4] = input.try_into().unwrap_or([false; 4]);
    reversed.reverse();
    match reversed {
        [false, false, false, false] => s = [true, true, false, true], // 0 - 3 = 13 (mod 16)
        [false, false, false, true] => s = [true, true, true, false],  // 1 - 3 = 14 (mod 16)
        [false, false, true, false] => s = [true, true, true, true],   // 2 - 3 = 15 (mod 16)
        [false, false, true, true] => s = [false, false, false, false], // 3 - 3 = 0
        [false, true, false, false] => s = [false, false, false, true], // 4 - 3 = 1
        [false, true, false, true] => s = [false, false, true, false],  // 5 - 3 = 2
        [false, true, true, false] => s = [false, false, true, true],   // 6 - 3 = 3
        [false, true, true, true] => s = [false, true, false, false],   // 7 - 3 = 4
        [true, false, false, false] => s = [false, true, false, true],  // 8 - 3 = 5
        [true, false, false, true] => s = [false, true, true, false],   // 9 - 3 = 6
        [true, false, true, false] => s = [false, true, true, true],    // 10 - 3 = 7
        [true, false, true, true] => s = [true, false, false, false],   // 11 - 3 = 8
        [true, true, false, false] => s = [true, false, false, true],   // 12 - 3 = 9
        [true, true, false, true] => s = [true, false, true, false],    // 13 - 3 = 10
        [true, true, true, false] => s = [true, false, true, true],     // 14 - 3 = 11
        [true, true, true, true] => s = [true, true, false, false],     // 15 - 3 = 12
        _ => s = [true, true, false, true],
    }
    s.reverse();
    println!("{:?}", reversed);
    println!("{:?}", s);
    s
}



impl fi {
    pub fn bin_bcd(&self) -> bcd {
        let length: usize = self.value.len();
        let mut output: Vec<bool> = vec![false; length + (length - 4) / 3 + 1]; // why +1: because index != length
        
        for index in 0..length {
            output[index] = self.value[index];
        }

        for i in 0..=(length - 4) {
            for j in 0..=(i / 3) {
                let current = length - i + j * 4;
                if to_dec(&output[current - 3..=current]) > 4 { // actually 4 going down from current (inverted because that's how slices work in rust)
                    let temp = plus_three(&output[current - 3..=current]);
                    let prev = output[current - 3..=current].to_vec();
                    output[current - 3..=current].copy_from_slice(&temp);
                }
                
            }
        }
        output.reverse();
        let mut vec: Vec<Vec<bool>> = Vec::new();
        let mut arr = vec![false; 4];
        let num_last = output.len() % 4;

        // fill the list so it's easiably matchable
        if num_last > 0 {                                                 
            arr[(4 - num_last)..4].copy_from_slice(&output[0..num_last]);
            vec.push(arr.clone());
        }
        
        for chunk in output[num_last..].chunks(4) {
            vec.push(chunk.to_vec());
        }

        bcd{sign: self.sign, value: vec}
    }
}


impl bcd {
    pub fn bcd_bin(&self) -> fi {
        let mut flat: Vec<bool> = Vec::new();
        for i in 0..self.value.len() {
            for j in 0..self.value[i].len() {
                flat.push(self.value[i][j])
            }
        }
        let length: usize = flat.len();
        let mut output: Vec<bool> = vec![false; length + (length - 4) / 3 + 1]; // why +1: because index != length
        flat.reverse();
        for index in 0..length {
            output[index] = flat[index];
        }
        
        for i in 0..=(length - 4) {
            for j in (i / 3)..(length / 4) {
                let current = length - 4 + i - j * 4;
                if to_dec(&output[current..current + 4]) > 7 { // actually 4 going down from current (inverted because that's how slices work in rust)
                    let temp = minus_three(&output[current..current + 4]);  
                    output[current..current + 4].copy_from_slice(&temp);
                    
                }
            }
        }
        

        if !output.last().unwrap() {

            for el in output.clone().iter().rev() {
                if *el {
                    break;
                } else {
                    output.pop();
                }
            }
        }
        

        fi{sign: self.sign, value: output}        
    } 
}
  


