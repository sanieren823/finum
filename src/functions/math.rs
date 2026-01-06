use crate::fi::{FiBin, FiLong};
use std::time::Instant;
use crate::errors::FiError;
use crate::errors::FiErrorKind;


trait PowInteger<Rhs = Self> {
    type Output;

    fn pow_int(self, rhs: Rhs) -> Self::Output;
}














impl FiBin {
    pub fn pow(self, n: Self) -> Self {
        if n.is_integer() {
            gen_int_pow(self, n.clone() / FiBin::decimals())
        } else {
            binominal_series(self - 1.into(), n)
        }
        
    }

    pub fn sqrt(self) -> Self {
        FiBin::new()
    }
    pub fn nth_root(self, n: Self) -> Self {
        let mut prev: FiBin = FiBin::new();
        let mut guess: FiBin = self.clone() / n.clone();
        let cur = Instant::now();
        while guess != prev {
            let single = Instant::now();
            prev = guess.clone();
            guess = newtons_method(prev.clone(), self.clone(), n.clone());
            println!("{:?}", single.elapsed());
        }
        println!("{:?}", cur.elapsed());
        guess
    }
    pub fn factorial(self) -> Self {
        FiBin::new()
    }
    pub fn termial(self) -> Self {
        let mut num = self.clone() % 1.into();
        let mut counter = self.clone() / FiBin::decimals();
        while !counter.is_zero() {
            num += counter.clone() * FiBin::decimals();
            if self.sign {
                counter += FiBin{sign: false, value: vec![true]};
            } else {
                counter -= FiBin{sign: false, value: vec![true]};
            }
        }
        num
    }
    pub fn ln(self) -> Self {
        FiBin::new()
    }

    pub fn log(self, base: Self) -> Self {
        FiBin::new()
    }

    pub fn log_10(self) -> FiBin {
        self.ln() / FiBin::from(10)
    }

    pub fn log_2(self) -> FiBin {
        self.ln() / FiBin::from(2)
    }
}


fn gen_int_pow(base: FiBin, exponent: FiBin) -> FiBin {
    let mut res: FiBin = 1.into();
    let mut counter = exponent.clone(); 
    let time = Instant::now();
    while !counter.is_zero() {
        let single = Instant::now();
        res *= base.clone();
        println!("{:?}", res.to_string());
        println!("{:?}", single.elapsed());
        if counter.sign {
            counter += FiBin{sign: false, value: vec![true]};
        } else {
            counter -= FiBin{sign: false, value: vec![true]};
        }
    }
    if exponent.sign {
        res = FiBin::from(1) / res;
    }
    println!("{:?}", time.elapsed());
    res
}

fn newtons_method(guess: FiBin, x: FiBin, n: FiBin) -> FiBin {
    let n_1 = n.clone() - 1.into();
    (guess.clone() * n_1.clone() + x / guess.pow(n_1)) / n
}

fn binominal_series(base: FiBin, exponent: FiBin) -> FiBin {
    let mut sum = FiBin::new();
    let mut temp: FiBin = 1.into();
    let mut k: FiBin = FiBin::new();
    let mut last: FiBin = FiBin::new();
    while temp != sum {
        last = binominal_coefficent(exponent.clone(), k.clone(), last.clone());
        temp = sum.clone();
        sum += base.clone().pow(k.clone()) * last.clone();

        k += 1.into();
        println!("{}", sum.to_string());
    }
    sum
}


fn binominal_coefficent_recursive(alpha: FiBin, k: FiBin) -> FiBin {
    if k.is_zero() {
        1.into()
    } else {
        let prev: FiBin = k.clone() - 1.into();
        binominal_coefficent_recursive(alpha.clone(), prev.clone()) * (alpha - prev) / k
    }
    
}

fn binominal_coefficent(alpha: FiBin, k: FiBin, prev: FiBin) -> FiBin {
    if k.is_zero() {
        1.into()
    } else {
        prev * ((alpha - (k.clone() - 1.into())) / k)
    }
    
}


fn fl_log_2(num: FiBin) -> Result<FiBin, FiError> { 
    let mut shifted;
    let mut res = FiBin::new();
    if num.sign {
        return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "Mind that the logarithmic function doesn't implement negative numbers."));
    } else if num < FiBin::one() {
        shifted = FiBin::one() / num;
        println!("num: {:?}", shifted.to_string());
        res.sign = true;  
    } else {
        shifted = num
    }
    shifted = shifted / FiBin::decimals();
    println!("{:?}", shifted.to_string());
    
    while shifted.len() > 1 {
        shifted = shifted.clone() >> 1;
        if res.sign {
            res -= 1.into();
        } else {
            res += 1.into();
        }
    }
    Ok(res)
}


fn fl_log_2_long(num: FiLong) -> Result<FiLong, FiError> { 
    let mut shifted;
    let mut res: FiLong = FiLong::new();
    let mut sign = false;
    if num.sign {
        return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "Mind that the logarithmic function doesn't implement negative numbers."));
    } else if num < FiLong::one() {
        shifted = FiLong::one() / num;
        println!("num: {:?}", shifted.to_bin().to_string());
        sign = true;
    } else {
        shifted = num
    }
    shifted = shifted / FiLong::decimals();
    println!("{:?}", shifted.to_bin().to_string());
    
    while shifted >= 2.into() {
        shifted >>= 1;
        res += 1;
    }
    res.sign = sign;
    Ok(res)
}

fn decimals_log_2_long(num: FiLong) -> FiLong { // input must be the residue --> between 1 and 2
    let y = num.clone();
    let mut z = num;
    let mut m: usize = 0;
    let mut res = FiLong::new();
    while z < 2.into() {
        z *= &y;
        m -= 1;
    }
    // res = 2^-m + 2-m log2(z/2)
    res = FiLong::two().pow_int(m) + FiLong::two().pow_int(m) * decimals_log_2_long(z / FiLong::two());
    FiLong::new()
}



impl PowInteger<FiLong> for FiLong {
    type Output = FiLong;

    fn pow_int(self, num: FiLong) -> Self::Output {
        let mut res: FiLong = 1.into();
        let mut counter = self.absolute();
        while counter > 1.into() {
            res *= &self;
            counter -= 1;
        }
        if num.sign {
            res = FiLong::from(1) / res;
        }
        res
    }
}

impl PowInteger<&FiLong> for FiLong {
    type Output = FiLong;

    fn pow_int(self, num: &FiLong) -> Self::Output {
        let mut res: FiLong = 1.into();
        let mut counter = self.absolute();
        while counter > 1.into() {
            res *= &self;
            counter -= 1;
        }
        if num.sign {
            res = FiLong::from(1) / res;
        }
        res
    }
}

impl PowInteger<FiLong> for &FiLong {
    type Output = FiLong;

    fn pow_int(self, num: FiLong) -> Self::Output {
        let mut res: FiLong = 1.into();
        let mut counter = self.absolute();
        while counter > 1.into() {
            res *= self;
            counter -= 1;
        }
        if num.sign {
            res = FiLong::from(1) / res;
        }
        res
    }
}

impl PowInteger<&FiLong> for &FiLong {
    type Output = FiLong;

    fn pow_int(self, num: &FiLong) -> Self::Output {
        let mut res: FiLong = 1.into();
        let mut counter = self.absolute();
        while counter > 1.into() {
            res *= self;
            counter -= 1;
        }
        if num.sign {
            res = FiLong::from(1) / res;
        }
        res
    }
}

macro_rules! pow_int_for_int {
    ($type:ty) => {
        impl PowInteger<$type> for FiLong {
            type Output = FiLong;

            fn pow_int(self, num: $type) -> Self::Output {
                let mut res: FiLong = 1.into();
                let exponent = num.abs_diff(0);
                for _ in 0..exponent {
                    res *= &self;
                }
                if num < 0 {
                    res = FiLong::from(1) / res;
                }
                res
            }
        }
        impl PowInteger<&$type> for FiLong {
            type Output = FiLong;

            fn pow_int(self, num: &$type) -> Self::Output {
                let mut res: FiLong = 1.into();
                let exponent = num.abs_diff(0);
                for _ in 0..exponent {
                    res *= &self;
                }
                if *num < 0 {
                    res = FiLong::from(1) / res;
                }
                res
            }
        }
        impl PowInteger<$type> for &FiLong {
            type Output = FiLong;

            fn pow_int(self, num: $type) -> Self::Output {
                let mut res: FiLong = 1.into();
                let exponent = num.abs_diff(0);
                for _ in 0..exponent {
                    res *= self;
                }
                if num < 0 {
                    res = FiLong::from(1) / res;
                }
                res
            }
        }
        impl PowInteger<&$type> for &FiLong {
            type Output = FiLong;

            fn pow_int(self, num: &$type) -> Self::Output {
                let mut res: FiLong = 1.into();
                let exponent = num.abs_diff(0);
                for _ in 0..exponent {
                    res *= self;
                }
                if *num < 0 {
                    res = FiLong::from(1) / res;
                }
                res
            }  
        }
    };
}


pow_int_for_int!(isize);
pow_int_for_int!(i8);
pow_int_for_int!(i16);
pow_int_for_int!(i32);
pow_int_for_int!(i64);
pow_int_for_int!(i128);
pow_int_for_int!(usize);
pow_int_for_int!(u8);
pow_int_for_int!(u16);
pow_int_for_int!(u32);
pow_int_for_int!(u64);
pow_int_for_int!(u128);