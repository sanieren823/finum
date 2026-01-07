use crate::fi::{FiBin, FiLong};
use std::time::Instant;
use crate::errors::FiError;
use crate::errors::FiErrorKind;


trait PowInteger<Rhs = Self> {
    type Output;

    fn pow_int(self, rhs: Rhs) -> Self::Output;
}

trait Logarithm<Rhs = Self> {
    type Output;

    fn log(self, rhs: Rhs) -> Self::Output;
}

// TODO: change all integer arithmetic to the FiLong counterpart (should be marginally faster)

// PowReal --> x^n = e^(n * ln(x)) --> e^x can be approximated used in taylor series and pre calculated factorials

// traits to implement: Factorial + Termial + PowReal + Exponential + Trigonometry + Sqrt + NthRoot
// macro for pow? or seperate trait?













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

impl FiLong { // helper functions for the log2 calculation
    #[inline(always)]
    fn hundred() -> FiLong {
        FiLong{sign: false, value: vec![1864712049423024128, 542]}
    }
    #[inline(always)]
    fn hundredth() -> FiLong {
        FiLong{sign: false, value: vec![1000000000000000000]}
    }
}

fn fl_log_2_long(num: &FiLong) -> Result<FiLong, FiError> { 
    let mut shifted;
    let mut res: FiLong = FiLong::new();
    let mut sign = false;
    if num.sign {
        return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "Mind that the logarithmic function doesn't implement negative numbers."));
    } else if *num < FiLong::one() {
        shifted = FiLong::one() / num;
        sign = true;
    } else {
        shifted = num.clone();
    }
    while shifted >= FiLong::two() {
        shifted >>= 1;
        res += 1;
    }
    res += decimals_log_2_long(&shifted, &FiLong::hundred()) * FiLong::hundredth(); // this adds precision; from what i measured 100 is usually enough
    res.sign = sign;
    Ok(res)
}

fn decimals_log_2_long(num: &FiLong, factor: &FiLong) -> FiLong { // input must be the residue --> between 1 and 2
    
    if *num == FiLong::one() {
        return FiLong::new();
    }
    let mut z = num.clone();
    let mut m: usize = 0;
    let mut res = FiLong::new();
    while z < FiLong::two() {
        z = &z * &z;
        m += 1;
    }
    let inverse: FiLong = (FiLong::one() >> m) * factor;
    if !inverse.is_zero() {
        res = &inverse + decimals_log_2_long(&(z >> 1), &inverse);
    }
    res
}

impl PowInteger<FiLong> for FiLong {
    type Output = FiLong;

    fn pow_int(self, num: FiLong) -> Self::Output {
        let mut res: FiLong = FiLong::one();
        let mut counter = self.absolute();
        while counter > FiLong::one() {
            res *= &self;
            counter -= 1;
        }
        if num.sign {
            res = 1 / res;
        }
        res
    }
}

impl PowInteger<&FiLong> for FiLong {
    type Output = FiLong;

    fn pow_int(self, num: &FiLong) -> Self::Output {
        let mut res: FiLong = FiLong::one();
        let mut counter = self.absolute();
        while counter > FiLong::one() {
            res *= &self;
            counter -= 1;
        }
        if num.sign {
            res = 1 / res;
        }
        res
    }
}

impl PowInteger<FiLong> for &FiLong {
    type Output = FiLong;

    fn pow_int(self, num: FiLong) -> Self::Output {
        let mut res: FiLong = FiLong::one();
        let mut counter = self.absolute();
        while counter > FiLong::one() {
            res *= self;
            counter -= 1;
        }
        if num.sign {
            res = 1 / res;
        }
        res
    }
}

impl PowInteger<&FiLong> for &FiLong {
    type Output = FiLong;

    fn pow_int(self, num: &FiLong) -> Self::Output {
        let mut res: FiLong = FiLong::one();
        let mut counter = self.absolute();
        while counter > FiLong::one() {
            res *= self;
            counter -= 1;
        }
        if num.sign {
            res = 1 / res;
        }
        res
    }
}

macro_rules! pow_int_for_int {
    ($type:ty) => {
        impl PowInteger<$type> for FiLong {
            type Output = FiLong;

            fn pow_int(self, num: $type) -> Self::Output {
                let mut res: FiLong = FiLong::one();
                let exponent = num.abs_diff(0);
                for _ in 0..exponent {
                    res *= &self;
                }
                if num < 0 {
                    res = 1 / res;
                }
                res
            }
        }
        impl PowInteger<&$type> for FiLong {
            type Output = FiLong;

            fn pow_int(self, num: &$type) -> Self::Output {
                let mut res: FiLong = FiLong::one();
                let exponent = num.abs_diff(0);
                for _ in 0..exponent {
                    res *= &self;
                }
                if *num < 0 {
                    res = 1 / res;
                }
                res
            }
        }
        impl PowInteger<$type> for &FiLong {
            type Output = FiLong;

            fn pow_int(self, num: $type) -> Self::Output {
                let mut res: FiLong = FiLong::one();
                let exponent = num.abs_diff(0);
                for _ in 0..exponent {
                    res *= self;
                }
                if num < 0 {
                    res = 1 / res;
                }
                res
            }
        }
        impl PowInteger<&$type> for &FiLong {
            type Output = FiLong;

            fn pow_int(self, num: &$type) -> Self::Output {
                let mut res: FiLong = FiLong::one();
                let exponent = num.abs_diff(0);
                for _ in 0..exponent {
                    res *= self;
                }
                if *num < 0 {
                    res = 1 / res;
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


impl Logarithm<FiLong> for FiLong {
    type Output = FiLong;

    fn log(self, base: FiLong) -> Self::Output {
        self.log2() / base.log2()
    }
}

impl Logarithm<&FiLong> for FiLong {
    type Output = FiLong;

    fn log(self, base: &FiLong) -> Self::Output {
        self.log2() / base.log2()
    }
}

impl Logarithm<FiLong> for &FiLong {
    type Output = FiLong;

    fn log(self, base: FiLong) -> Self::Output {
        self.log2() / base.log2()
    }
}

impl Logarithm<&FiLong> for &FiLong {
    type Output = FiLong;

    fn log(self, base: &FiLong) -> Self::Output {
        self.log2() / base.log2()
    }
}

impl FiLong {
    pub fn log2(&self) -> FiLong {
        match fl_log_2_long(&self) {
            Ok(val) => val,
            Err(e) => panic!("{}", e.msg()),
        }
    }

    pub fn ln(&self) -> FiLong {
        self.log2() * FiLong{sign: false, value: vec![13974485834865876094, 3]}
    }

    pub fn log10(&self) -> FiLong {
        self.log2() * FiLong{sign: false, value: vec![11656255492688567905, 1]}
    }
}