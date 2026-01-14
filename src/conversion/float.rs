use crate::fi::{FiBin, FiLong};
use crate::errors::FiError;
use crate::errors::FiErrorKind;
use std::any::TypeId;
use std::mem::size_of;
use std::process::Output;
use crate::operations::arithm::Floor;
use crate::operations::math::Pow;




// to float (bin)
// 1. determine log2(x).floor()
// 2. exponent --> log2(x) + bias
// 3. fraction --> x / log2(x) * 2
// 4. significand --> fraction - 1



// fraction to bits algorithm psuedo-rust-code
//  let mut fraction = fraction - 1;
//  for i in 0..fN::MANTISSA_DIGITS {
//      fraction *= 2;
//      if fraction >= 1 {
//          bits |= (1 << i);
//          fraction -= 1;
//      }
//  }
trait F64 {}

impl F64 for f64 {}

trait F32 {}

impl F32 for f64 {}


trait Parse<Type> {
    fn parse<Typ>(&self) -> Result<Type, FiError>;
}

impl Parse<f32> for FiLong {
    fn parse<F32>(&self) -> Result<f32, FiError> {
        let sign : u32= self.sign as u32;
        let log = self.log2().floor();
        const NUM_BITS: u32 = 32;
        const MAX: i32 = 127; // fomula 2^(32 - mantissa_digits - 1) - 1
        const MIN: i32 = -126; // fomula -(2^(32 - mantissa_digits - 1) - 2); why -2? the definition of IEEE 754 states that if all bits are 1 that the value must be either NaN or +/- infinity
        let mut log: i32 = match log.parse::<i32>() {
            Err(e) => return Err(e),
            Ok(val) => val,
        };
        if log < 0 {
            log -= 1;
            if log < MIN {
                return Ok(0.0); // in this case the number is two small to be represented by a float other than 0.0
            }
        } else if log > MAX {
            return Err(FiError::new(FiErrorKind::NumberTooLarge, "The number you provided exceeds the limit of the type f32"));
        }
        let exponent: u32 = (2i32.pow(NUM_BITS - f32::MANTISSA_DIGITS - 1) - 1 + log) as u32;
        let mut fraction = if log == 0 { // div zero if log2().floor() == 0
            self - FiLong::one()
        } else {
            (self / (FiLong::two().pow(log))) - FiLong::one()
        };
        let mut bits: u32 = 0;
        let size_significand = f32::MANTISSA_DIGITS - 1;
        let mut error = FiLong::one_half() >> size_significand as usize;
        if &error + &fraction >= FiLong::one() { // if the best representation is a number of a different exponent the fraction is 0
            return Ok(f32::from_bits((sign << 31u32) | ((exponent + 1) << (f32::MANTISSA_DIGITS - 1)) | bits));
        }
        for i in (0..size_significand).rev() { // it's important to understand that this algorithm will return a result regarless of whether it can be represented accurately
            fraction *= 2;
            error <<= 1;
            if fraction >= FiLong::one() {
                bits += 1 << i;
                fraction -= FiLong::one();
            } else if &fraction + &error >= FiLong::one() { // the error is necessary since the float has a fixed-size significand; in some calculations the closest floating-point representation is a larger number than the actual number
                bits += 1 << i;
                break;
            }

        }   
        let int = (sign << 31u32) | (exponent << (f32::MANTISSA_DIGITS - 1)) | bits;
        Ok(f32::from_bits(int))
    }
}

impl Parse<f64> for FiLong {
    fn parse<F64>(&self) -> Result<f64, FiError> {
        let sign : u64= self.sign as u64;
        let log: FiLong = self.log2().floor();
        const NUM_BITS: u32 = 64;
        const MAX: i64 = 1023; // fomula 2^(32 - mantissa_digits - 1) - 1
        const MIN: i64 = -1022; // fomula -(2^(32 - mantissa_digits - 1) - 2); why -2? the definition of IEEE 754 states that if all bits are 1 that the value must be either NaN or +/- infinity
        let mut log: i64 = match log.parse::<i64>() {
            Err(e) => return Err(e),
            Ok(val) => val,
        };
        if log < 0 {
            log -= 1;
            if log < MIN {
                return Ok(0f64); // in this case the number is two small to be represented by a float other than 0.0
            }
        } else if log > MAX {
            return Err(FiError::new(FiErrorKind::NumberTooLarge, "The number you provided exceeds the limit of the type f64"));
        }
        let exponent: u64 = (2i64.pow(NUM_BITS - f64::MANTISSA_DIGITS - 1) - 1 + log) as u64;
        let mut fraction = if log == 0 { // div zero if log2().floor() == 0
            self - FiLong::one()
        } else {
            (self / (FiLong::two().pow(log))) - FiLong::one()
        };
        let mut bits: u64 = 0;
        let size_significand = f64::MANTISSA_DIGITS - 1;
        let mut error = FiLong::one_half() >> size_significand as usize;
        if &error + &fraction >= FiLong::one() { // if the best representation is a number of a different exponent the fraction is 0
            return Ok(f64::from_bits((sign << 63u64) | ((exponent + 1) << (f64::MANTISSA_DIGITS - 1)) | bits));
        }
        for i in (0..size_significand).rev() { // it's important to understand that this algorithm will return a result regarless of whether it can be represented accurately
            fraction *= 2;
            error <<= 1;
            if fraction >= FiLong::one() {
                bits += 1 << i;
                fraction -= FiLong::one();
            } else if &fraction + &error >= FiLong::one() { // the error is necessary since the float has a fixed-size significand; in some calculations the closest floating-point representation is a larger number than the actual number
                bits += 1 << i;
                break;
            }

        }   
        let int = (sign << 63u64) | (exponent << (f64::MANTISSA_DIGITS - 1)) | bits;
        Ok(f64::from_bits(int))
    }
}