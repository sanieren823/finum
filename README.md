# finum

This crate aims to solve precision problems for most cases. Numbers are represented by **fixed-point integers** of variable length.
 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
fi = "0.1.0"
```

## Precision

The fi crate has a precision of **20 decimal digits**. Any number with more than 20 decimal digits will automatically be rounded to 20 decimal digits. The number $2$ is actually $2.000... * 10^{20}$; that way every number is an integer disguised as a decimal numeral.

## Operations
This Table shows how precise each Operation is:

|  Operation              |Precision                        
|----------------|------------------------------------------------------------------------------|
|Addition                                 | 20 digits                                           |
|Subtraction                              | 20 digits                                           |
|Division                                 | 20 digits                                           |
|Logarithm                                | 20 digits                                           |
|Exponentiation (exponent is an integer)  | 20 digits                                           |
|Exponentiation (all cases)               | ~16-17 digits                                       |
|Nth Root                                 | ~16-17 digits                                       |
|Factorial & Termial                      | ~15 digits                                          |
|Trigonometry                             | 20 digits                                           |