# Fi

Most decimal numbers get represented by floating-point numbers. Floats usually create floating-point errors, which are quite annoying when you need the exact value. Therefore this type uses **fixed-point integers**. 


# Precision

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