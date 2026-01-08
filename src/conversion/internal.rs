
use std::process::Output;

// TODO: consider that all input can be non-spruced-up values 
use crate::fi::FiBin;
use crate::fi::FiLong;

impl FiBin {
	pub fn to_long(&self) -> FiLong {
		let mut output = Vec::with_capacity((self.len() + 63) / 64); // faster than Vec::new()

		let mut qword: u64 = 0;
		let mut byte: u8 = 0;
		let mut cur_bit = 0; 
		let mut cur_byte = 0; 

		for &b in self.iter() {
			byte |= (b as u8) << cur_bit;
			cur_bit += 1;

			if cur_bit == 8 { // case: i % 8 == 0
				qword |= (byte as u64) << (cur_byte * 8);
				byte = 0;
				cur_bit = 0;
				cur_byte += 1;

				if cur_byte == 8 { // case: i % 64 == 0
					output.push(qword);
					qword = 0;
					cur_byte = 0;
				}
			}
		}

		
		if cur_bit != 0 || cur_byte != 0 {
			if cur_bit != 0 {
				qword |= (byte as u64) << (cur_byte * 8);
			}
			output.push(qword);
		}
		FiLong{sign: self.sign, value: output}
	}
} 


impl FiLong {
	pub fn to_bin(&self) -> FiBin {
		let mut output = Vec::with_capacity(self.len() * 64);
		for qword in self.iter() {
			let bytes: [u8; 8] = u64::to_le_bytes(*qword);
			for byte in bytes.iter() {
				for i in 0..8 {
					output.push((byte >> i & 1) != 0);
				}
			}
		}
		FiBin{sign: self.sign, value: output}.spruce_up()
	}
}
