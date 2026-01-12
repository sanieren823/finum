use crate::fi::{FiBin, FiLong};
use std::time::Instant;
use crate::errors::FiError;
use crate::errors::FiErrorKind;
use crate::operations::arithm::Floor;


pub trait PowInteger<Rhs = Self> {
    type Output;

    fn pow_int(self, rhs: Rhs) -> Self::Output;
}

pub trait Logarithm<Rhs = Self> {
    type Output;

    fn log(self, rhs: Rhs) -> Self::Output;
}

pub trait Factorial {
    type Output;

    fn fact(self) -> Self::Output;
}

pub trait Termial {
    type Output;

    fn term(self) -> Self::Output;
}

pub trait Exponential {
    type Output;

    fn exp(self) -> Self::Output;
}

pub trait PowerOfTwo {
    type Output;

    fn pot(self) -> Self::Output;
}

pub trait PowReal<Rhs = Self> {
    type Output;

    fn pow_r(self, rhs: Rhs) -> Self::Output;
}

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

pub trait Root<Rhs = Self>{ // it's actually not quite the nth root as one might assume since real numbers are permittable for rhs
    type Output;

    fn root(self, rhs: Rhs) -> Self::Output;
}

pub trait Pow<Rhs = Self> {
    type Output; 

    fn pow(self, rhs: Rhs) -> Self::Output;
}

pub trait Trigonometry {
    type Output;

    fn sin(self) -> Self::Output;

    fn cos(self) -> Self::Output;

    fn tan(self) -> Self::Output;

    fn arcsin(self) -> Self::Output;

    fn arccos(self) -> Self::Output;

    fn arctan(self) -> Self::Output;

    fn sinh(self) -> Self::Output;

    fn cosh(self) -> Self::Output;

    fn tanh(self) -> Self::Output;

    fn coth(self) -> Self::Output;

    fn sech(self) -> Self::Output;

    fn csch(self) -> Self::Output;

    fn arcsinh(self) -> Self::Output;

    fn arccosh(self) -> Self::Output;

    fn arctanh(self) -> Self::Output;

    fn arccoth(self) -> Self::Output;

    fn arcsech(self) -> Self::Output;

    fn arccsch(self) -> Self::Output;

    fn cot(self) -> Self::Output;

    fn sec(self) -> Self::Output;

    fn csc(self) -> Self::Output;

    fn arccot(self) -> Self::Output;

    fn arcsec(self) -> Self::Output;

    fn arccsc(self) -> Self::Output;

    fn versin(self) -> Self::Output;

    fn coversin(self) -> Self::Output;

    fn vercos(self) -> Self::Output;

    fn covercos(self) -> Self::Output;

    fn exsec(self) -> Self::Output;

    fn excsc(self) -> Self::Output;
}


// PowReal --> x^n = e^(n * ln(x)) --> e^x can be approximated used in taylor series and pre calculated factorials

// traits to implement: Factorial + Termial + PowReal + Exponential + Trigonometry + Sqrt + Root
// macro for pow? or seperate trait?


// TODO: power int should have a check whether it's actually an integer










// impl FiBin {
//     pub fn pow(self, n: Self) -> Self {
//         if n.is_integer() {
//             gen_int_pow(self, n.clone() / FiBin::decimals())
//         } else {
//             binominal_series(self - 1.into(), n)
//         }
        
//     }

//     pub fn sqrt(self) -> Self {
//         FiBin::new()
//     }
//     pub fn nth_root(self, n: Self) -> Self {
//         let mut prev: FiBin = FiBin::new();
//         let mut guess: FiBin = self.clone() / n.clone();
//         let cur = Instant::now();
//         while guess != prev {
//             let single = Instant::now();
//             prev = guess.clone();
//             guess = newtons_method(prev.clone(), self.clone(), n.clone());
//             println!("{:?}", single.elapsed());
//         }
//         println!("{:?}", cur.elapsed());
//         guess
//     }
//     pub fn factorial(self) -> Self {
//         FiBin::new()
//     }
//     pub fn termial(self) -> Self {
//         let mut num = self.clone() % 1.into();
//         let mut counter = self.clone() / FiBin::decimals();
//         while !counter.is_zero() {
//             num += counter.clone() * FiBin::decimals();
//             if self.sign {
//                 counter += FiBin{sign: false, value: vec![true]};
//             } else {
//                 counter -= FiBin{sign: false, value: vec![true]};
//             }
//         }
//         num
//     }
//     pub fn ln(self) -> Self {
//         FiBin::new()
//     }

//     pub fn log(self, base: Self) -> Self {
//         FiBin::new()
//     }

//     pub fn log_10(self) -> FiBin {
//         self.ln() / FiBin::from(10)
//     }

//     pub fn log_2(self) -> FiBin {
//         self.ln() / FiBin::from(2)
//     }
// }


// fn gen_int_pow(base: FiBin, exponent: FiBin) -> FiBin {
//     let mut res: FiBin = 1.into();
//     let mut counter = exponent.clone(); 
//     let time = Instant::now();
//     while !counter.is_zero() {
//         let single = Instant::now();
//         res *= base.clone();
//         println!("{:?}", res.to_string());
//         println!("{:?}", single.elapsed());
//         if counter.sign {
//             counter += FiBin{sign: false, value: vec![true]};
//         } else {
//             counter -= FiBin{sign: false, value: vec![true]};
//         }
//     }
//     if exponent.sign {
//         res = FiBin::from(1) / res;
//     }
//     println!("{:?}", time.elapsed());
//     res
// }

// fn newtons_method(guess: FiBin, x: FiBin, n: FiBin) -> FiBin {
//     let n_1 = n.clone() - 1.into();
//     (guess.clone() * n_1.clone() + x / guess.pow(n_1)) / n
// }

// fn binominal_series(base: FiBin, exponent: FiBin) -> FiBin {
//     let mut sum = FiBin::new();
//     let mut temp: FiBin = 1.into();
//     let mut k: FiBin = FiBin::new();
//     let mut last: FiBin = FiBin::new();
//     while temp != sum {
//         last = binominal_coefficent(exponent.clone(), k.clone(), last.clone());
//         temp = sum.clone();
//         sum += base.clone().pow(k.clone()) * last.clone();

//         k += 1.into();
//         println!("{}", sum.to_string());
//     }
//     sum
// }


// fn binominal_coefficent_recursive(alpha: FiBin, k: FiBin) -> FiBin {
//     if k.is_zero() {
//         1.into()
//     } else {
//         let prev: FiBin = k.clone() - 1.into();
//         binominal_coefficent_recursive(alpha.clone(), prev.clone()) * (alpha - prev) / k
//     }
    
// }

// fn binominal_coefficent(alpha: FiBin, k: FiBin, prev: FiBin) -> FiBin {
//     if k.is_zero() {
//         1.into()
//     } else {
//         prev * ((alpha - (k.clone() - 1.into())) / k)
//     }
    
// }


// fn fl_log_2(num: FiBin) -> Result<FiBin, FiError>{
//     let mut shifted;
//     let mut res = FiBin::new();
//     if num.sign {
//         return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "Mind that the logarithmic function doesn't implement negative numbers."));
//     } else if num < FiBin::one() {
//         shifted = FiBin::one() / num;
//         res.sign = true;  
//     } else {
//         shifted = num
//     }
//     shifted = shifted / FiBin::decimals();
//     println!("{:?}", shifted.to_string());
    
//     while shifted.len() > 1 {
//         shifted = shifted.clone() >> 1;
//         if res.sign {
//             res -= 1.into();
//         } else {
//             res += 1.into();
//         }
//     }
//     Ok(res)
// }

fn fl_log_2_long(num: &FiLong) -> Result<FiLong, FiError>{
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
    // println!("{:?}", (decimals_log_2_long(&shifted, &FiLong::hundred()) + res.clone() * FiLong::hundred()).to_bin().to_string());
    res += decimals_log_2_long(&shifted, &FiLong::hundred()) * FiLong::hundredth(); // this adds precision; from what i measured 100 is usually enough
    res.sign = sign;
    // println!("{:?}", res.to_bin().to_string());
    Ok(res)
}

fn decimals_log_2_long(num: &FiLong, factor: &FiLong) -> FiLong{// input must be the residue --> between 1 and 2
    
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
        if !num.is_integer() {
            panic!("The pow_int() function requires the input to be an integer. Your input was: {:?}", num.to_string())
        }
        let mut res: FiLong = FiLong::one();
        let mut counter = num.absolute();
        while counter >= FiLong::one() {
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
        if !num.is_integer() {
            panic!("The pow_int() function requires the input to be an integer. Your input was: {:?}", num.to_string())
        }
        let mut res: FiLong = FiLong::one();
        let mut counter = num.absolute();
        while counter >= FiLong::one() {
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
        if !num.is_integer() {
            panic!("The pow_int() function requires the input to be an integer. Your input was: {:?}", num.to_string())
        }
        let mut res: FiLong = FiLong::one();
        let mut counter = num.absolute();
        while counter >= FiLong::one() {
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
        if !num.is_integer() {
            panic!("The pow_int() function requires the input to be an integer. Your input was: {:?}", num.to_string())
        }
        let mut res: FiLong = FiLong::one();
        let mut counter = num.absolute();
        while counter >= FiLong::one() {
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

    fn squared(&self) -> FiLong {
        self.pow_int(2)
    }

    fn cubed(&self) -> FiLong {
        self.pow_int(3)
    }
}
fn lookup_num_bernoulli(n: usize) -> FiLong {
    let arr = [FiLong::from("1"), FiLong::from("-1"), FiLong::from("1"), FiLong::from("-1"), FiLong::from("5"), FiLong::from("-691"), FiLong::from("7"), FiLong::from("-3691"), FiLong::from("43867"), FiLong::from("-174611"), FiLong::from("854513"), FiLong::from("-236364091"), FiLong::from("8553103"), FiLong::from("-23749461029"), FiLong::from("8615841276005"), FiLong::from("-7709321041217"), FiLong::from("2577687858367"), FiLong::from("-26315271553053477373"), FiLong::from("2929993913841559"), FiLong::from("-261082718496449122051")];
    arr[n].clone()
}

fn lookup_den_bernoulli(n: usize) -> FiLong {
    let arr = [FiLong::from("6"), FiLong::from("30"), FiLong::from("42"), FiLong::from("30"), FiLong::from("66"), FiLong::from("2730"), FiLong::from("6"), FiLong::from("510"), FiLong::from("798"), FiLong::from("330"), FiLong::from("138"), FiLong::from("2730"), FiLong::from("6"), FiLong::from("870"), FiLong::from("14322"), FiLong::from("510"), FiLong::from("6"), FiLong::from("-1919190"), FiLong::from("6"), FiLong::from("13530")];
    arr[n].clone()
}

fn lookup_num_bernoulli_calc(n: usize) -> FiLong {
    let arr = [FiLong{sign: false, value: vec![7766279631452241920, 5]}, FiLong{sign: true, value: vec![7766279631452241920, 5]}, FiLong{sign: false, value: vec![7766279631452241920, 5]}, FiLong{sign: true, value: vec![7766279631452241920, 5]}, FiLong{sign: false, value: vec![1937910009842106368, 27]}, FiLong{sign: true, value: vec![16943443957729198080, 3745]}, FiLong{sign: false, value: vec![17470469272746590208, 37]}, FiLong{sign: true, value: vec![17544573219291267072, 20008]}, FiLong{sign: false, value: vec![8919039647497060352, 237803]}, FiLong{sign: true, value: vec![2355636897145946112, 946568]}, FiLong{sign: false, value: vec![4705497475019964416, 4632324]}, FiLong{sign: true, value: vec![14735394401063796736, 1281332304]}, FiLong{sign: false, value: vec![4989132728540594176, 46366464]}, FiLong{sign: true, value: vec![149752777471426560, 128746086215]}, FiLong{sign: false, value: vec![9373257814498082816, 46706569146174]}, FiLong{sign: true, value: vec![7606701397510193152, 41792313106378]}, FiLong{sign: false, value: vec![798545564029419520, 13973673880155]}, FiLong{sign: true, value: vec![7111786010975928320, 13528164420866010793, 7]}, FiLong{sign: false, value: vec![15536869396137902080, 15883528833781620]}, FiLong{sign: true, value: vec![12262869239596056576, 13379703359431726623, 76]}];
    arr[n].clone()
}

fn lookup_den_bernoulli_calc(n: usize) -> FiLong {
    let arr = [FiLong{sign: false, value: vec![961635208879144960, 65]}, FiLong{sign: false, value: vec![10402312192664797184, 1951]}, FiLong{sign: false, value: vec![8737976563762462720, 6830]}, FiLong{sign: false, value: vec![5501720727113433088, 9107]}, FiLong{sign: false, value: vec![14840826552437964800, 32200]}, FiLong{sign: false, value: vec![8750847285274869760, 1953515]}, FiLong{sign: false, value: vec![13721827713163984896, 5919]}, FiLong{sign: false, value: vec![13458027427506683904, 663531]}, FiLong{sign: false, value: vec![14766147349596078080, 1323745]}, FiLong{sign: false, value: vec![14059156689068752896, 679794]}, FiLong{sign: false, value: vec![17866500431060926464, 345621]}, FiLong{sign: false, value: vec![9762824540299198464, 8169246]}, FiLong{sign: false, value: vec![17383537706369286144, 21141]}, FiLong{sign: false, value: vec![4877980077746290688, 3565507]}, FiLong{sign: false, value: vec![5300997791454068736, 67546554]}, FiLong{sign: false, value: vec![15043676404866613248, 2742597]}, FiLong{sign: false, value: vec![4521774043623325696, 36494]}, FiLong{sign: false, value: vec![12884462369710800896, 13108976794]}, FiLong{sign: false, value: vec![11946765188495048704, 45731]}, FiLong{sign: false, value: vec![1311290951863369728, 114420192]}];
    arr[n].clone()
}

pub fn bernouilli_coeffi(){
    for n in 1..21 {
        println!("{:?}", lookup_num_bernoulli(n - 1));
    }
}

pub fn cheb_lanczos(n: u8, m: u8) -> i128 {
    if n == m {
        match n {
            1 => 1,
            2 => 1,
            _ => 2 * cheb_lanczos(n - 1, n - 1),
        }
    } else if m == 1 {
        -cheb_lanczos(n - 2, 1)
    } else {
        2 * cheb_lanczos(n - 1, m - 1) - cheb_lanczos(n - 2, m)
    }
}

pub fn coefficients(k: u8, g: u8) -> FiLong {
    let base = FiLong::sqrt2() / FiLong::pi();
    let mut res = FiLong::new();
    for l in 0..=k {
        res += (l - FiLong::one_half()).fact() * (l + g + FiLong::one_half()).pow(-(l + FiLong::one_half())) * (l + g + FiLong::one_half()).exp() * cheb_lanczos(2 * k + 1, 2 * l + 1);
    }
    res * base
}


impl Factorial for FiLong{// TODO finish + impl for &FiLong
    type Output = FiLong;

    fn fact(self) -> Self::Output {
        if self < FiLong::new() {
            if self == -FiLong::one_half() {
                FiLong::from("1.7724538509055160272981674833411")
            } else {
                panic!("Make sure to input a positive number. Factorials can only be calculated of numbers 0 or larger.");
            }
            
        } else if self.is_zero() {
            FiLong::one()
        } else if self.is_integer() {
            let mut res = FiLong::one();
            let mut num = self;
            while num > FiLong::one() {
                res *= &num;
                num -= FiLong::one();
            }
            res
        } else {
            let decimals: FiLong = self.decimal_part() + 1;

            if self < FiLong::one() {
                lanczos(decimals, 10) / self
            } else {
                let mut counter = self;
                let mut reg = FiLong::one();
                while counter >= FiLong::two() {
                    reg *= &counter;
                    counter -= FiLong::one();
                }
                lanczos(decimals, 10) * reg
            }
        }
    }
}

fn sum_coef(z: FiLong, g: usize) -> FiLong {
    let mut res = FiLong::new();
    for k in 0..9 {
        res += &coef(k, g) * FiLong::neg_one().pow_int(k) * k_loop(k, -(&z)) / k_loop(k, &z + 1);
    }
    res
}

fn coef(k: usize, g: usize) -> FiLong {
    let base = FiLong::from(g).exp() * epsilon(k) * FiLong::neg_one().pow_int(k) / FiLong{sign: false, value: vec![10855154504875879234, 13]};
    let mut sum = FiLong::new();
    for r in 0..=k {
        sum += FiLong::neg_one().pow_int(r) * (FiLong::from(k).fact() / (FiLong::from((k - r)).fact() * FiLong::from(r).fact())) * k_loop(r, FiLong::from(k)) * (FiLong::e() * FiLong::million() / (r + g + FiLong::one_half())).pow(r + FiLong::one_half()) / (FiLong::million()).pow(r + FiLong::one_half());
        println!("fact: {:?}", (FiLong::from(k).fact() / (FiLong::from((k - r)).fact() * FiLong::from(r).fact())));
        println!("k: {:?}", k_loop(r, FiLong::from(k)));
        println!("pow: {:?}", (FiLong::e() / (r + g + FiLong::one_half())).pow(r + FiLong::one_half()));
        println!("-1: {:?}", FiLong::neg_one().pow_int(r));
    }
    println!("base: {:?}, sum: {:?}", &base, &sum);
    println!("{:?}, k: {:?}", &base * &sum, k);
    base * sum
}

fn epsilon(k: usize) -> FiLong {
    match k {
        0 => FiLong::one(),
        _ => FiLong::two(),
    }
}

fn k_loop(k: usize, subject: FiLong) -> FiLong {
    let mut res = FiLong::one();
    if k == 0 {
        res
    } else {
        for i in 0..k {
            res *= &subject + i;
        }
        res
    }
    
}


fn lanczos(z: FiLong, g: usize) -> FiLong {
    if z == FiLong::from("1.5") {
        FiLong::from("1.3293403881791370204736256125059")
    } else {
        let sqrt_2pi =  FiLong{sign: false, value: vec![10855154504875879234, 13]};
        let sum = &z + g + FiLong::one_half();
        sqrt_2pi * (&sum).pow(&z + FiLong::one_half()) * (-sum).exp() * sum_coef(z, g)
    }
    
}


impl Termial for FiLong {
    type Output = FiLong;

    fn term(self) -> Self::Output {
        let mut sum = self.decimal_part();
        let mut int_part = self.floor();
        
        while int_part >= FiLong::one() {
            sum += &int_part;
            int_part -= FiLong::one();
        }
        sum
    }
}

impl Termial for &FiLong {
    type Output = FiLong;

    fn term(self) -> Self::Output {
        let mut sum = self.decimal_part();
        let mut int_part = self.floor();
        
        while int_part >= FiLong::one() {
            sum += &int_part;
            int_part -= FiLong::one();
        }
        sum
    }
}

fn lookup_fact(n: usize) -> FiLong {
    let arr = [FiLong{sign: false, value: vec![7766279631452241920, 5]}, FiLong{sign: false, value: vec![7766279631452241920, 5]}, FiLong{sign: false, value: vec![15532559262904483840, 10]}, FiLong{sign: false, value: vec![9704189641294348288, 32]}, FiLong{sign: false, value: vec![1923270417758289920, 130]}, FiLong{sign: false, value: vec![9616352088791449600, 650]}, FiLong{sign: false, value: vec![2357880311620042752, 3903]}, FiLong{sign: false, value: vec![16505162181340299264, 27321]}, FiLong{sign: false, value: vec![2914088934755532800, 218575]}, FiLong{sign: false, value: vec![7780056339090243584, 1967176]}, FiLong{sign: false, value: vec![4013587096064229376, 19671764]}, FiLong{sign: false, value: vec![7255969909287419904, 216389406]}, FiLong{sign: false, value: vec![13284662616610832384, 2596672876]}, FiLong{sign: false, value: vec![6679917352554856448, 33756747397]}, FiLong{sign: false, value: vec![1285122567220232192, 472594463563]}, FiLong{sign: false, value: vec![830094434593931264, 7088916953446]}, FiLong{sign: false, value: vec![13281510953502900224, 113422671255136]}, FiLong{sign: false, value: vec![4424757325034684416, 1928185411337324]}, FiLong{sign: false, value: vec![5858655555786113024, 34707337404071836]}, FiLong{sign: false, value: vec![633991117678837760, 659439410677364890]}, FiLong{sign: false, value: vec![12679822353576755200, 13188788213547297800]}, FiLong{sign: false, value: vec![8021852393178136576, 263391378849979574, 15]}, FiLong{sign: false, value: vec![10460055986533040128, 5794610334699550637, 330]}, FiLong{sign: false, value: vec![773614732035751936, 4148829182122803352, 7597]}, FiLong{sign: false, value: vec![120009495148494848, 7338180002399522369, 182333]}, FiLong{sign: false, value: vec![3000237378712371200, 17433803396602094681, 4558334]}, FiLong{sign: false, value: vec![4219195551683444736, 10557030542625222926, 118516708]}, FiLong{sign: false, value: vec![3237815453195698176, 8338663545237744768, 3199951131]}, FiLong{sign: false, value: vec![16871856394641342464, 12121650382142234116, 89598631680]}, FiLong{sign: false, value: vec![9668489528150589440, 1039723681643308686, 2598360318739]}, FiLong{sign: false, value: vec![13353524738874408960, 12744966375589708979, 77950809562171]}, FiLong{sign: false, value: vec![8130897283496542208, 7712332095380394435, 2416475096427322]}, FiLong{sign: false, value: vec![1934296039955628032, 6986954093948450926, 77327203085674317]}, FiLong{sign: false, value: vec![8491537097407070208, 9208556215784261169, 2551797701827252473]}, FiLong{sign: false, value: vec![12011100206197112832, 17943006157312053905, 12974145567288377634, 4]}, FiLong{sign: false, value: vec![14560137595288813568, 815916999797131753, 11373237086063978440, 164]}, FiLong{sign: false, value: vec![7656119366529843200, 10926267918987191520, 3608165476693088289, 5926]}, FiLong{sign: false, value: vec![6575255455960924160, 16890287454625502319, 4374914121677405402, 219269]}, FiLong{sign: false, value: vec![10052034368290947072, 14641624769644333191, 226039960355440766, 8332231]}, FiLong{sign: false, value: vec![4647714815446351872, 17621043804842445990, 8815558453862189904, 324957009]}, FiLong{sign: false, value: vec![1441151880758558720, 3865477392734878202, 2134200754006115494, 12998280379]}, FiLong{sign: false, value: vec![3746994889972252672, 10910620512453593357, 13715254619412528798, 532929495543]}, FiLong{sign: false, value: vec![9799832789158199296, 15524203754021682218, 4191627730330109444, 22383038812837]}, FiLong{sign: false, value: vec![15564440312192434176, 3457974769388477220, 14219295740808741584, 962470668952000]}, FiLong{sign: false, value: vec![2305843009213693952, 4576937263416584789, 16906458163169426376, 42348709433888033]}, FiLong{sign: false, value: vec![11529215046068469760, 3047992042941247734, 4474110320532570675, 1905691924524961526]}, FiLong{sign: false, value: vec![13835058055282163712, 11080425459330534480, 2894889933693183281, 13874852233310023743, 4]}, FiLong{sign: false, value: vec![4611686018427387904, 4271162524667675347, 6932618367612752923, 6482012385736809368, 223]}, FiLong{sign: false, value: vec![0, 2101616373243348892, 724288318640211227, 15988689336014023826, 10720]}, FiLong{sign: false, value: vec![0, 10745481920376337628, 17043383539660798512, 8682526368885999603, 525322]}];
    arr[n].clone()
}

fn lookup_ln_two(n: usize) -> FiLong {
    let arr = [FiLong{sign: false, value: vec![7766279631452241920, 5]}, FiLong{sign: false, value: vec![13974485834865876094, 3]}, FiLong{sign: false, value: vec![11151813244401039235, 2]}, FiLong{sign: false, value: vec![14855721125183396356, 1]}, FiLong{sign: false, value: vec![4636765784598793573, 1]}, FiLong{sign: false, value: vec![16000269775714132108]}, FiLong{sign: false, value: vec![11090541883234759167]}, FiLong{sign: false, value: vec![7687377837246159501]}, FiLong{sign: false, value: vec![5328484273786185586]}, FiLong{sign: false, value: vec![3693423851032902237]}, FiLong{sign: false, value: vec![2560086328956311634]}, FiLong{sign: false, value: vec![1774516620906128084]}, FiLong{sign: false, value: vec![1230001192637843984]}, FiLong{sign: false, value: vec![852571858762291718]}, FiLong{sign: false, value: vec![590957780125834408]}, FiLong{sign: false, value: vec![409620719124186202]}, FiLong{sign: false, value: vec![283927446559866936]}, FiLong{sign: false, value: vec![196803509066556310]}, FiLong{sign: false, value: vec![136413797433787140]}, FiLong{sign: false, value: vec![94554839080705059]}, FiLong{sign: false, value: vec![65540420117090043]}, FiLong{sign: false, value: vec![45429157416875284]}, FiLong{sign: false, value: vec![31489092378721031]}, FiLong{sign: false, value: vec![21826575600702144]}, FiLong{sign: false, value: vec![15129029338905186]}, FiLong{sign: false, value: vec![10486644030870823]}, FiLong{sign: false, value: vec![7268787743533891]}, FiLong{sign: false, value: vec![5038339730519203]}, FiLong{sign: false, value: vec![3492310978912540]}, FiLong{sign: false, value: vec![2420685508671770]}, FiLong{sign: false, value: vec![1677891335358154]}, FiLong{sign: false, value: vec![1163025648389466]}, FiLong{sign: false, value: vec![806147949100061]}, FiLong{sign: false, value: vec![558779178032890]}, FiLong{sign: false, value: vec![387316211809101]}, FiLong{sign: false, value: vec![268467140200637]}, FiLong{sign: false, value: vec![186087241303063]}, FiLong{sign: false, value: vec![128985846647396]}, FiLong{sign: false, value: vec![89406175935780]}, FiLong{sign: false, value: vec![61971638774532]}, FiLong{sign: false, value: vec![42955466691246]}, FiLong{sign: false, value: vec![29774460626674]}, FiLong{sign: false, value: vec![20638083436072]}, FiLong{sign: false, value: vec![14305229345874]}, FiLong{sign: false, value: vec![9915629388356]}, FiLong{sign: false, value: vec![6872990554016]}, FiLong{sign: false, value: vec![4763994024531]}, FiLong{sign: false, value: vec![3302149026308]}, FiLong{sign: false, value: vec![2288875287374]}, FiLong{sign: false, value: vec![1586527452097]}];
    arr[n].clone()
}

impl PowerOfTwo for FiLong{// fix the floor function
    type Output = FiLong;

    fn pot(self) -> Self::Output {
        let mut decimals = self.decimal_part();
        let mut int_part = self.floor();
        if decimals.absolute() > FiLong::one_half() {
            int_part.gen_increment(decimals.sign);
            decimals.gen_decrement(decimals.sign);
        }
        let mut sum = FiLong::ten() + &decimals * FiLong::ln2() * FiLong::ten();
        for i in 2..16 {
            sum += FiLong::ten() * (&decimals).pow_int(i) * lookup_ln_two(i) / lookup_fact(i);
        }
        sum * FiLong::two().pow_int(int_part) / FiLong::ten()
    }
}

impl PowerOfTwo for &FiLong {
    type Output = FiLong;

    fn pot(self) -> Self::Output {
        let mut decimals = self.decimal_part();
        let mut int_part = self.floor();
        if decimals.absolute() > FiLong::one_half() {
            int_part.gen_increment(decimals.sign);
            decimals.gen_decrement(decimals.sign);
            
        }
        let mut sum = FiLong::ten() + &decimals * FiLong::ln2() * FiLong::ten();
        for i in 2..16 {
            sum += FiLong::ten() * (&decimals).pow_int(i) * lookup_ln_two(i) / lookup_fact(i);
        }
        sum * FiLong::two().pow_int(int_part) / FiLong::ten()
    }
}

impl PowReal<FiLong> for FiLong{// lower precision only around 17digits
    type Output = FiLong;

    fn pow_r(self, rhs: FiLong) -> Self::Output {
        let exponent = rhs * self.log2();
        exponent.pot()
    }
}

impl PowReal<&FiLong> for FiLong {
    type Output = FiLong;

    fn pow_r(self, rhs: &FiLong) -> Self::Output {
        let exponent = rhs * self.log2();
        exponent.pot()
    }
}

impl PowReal<FiLong> for &FiLong {
    type Output = FiLong;

    fn pow_r(self, rhs: FiLong) -> Self::Output {
        let exponent = rhs * self.log2();
        exponent.pot()
    }
}

impl PowReal<&FiLong> for &FiLong {
    type Output = FiLong;

    fn pow_r(self, rhs: &FiLong) -> Self::Output {
        let exponent = rhs * self.log2();
        exponent.pot()
    }
}

impl Exponential for FiLong {
    type Output = FiLong;

    fn exp(self) -> Self::Output {
        let mut decimals = self.decimal_part();
        let mut int_part = self.floor();
        if decimals.absolute() > FiLong::one_half() {
            int_part.gen_increment(decimals.sign);
            decimals.gen_decrement(decimals.sign);
        }
        let mut sum = FiLong::ten() + FiLong::ten() * &decimals;
        for i in 2..18 {
            sum += FiLong::ten() * (&decimals).pow_int(i) / lookup_fact(i);
        }
        sum * FiLong::e().pow_int(int_part) * FiLong::tenth()
    }
}

impl Exponential for &FiLong {
    type Output = FiLong;

    fn exp(self) -> Self::Output {
        let mut decimals = self.decimal_part();
        let mut int_part = self.floor();
        if decimals.absolute() > FiLong::one_half() {
            int_part.gen_increment(decimals.sign);
            decimals.gen_decrement(decimals.sign);
        }
        let mut sum = FiLong::ten() + FiLong::ten() * &decimals;
        for i in 2..18 {
            sum += FiLong::ten() * (&decimals).pow_int(i) / lookup_fact(i);
        }
        sum * FiLong::e().pow_int(int_part) * FiLong::tenth()
    }
}

impl Root<FiLong> for FiLong {
    type Output = FiLong;

    fn root(self, num: FiLong) -> Self::Output {
        self.pow(FiLong::one() / num)
    }
}

impl Root<&FiLong> for FiLong {
    type Output = FiLong;

    fn root(self, num: &FiLong) -> Self::Output {
        self.pow(FiLong::one() / num)
    }
}

impl Root<FiLong> for &FiLong {
    type Output = FiLong;

    fn root(self, num: FiLong) -> Self::Output {
        self.pow(FiLong::one() / num)
    }
}

impl Root<&FiLong> for &FiLong {
    type Output = FiLong;

    fn root(self, num: &FiLong) -> Self::Output {
        self.pow(FiLong::one() / num)
    }
}

impl Pow<FiLong> for FiLong {
    type Output = FiLong;

    fn pow(self, num: FiLong) -> Self::Output {
        if num.is_integer() {
            self.pow_int(num)
        } else {
            self.pow_r(num)
        }
    }
}

impl Pow<&FiLong> for FiLong {
    type Output = FiLong;

    fn pow(self, num: &FiLong) -> Self::Output {
        if num.is_integer() {
            self.pow_int(num)
        } else {
            self.pow_r(num)
        }
    }
}

impl Pow<FiLong> for &FiLong {
    type Output = FiLong;

    fn pow(self, num: FiLong) -> Self::Output {
        if num.is_integer() {
            self.pow_int(num)
        } else {
            self.pow_r(num)
        }
    }
}

impl Pow<&FiLong> for &FiLong {
    type Output = FiLong;

    fn pow(self, num: &FiLong) -> Self::Output {
        if num.is_integer() {
            self.pow_int(num)
        } else {
            self.pow_r(num)
        }
    }
}

macro_rules! pow_for_int {
    ($type:ty) => {
        impl Pow<$type> for FiLong {
            type Output = FiLong;

            fn pow(self, num: $type) -> Self::Output {
                self.pow_int(num)
            }
        }
        impl Pow<&$type> for FiLong {
            type Output = FiLong;

            fn pow(self, num: &$type) -> Self::Output {
                self.pow_int(num)
            }
        }
        impl Pow<$type> for &FiLong {
            type Output = FiLong;

            fn pow(self, num: $type) -> Self::Output {
                self.pow_int(num)
            }
        }
        impl Pow<&$type> for &FiLong {
            type Output = FiLong;

            fn pow(self, num: &$type) -> Self::Output {
                self.pow_int(num)
            }  
        }
    };
}

pow_for_int!(isize);
pow_for_int!(i8);
pow_for_int!(i16);
pow_for_int!(i32);
pow_for_int!(i64);
pow_for_int!(i128);
pow_for_int!(usize);
pow_for_int!(u8);
pow_for_int!(u16);
pow_for_int!(u32);
pow_for_int!(u64);
pow_for_int!(u128);

impl Sqrt for FiLong {
    type Output = FiLong;

    fn sqrt(self) -> Self::Output {
        let mut guess = FiLong::one_eighth() * FiLong::two().pow_int(self.bits() / 2);
        let mut prev = FiLong::new();
        while guess != prev {
            prev = guess.clone();
            guess = &guess * (guess.squared() + FiLong::three() * &self) /  (FiLong::three() * guess.squared() + &self);
        }
        guess
    }
}

impl Sqrt for &FiLong {
    type Output = FiLong;

    fn sqrt(self) -> Self::Output {
        let mut guess = FiLong::one_eighth() * FiLong::two().pow_int(self.bits() / 2);
        let mut prev = FiLong::new();
        while guess != prev {
            prev = guess.clone();
            guess = &guess * (guess.squared() + FiLong::three() * self) /  (FiLong::three() * guess.squared() + self);
        }
        guess
    }
}

impl Trigonometry for FiLong{
    type Output = FiLong;

    fn sin(self) -> Self::Output {
        let mut x = &self % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&self.sign);
        }
        let mut switch = true;
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&x).pow(2 * n + 1);
            let fact = lookup_fact(2 * n + 1).value_of_sign(&switch);
            sum += FiLong::ten() * &pow / &fact;
            switch ^= true;
        }
        sum / FiLong::ten()
    }

    fn cos(self) -> Self::Output {
        let mut x = &self % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&self.sign);
        }
        let mut switch = true;
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&x).pow(2 * n);
            let fact = lookup_fact(2 * n).value_of_sign(&switch);
            sum += FiLong::ten() * &pow / &fact;
            switch ^= true;
        }
        sum / FiLong::ten()
    }

    fn tan(self) -> Self::Output{// seperate implementation? + exception
        let sin = (&self).sin();
        let cos = (&self).cos();
        if cos == FiLong::new() {
            panic!("The tangent function is not defined for values where x % pi/2 = 0")
        } else {
            sin / cos
        }
    }

    fn arcsin(self) -> Self::Output {
        if self.absolute() > FiLong::one() {
            panic!("The inverse functions of trigonometric functions are defined for [-1; 1]")
        }
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&self).pow(2 * n + 1) + lookup_fact(2 * n);
            let fact = lookup_fact(n).pow_int(2) * FiLong::four().pow_int(n) * FiLong::from(2 * n + 1);
            sum += FiLong::ten() * &pow / &fact;
        }
        sum / FiLong::ten()
    }

    fn arccos(self) -> Self::Output {
        FiLong::pi_div_two() - self.arcsin()
    }

    fn arctan(self) -> Self::Output {
        if self.absolute() > FiLong::one() {
            panic!("The inverse functions of trigonometric functions are defined for [-1; 1]")
        }
        let mut switch = true;
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&self).pow(2 * n + 1);
            let fact = FiLong::from(2 * n + 1).value_of_sign(&switch);
            sum += FiLong::ten() * &pow / &fact;
            switch ^= true;
        }
        sum / FiLong::ten()
    }

    fn sinh(self) -> Self::Output {
        let mut x = &self % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&self.sign);
        }
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&x).pow(2 * n + 1);
            let fact = lookup_fact(2 * n + 1);
            sum += FiLong::ten() * &pow / &fact;
        }
        sum / FiLong::ten()
    }

    fn cosh(self) -> Self::Output {
        let mut x = &self % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&self.sign);
        }
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&x).pow(2 * n);
            let fact = lookup_fact(2 * n);
            sum += FiLong::ten() * &pow / &fact;
        }
        sum / FiLong::ten()
    }

    fn tanh(self) -> Self::Output {
        let sinh = (&self).sinh();
        let cosh = (&self).cosh();
        if cosh == FiLong::new() {
            panic!("The tangent function is not defined for values where x % pi/2 = 0")
        } else {
            sinh / cosh
        }
    }

    fn coth(self) -> Self::Output {
        let sinh = (&self).sinh();
        let cosh = (&self).cosh();
        if sinh == FiLong::new() {
            panic!("The cotangent function is not defined for values where x % pi/2 = 0")
        } else {
            cosh / sinh
        }
    }

    fn sech(self) -> Self::Output {
        FiLong::one() / self.cos()
    }

    fn csch(self) -> Self::Output {
        FiLong::one() / self.sin()
    }


    fn arcsinh(self) -> Self::Output {
        (&self + ((&self).pow_int(2u8) + FiLong::one()).sqrt()).ln()
    }

    fn arccosh(self) -> Self::Output {
        (&self + ((&self).pow_int(2u8) - FiLong::one()).sqrt()).ln()
    }

    fn arctanh(self) -> Self::Output {
        FiLong::one_half() * ((FiLong::one() + &self) / (FiLong::one() - &self)).ln()
    }

    fn arccoth(self) -> Self::Output {
        FiLong::one_half() * ((&self + FiLong::one()) / (&self - FiLong::one())).ln()
    }

    fn arcsech(self) -> Self::Output {
        (self.inverse() + (self.pow_int(-2i8) - FiLong::one()).sqrt()).ln()
    }

    fn arccsch(self) -> Self::Output {
       (self.inverse() + (self.pow_int(-2i8) + FiLong::one()).sqrt()).ln()
    }

    fn cot(self) -> Self::Output {
        let sin = (&self).sin();
        let cos = (&self).cos();
        if sin == FiLong::new() {
            panic!("The cotangent function is not defined for values where x % pi/2 = 0")
        } else {
            cos / sin
        }
    }

    fn sec(self) -> Self::Output {
        FiLong::one() / self.cos()
    }

    fn csc(self) -> Self::Output {
        FiLong::one() / self.sin()
    }

    fn arccot(self) -> Self::Output {
        self.inverse().arctan()
    }

    fn arcsec(self) -> Self::Output {
        self.inverse().arccos()
    }

    fn arccsc(self) -> Self::Output {
        self.inverse().arcsin()
    }

    fn versin(self) -> Self::Output {
        FiLong::one() - self.cos()
    }

    fn coversin(self) -> Self::Output {
        FiLong::one() - self.sin()
    }

    fn vercos(self) -> Self::Output {
        FiLong::one() + self.cos()
    }

    fn covercos(self) -> Self::Output {
        FiLong::one() + self.sin()
    }

    fn exsec(self) -> Self::Output {
        self.sec() - FiLong::one()
    }

    fn excsc(self) -> Self::Output {
        self.csc() - FiLong::one()
    }
}

impl Trigonometry for &FiLong{
    type Output = FiLong;

    fn sin(self) -> Self::Output {
        let mut x = self % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&self.sign);
        }
        let mut switch = true;
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&x).pow(2 * n + 1);
            let fact = lookup_fact(2 * n + 1).value_of_sign(&switch);
            sum += FiLong::ten() * &pow / &fact;
            switch ^= true;
        }
        sum / FiLong::ten()
    }

    fn cos(self) -> Self::Output {
        let mut x = self % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&self.sign);
        }
        let mut switch = true;
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&x).pow(2 * n);
            let fact = lookup_fact(2 * n).value_of_sign(&switch);
            sum += FiLong::ten() * &pow / &fact;
            switch ^= true;
        }
        sum / FiLong::ten()
    }

    fn tan(self) -> Self::Output{// seperate implementation? + exception
        let sin = self.sin();
        let cos = self.cos();
        if cos == FiLong::new() {
            panic!("The tangent function is not defined for values where x % pi/2 = 0")
        } else {
            sin / cos
        }
    }

    fn arcsin(self) -> Self::Output {
        if self.absolute() > FiLong::one() {
            panic!("The inverse functions of trigonometric functions are defined for [-1; 1]")
        }
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&self).pow(2 * n + 1) + lookup_fact(2 * n);
            let fact = lookup_fact(n).pow_int(2) * FiLong::four().pow_int(n) * FiLong::from(2 * n + 1);
            sum += FiLong::ten() * &pow / &fact;
        }
        sum / FiLong::ten()
    }

    fn arccos(self) -> Self::Output {
        FiLong::pi_div_two() - self.arcsin()
    }

    fn arctan(self) -> Self::Output {
        if self.absolute() > FiLong::one() {
            panic!("The inverse functions of trigonometric functions are defined for [-1; 1]")
        }
        let mut switch = true;
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&self).pow(2 * n + 1);
            let fact = FiLong::from(2 * n + 1).value_of_sign(&switch);
            sum += FiLong::ten() * &pow / &fact;
            switch ^= true;
        }
        sum / FiLong::ten()
    }

    fn sinh(self) -> Self::Output {
        let mut x = self % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&self.sign);
        }
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&x).pow(2 * n + 1);
            let fact = lookup_fact(2 * n + 1);
            sum += FiLong::ten() * &pow / &fact;
        }
        sum / FiLong::ten()
    }

    fn cosh(self) -> Self::Output {
        let mut x = self % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&self.sign);
        }
        let mut sum = FiLong::ten();
        for n in 1..13 {
            let pow = (&x).pow(2 * n);
            let fact = lookup_fact(2 * n);
            sum += FiLong::ten() * &pow / &fact;
        }
        sum / FiLong::ten()
    }

    fn tanh(self) -> Self::Output {
        let sinh = self.sinh();
        let cosh = self.cosh();
        if cosh == FiLong::new() {
            panic!("The tangent function is not defined for values where x % pi/2 = 0")
        } else {
            sinh / cosh
        }
    }

    fn coth(self) -> Self::Output {
        let sinh = self.sinh();
        let cosh = self.cosh();
        if sinh == FiLong::new() {
            panic!("The cotangent function is not defined for values where x % pi/2 = 0")
        } else {
            cosh / sinh
        }
    }

    fn sech(self) -> Self::Output {
        FiLong::one() / self.cos()
    }

    fn csch(self) -> Self::Output {
        FiLong::one() / self.sin()
    }


    fn arcsinh(self) -> Self::Output {
        (self + (self.pow_int(2u8) + FiLong::one()).sqrt()).ln()
    }

    fn arccosh(self) -> Self::Output {
        (self + (self.pow_int(2u8) - FiLong::one()).sqrt()).ln()
    }

    fn arctanh(self) -> Self::Output {
        FiLong::one_half() * ((FiLong::one() + self) / (FiLong::one() - self)).ln()
    }

    fn arccoth(self) -> Self::Output {
        FiLong::one_half() * ((self + FiLong::one()) / (self - FiLong::one())).ln()
    }

    fn arcsech(self) -> Self::Output {
        (self.inverse() + (self.pow_int(-2i8) - FiLong::one()).sqrt()).ln()
    }

    fn arccsch(self) -> Self::Output {
       (self.inverse() + (self.pow_int(-2i8) + FiLong::one()).sqrt()).ln()
    }

    fn cot(self) -> Self::Output {
        let sin = self.sin();
        let cos = self.cos();
        if sin == FiLong::new() {
            panic!("The cotangent function is not defined for values where x % pi/2 = 0")
        } else {
            cos / sin
        }
    }

    fn sec(self) -> Self::Output {
        FiLong::one() / self.cos()
    }

    fn csc(self) -> Self::Output {
        FiLong::one() / self.sin()
    }

    fn arccot(self) -> Self::Output {
        self.inverse().arctan()
    }

    fn arcsec(self) -> Self::Output {
        self.inverse().arccos()
    }

    fn arccsc(self) -> Self::Output {
        self.inverse().arcsin()
    }

    fn versin(self) -> Self::Output {
        FiLong::one() - self.cos()
    }

    fn coversin(self) -> Self::Output {
        FiLong::one() - self.sin()
    }

    fn vercos(self) -> Self::Output {
        FiLong::one() + self.cos()
    }

    fn covercos(self) -> Self::Output {
        FiLong::one() + self.sin()
    }

    fn exsec(self) -> Self::Output {
        self.sec() - FiLong::one()
    }

    fn excsc(self) -> Self::Output {
        self.csc() - FiLong::one()
    }
}
