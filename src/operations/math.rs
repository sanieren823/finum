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

    fn arcsinh(self) -> Self::Output;

    fn arccosh(self) -> Self::Output;

    fn arctanh(self) -> Self::Output;

    fn cot(self) -> Self::Output;

    fn sec(self) -> Self::Output;

    fn csc(self) -> Self::Output;

    fn versin(self) -> Self::Output;

    fn coversin(self) -> Self::Output;

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

// impl FiLong{// helper functions for the log2 calculation
//     #[inline(always)]
//     fn hundred() -> FiLong {
//         FiLong{sign: false, value: vec![1864712049423024128, 542]}
//     }
//     #[inline(always)]
//     fn hundredth() -> FiLong {
//         FiLong{sign: false, value: vec![1000000000000000000]}
//     }
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

// pub fn tenth_log_long(num: &FiLong) -> FiLong{// TODO: remove
//     let mut shifted;
//     let mut res: FiLong = FiLong::new();
//     let mut sign = false;
//     if num.sign {
//         panic!("readsoon");
//     } else if *num < FiLong::one() {
//         shifted = FiLong::one() / num;
//         sign = true;
//     } else {
//         shifted = num.clone();
//     }
//     while shifted >= FiLong::two() {
//         shifted >>= 1;
//         res += 1;
//     }
//     res *= 1000;
//     res += decimals_log_2_long(&shifted, &FiLong::ten()); // this adds precision; from what i measured 100 is usually enough
//     res.sign = sign;
//     // println!("{:?}", res.to_bin().to_string());
//     res
// }


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

impl PowReal<FiLong> for FiLong { // lower precision only around 17digits
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
        println!("{:?}", int_part.to_bin().to_string());
        let mut sum = FiLong::ten() + FiLong::ten() * &decimals;
        for i in 2..18 {
            sum += FiLong::ten() * (&decimals).pow_int(i) / lookup_fact(i);
        }
        println!("{:?}", int_part.to_bin().to_string());
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
        println!("{:?}", int_part.to_bin().to_string());
        let mut sum = FiLong::ten() + FiLong::ten() * &decimals;
        for i in 2..18 {
            sum += FiLong::ten() * (&decimals).pow_int(i) / lookup_fact(i);
        }
        println!("{:?}", int_part.to_bin().to_string());
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

// impl Trigonometry for FiLong{
//     type Output = FiLong;

//     fn sin(self) -> Self::Output {

//     }

//     fn cos(self) -> Self::Output {
//         let mut x = &self % FiLong::pi();
//         if x > FiLong::pi_div_two() {
//             x -= FiLong::pi().value_of_sign(&self.sign);
//         }
//         let mut switch = true;
//         let mut sum = FiLong::one();
//         for n in 1..20 {
//             sum += (&self).pow(2 * n) / lookup_fact(2 * n).value_of_sign(&switch);
//             switch ^= true;
//         }
//         sum
//     }

//     fn tan(self) -> Self::Output;

//     fn arcsin(self) -> Self::Output;

//     fn arccos(self) -> Self::Output;

//     fn arctan(self) -> Self::Output;

//     fn sinh(self) -> Self::Output;

//     fn cosh(self) -> Self::Output;

//     fn tanh(self) -> Self::Output;

//     fn arcsinh(self) -> Self::Output;

//     fn arccosh(self) -> Self::Output;

//     fn arctanh(self) -> Self::Output;

//     fn cot(self) -> Self::Output;

//     fn sec(self) -> Self::Output;

//     fn csc(self) -> Self::Output;

//     fn versin(self) -> Self::Output;

//     fn coversin(self) -> Self::Output;

//     fn exsec(self) -> Self::Output;

//     fn excsc(self) -> Self::Output;
// }

pub fn cos(num: FiLong) -> FiLong {
    let mut x = &num % FiLong::pi();
        if x > FiLong::pi_div_two() {
            x -= FiLong::pi().value_of_sign(&num.sign);
        }
        let mut switch = true;
        let mut sum = FiLong::one();
        println!("{:?}", sum);
        for n in 1..20 {
            let pow = (&num).pow(2 * n);
            let fact = lookup_fact(2 * n).value_of_sign(&switch);
            sum += &pow / &fact;
            switch ^= true;
            println!("pow: {:?} fact: {:?}", pow.to_string(), fact.to_string());
            println!("n: {:?} val: {:?}", n,  ((&num).pow(2 * n) / lookup_fact(2 * n).value_of_sign(&switch)).to_string());
        }
        sum
}