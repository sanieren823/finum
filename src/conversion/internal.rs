
// TODO: consider that all input can be non-spruced-up values 



#[inline(always)]
pub fn low_bits(num: u128) -> u128 {
	num % 64
}

#[inline(always)]
pub fn high_bits(num: u128) -> u128 {
	num >> 64
}


pub fn add(self, other: Self) -> Self {
	let mut carry: u128 = 0;
	let biggest = self.abs() >= other.abs();
	let smallest = self.abs() < other.abs(); // TODO: find a better way to define the smaller one
	let result: FiLong = FiLong::new(); // TODO: change
	for i in 0..biggest.len() {
		let res: u128;
		if smallest.len() > i {
			res = biggest[i] as u128 + carry:
		} else {
			res = biggest[i] as u128 + smallest as u128 + carry;
		}
		carry = high_bits(res); // clone
		result.push(low_bits(res) as u64);

	}
	if carry {
		result.push(carry as u64);
	}
	result.sign = biggest.sign;
	result
}

pub fn sub(self, other: Self) -> Self {
	let mut borrow: u128 = 0;
	let biggest = self.abs() >= other.abs();
	let smallest = self < other; // TODO: find a better way to define the smaller one
	let result: FiLong = FiLong::new(); // TODO: change
	for i in 0..biggest.len() {
		if smallest[i] + borrow > biggest[i]) {
			result.push(u64::MAX + 1 - (smallest[i] - biggest[i]) - borrow);
			borrow = 1;
		} else {
			result.push(biggest[i] - smallest[i] - borrow);
			borrow = 0;
		}

	}
	result.sign = biggest.sign;
	result
}

pub fn mul(self, other: Self) -> Self {
	if self == 0 || other == 0 {
		return FiLong::new();
	}
	let mut carry: u128 = 0;
	let len = self.len() + other.len(); // consider spruce_up()
	let result: FiLong = FiLong{sign: self.sign ^ other.sign, value: vec![0; len)]}; 
	for i in 0..self.len() {
		for j in 0..other.len() {
			let res: u128 = self[i] as u128 * other[i] as u128 + carry:
			carry = high_bits(res); // clone
			result[i + j] = low_bits(res) as u64);
		}

	}
	if carry {
		result[len - 1] = carry as u64;
	}
	result.sign = biggest.sign;
	result
}






// implement long division? requires shr + shl implementation 