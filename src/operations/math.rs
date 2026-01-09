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

pub trait Root<Rhs = Self>{// it's actually not quite the nth root as one might assume since real numbers are permittable for rhs
    type Output;

    fn root(self, rhs: Rhs) -> Self::Output;
}

pub trait Pow<Rhs = Self> {
    type Output; 

    fn pow(self, rhs: Rhs) -> Self::Output;
}


// PowReal --> x^n = e^(n * ln(x)) --> e^x can be approximated used in taylor series and pre calculated factorials

// traits to implement: Factorial + Termial + PowReal + Exponential + Trigonometry + Sqrt + Root
// macro for pow? or seperate trait?


// TODO: power int should have a check whether it's actually an integer










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


fn fl_log_2(num: FiBin) -> Result<FiBin, FiError>{
    let mut shifted;
    let mut res = FiBin::new();
    if num.sign {
        return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "Mind that the logarithmic function doesn't implement negative numbers."));
    } else if num < FiBin::one() {
        shifted = FiBin::one() / num;
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

impl FiLong{// helper functions for the log2 calculation
    #[inline(always)]
    fn hundred() -> FiLong {
        FiLong{sign: false, value: vec![1864712049423024128, 542]}
    }
    #[inline(always)]
    fn hundredth() -> FiLong {
        FiLong{sign: false, value: vec![1000000000000000000]}
    }
}

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
    println!("{:?}", (decimals_log_2_long(&shifted, &FiLong::hundred()) + res.clone() * FiLong::hundred()).to_bin().to_string());
    res += decimals_log_2_long(&shifted, &FiLong::hundred()) * FiLong::hundredth(); // this adds precision; from what i measured 100 is usually enough
    res.sign = sign;
    println!("{:?}", res.to_bin().to_string());
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





impl Factorial for FiLong{// TODO finish + impl for &FiLong
    type Output = FiLong;

    fn fact(self) -> Self::Output {
        // println!("{:?}", &self % FiLong::one());
        if self < FiLong::new() {
            panic!("negative lol") // TODO obv
        } else if self.spruce_up() == FiLong::new() {
            FiLong::one()
        } else if self.is_integer() {
            let mut res = FiLong::one();
            let mut num = self;
            while num > FiLong::one() {
                res *= &num;
                num -= FiLong::one();
                // println!("{:?}", res);
            }
            res
        } else {
            FiLong::new()
        }
    }
}

fn lookup_inverse_fact(n: usize) -> FiLong {
    let arr = [FiLong{sign: false, value: vec![7886392056514347008, 5421010862]}, FiLong{sign: false, value: vec![7886392056514347008, 5421010862]}, FiLong{sign: false, value: vec![3943196028257173504, 2710505431]}, FiLong{sign: false, value: vec![7463313367322241707, 903501810]}, FiLong{sign: false, value: vec![11089200378685336235, 225875452]}, FiLong{sign: false, value: vec![9596537705220887893, 45175090]}, FiLong{sign: false, value: vec![13897252333343182393, 7529181]}, FiLong{sign: false, value: vec![7255820068680326518, 1075597]}, FiLong{sign: false, value: vec![12436192554653510575, 134449]}, FiLong{sign: false, value: vec![15729266785624485765, 14938]}, FiLong{sign: false, value: vec![16330321937530089869, 1493]}, FiLong{sign: false, value: vec![14900388593382409345, 135]}, FiLong{sign: false, value: vec![5853385067875922016, 11]}, FiLong{sign: false, value: vec![16059043836821614599]}, FiLong{sign: false, value: vec![1147074559772972471]}, FiLong{sign: false, value: vec![76471637318198165]}, FiLong{sign: false, value: vec![4779477332387385]}, FiLong{sign: false, value: vec![281145725434552]}, FiLong{sign: false, value: vec![15619206968586]}, FiLong{sign: false, value: vec![822063524662]}, FiLong{sign: false, value: vec![41103176233]}, FiLong{sign: false, value: vec![1957294106]}, FiLong{sign: false, value: vec![88967914]}, FiLong{sign: false, value: vec![3868170]}, FiLong{sign: false, value: vec![161174]}, FiLong{sign: false, value: vec![6447]}, FiLong{sign: false, value: vec![248]}, FiLong{sign: false, value: vec![9]}];
    arr[n].clone()
}

fn lookup_ln_two(n: usize) -> FiLong {
    let arr = [FiLong{sign: false, value: vec![7766279631452241920, 5]}, FiLong{sign: false, value: vec![13974485834865876094, 3]}, FiLong{sign: false, value: vec![11151813244401039235, 2]}, FiLong{sign: false, value: vec![14855721125183396356, 1]}, FiLong{sign: false, value: vec![4636765784598793573, 1]}, FiLong{sign: false, value: vec![16000269775714132108]}, FiLong{sign: false, value: vec![11090541883234759167]}, FiLong{sign: false, value: vec![7687377837246159501]}, FiLong{sign: false, value: vec![5328484273786185586]}, FiLong{sign: false, value: vec![3693423851032902237]}, FiLong{sign: false, value: vec![2560086328956311634]}, FiLong{sign: false, value: vec![1774516620906128084]}, FiLong{sign: false, value: vec![1230001192637843985]}, FiLong{sign: false, value: vec![852571858762291718]}, FiLong{sign: false, value: vec![590957780125834408]}, FiLong{sign: false, value: vec![409620719124186202]}, FiLong{sign: false, value: vec![283927446559866936]}, FiLong{sign: false, value: vec![196803509066556310]}, FiLong{sign: false, value: vec![136413797433787140]}, FiLong{sign: false, value: vec![94554839080705059]}, FiLong{sign: false, value: vec![65540420117090043]}, FiLong{sign: false, value: vec![45429157416875284]}, FiLong{sign: false, value: vec![31489092378721031]}, FiLong{sign: false, value: vec![21826575600702144]}, FiLong{sign: false, value: vec![15129029338905186]}, FiLong{sign: false, value: vec![10486644030870823]}, FiLong{sign: false, value: vec![7268787743533891]}, FiLong{sign: false, value: vec![5038339730519203]}, FiLong{sign: false, value: vec![3492310978912540]}, FiLong{sign: false, value: vec![2420685508671770]}, FiLong{sign: false, value: vec![1677891335358154]}, FiLong{sign: false, value: vec![1163025648389466]}, FiLong{sign: false, value: vec![806147949100061]}, FiLong{sign: false, value: vec![558779178032890]}, FiLong{sign: false, value: vec![387316211809101]}, FiLong{sign: false, value: vec![268467140200637]}, FiLong{sign: false, value: vec![186087241303063]}, FiLong{sign: false, value: vec![128985846647396]}, FiLong{sign: false, value: vec![89406175935780]}, FiLong{sign: false, value: vec![61971638774532]}, FiLong{sign: false, value: vec![42955466691246]}, FiLong{sign: false, value: vec![29774460626674]}, FiLong{sign: false, value: vec![20638083436072]}, FiLong{sign: false, value: vec![14305229345874]}, FiLong{sign: false, value: vec![9915629388356]}, FiLong{sign: false, value: vec![6872990554016]}, FiLong{sign: false, value: vec![4763994024531]}, FiLong{sign: false, value: vec![3302149026308]}, FiLong{sign: false, value: vec![2288875287374]}, FiLong{sign: false, value: vec![1586527452097]}];
    arr[n].clone()
}


impl PowerOfTwo for FiLong{// fix the floor function
    type Output = FiLong;

    fn pot(self) -> Self::Output {
        let mut sum = FiLong::new();
        let decimals = &self % FiLong::one();
        let int_part = self.floor();
        for i in 0..25 {
            sum += (&decimals).pow_int(i) * lookup_ln_two(i) * lookup_inverse_fact(i);
        }
        // println!("{:?}", FiLong::from(2).pow_int(int_part.clone()).to_bin().to_string());
        sum * FiLong::from(2).pow_int(int_part.parse::<u64>().unwrap()) * FiLong::billionth()
    }
}

impl PowerOfTwo for &FiLong {
    type Output = FiLong;

    fn pot(self) -> Self::Output {
        let mut sum = FiLong::new();
        let decimals = self % FiLong::one();
        for i in 0..23 {
            sum += lookup_ln_two(i) * lookup_inverse_fact(i) * (&decimals).pow_int(i);
        }

        sum * (self.floor().pow_int(2))
    }
}

impl PowReal<FiLong> for FiLong {
    type Output = FiLong;

    fn pow_r(self, rhs: FiLong) -> Self::Output {
        let exponent = rhs * self.log2();
        println!("{:?}", exponent.to_bin().to_string());
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
        let mut sum = FiLong::new();
        let decimals = &self % FiLong::one();
        let int_part = self.floor();
        for i in 0..28 {
            sum += lookup_inverse_fact(i) * (&decimals).pow_int(i);
        }
        // println!("{:?}", FiLong::from(2).pow_int(int_part.clone()).to_bin().to_string());
        sum * FiLong::e().pow_int(int_part.parse::<u64>().unwrap() + 1) * FiLong::billionth()
    }
}

impl Exponential for &FiLong {
    type Output = FiLong;

    fn exp(self) -> Self::Output {
        let mut sum = FiLong::new();
        let decimals = self % FiLong::one();
        let int_part = self.floor();
        for i in 0..28 {
            sum += lookup_inverse_fact(i) * (&decimals).pow_int(i);
        }
        // println!("{:?}", FiLong::from(2).pow_int(int_part.clone()).to_bin().to_string());
        sum * FiLong::e().pow_int(int_part.parse::<u64>().unwrap() + 1) * FiLong::billionth()
    }
}



pub fn pow_r_experimental(num: &FiLong, rhs: &FiLong) -> FiLong {
    let mut shifted;
    let mut res: FiLong = FiLong::new();
    let mut sign = false;
    if num.sign {
        // return Err(FiError::new(FiErrorKind::NumberCannotBeNegative, "Mind that the logarithmic function doesn't implement negative numbers."));
        panic!("reason")
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
    res *= 100;
    res += decimals_log_2_long(&shifted, &FiLong::hundred()); // * FiLong::hundredth(); // this adds precision; from what i measured 100 is usually enough
    res.sign = sign;
    // println!("{:?}", res.to_bin().to_string());
    let exponent = rhs * res.log2();
    println!("{:?}", exponent.to_bin().to_string());
    let mut sum = FiLong::new();
    let decimals = &exponent % FiLong::one();
    let int_part = exponent.floor();
    for i in 0..25 {
        sum += (&decimals).pow_int(i) * lookup_ln_two(i) * lookup_inverse_fact(i);
    }
    // println!("{:?}", FiLong::from(2).pow_int(int_part.clone()).to_bin().to_string());
    sum * FiLong::from(2).pow_int(int_part.parse::<u64>().unwrap() + 1) * FiLong::billionth()
}