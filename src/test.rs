use crate::fi::{FiBin, FiLong};
use crate::fi::Parsing;

#[test]
fn create_new() {
    assert_eq!(FiBin::new(), FiBin{sign: false, value: Vec::new()});
}


#[test]
fn reg_add() {
    let num1: i8 = 7;
    let num2: i8 = 9;
    let res: i8 = num1 + num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) + FiBin::from(num2)).spruce_up()); 
}

#[test]
fn neg_add() {
    let num1: i8 = -7;
    let num2: i8 = 9;
    let res: i8 = num1 + num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) + FiBin::from(num2)).spruce_up()); 
    
    let num1: i8 = 8;
    let num2: i8 = -3;
    let res: i8 = num1 + num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) + FiBin::from(num2)).spruce_up()); 

    let num1: i8 = -13;
    let num2: i8 = -5;
    let res: i8 = num1 + num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) + FiBin::from(num2)).spruce_up()); 
}

#[test]
fn reg_sub() {
    let num1: i8 = 12;
    let num2: i8 = 6;
    let res: i8 = num1 - num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) - FiBin::from(num2)).spruce_up()); 
}

#[test]
fn neg_sub() {
    let num1: i8 = 12;
    let num2: i8 = -6;
    let res: i8 = num1 - num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) - FiBin::from(num2)).spruce_up()); 

    let num1: i8 = -8;
    let num2: i8 = 3;
    let res: i8 = num1 - num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) - FiBin::from(num2)).spruce_up()); 

    let num1: i8 = -45;
    let num2: i8 = -87;
    let res: i8 = num1 - num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) - FiBin::from(num2)).spruce_up()); 
}

#[test]
fn reg_mul() {
    let num1: i8 = 5;
    let num2: i8 = 6;
    let res: i8 = num1 * num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) * FiBin::from(num2)).spruce_up()); 
}

#[test]
fn neg_mul() {
    let num1: i8 = -2;
    let num2: i8 = 7;
    let res: i8 = num1 * num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) * FiBin::from(num2)).spruce_up()); 

    let num1: i8 = -8;
    let num2: i8 = 3;
    let res: i8 = num1 * num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) * FiBin::from(num2)).spruce_up()); 

    let num1: i8 = -6;
    let num2: i8 = -2;
    let res: i8 = num1 * num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) * FiBin::from(num2)).spruce_up()); 
}

#[test]
fn float_mul() {
    let num1: f32 = 29.8;
    let num2: f32 = 0.4;
    let res: f32 = num1 * num2;
    assert_eq!(FiBin::from(res.to_string()), (FiBin::from(num1.to_string()) * FiBin::from(num2.to_string())).spruce_up()); // TODO: remove .to_string() once float implementation is finished (for the next test as well)
}

#[test]
fn reg_div() { 
    let num1: i8 = 12;
    let num2: i8 = 3;
    let res: i8 = num1 / num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) / FiBin::from(num2)).spruce_up()); 
}

#[test]
fn float_div() {
    let num1: f32 = 9.0;
    let num2: f32 = 16.0;
    let res: f32 = num1 / num2;
    assert_eq!(FiBin::from(res.to_string()), (FiBin::from(num1.to_string()) / FiBin::from(num2.to_string())).spruce_up()); // TODO: remove .to_string() once float implementation is finished (for the next test as well)
}

#[test]
fn neg_div() {
    let num1: f32 = 4.25;
    let num2: f32 = -12.5;
    let res: f32 = num1 / num2;
    assert_eq!(FiBin::from(res.to_string()), (FiBin::from(num1.to_string()) / FiBin::from(num2.to_string())).spruce_up()); 

    let num1: f32 = -3.9;
    let num2: f32 = 0.2;
    let res: f32 = num1 / num2;
    assert_eq!(FiBin::from(res.to_string()), (FiBin::from(num1.to_string()) / FiBin::from(num2.to_string())).spruce_up()); 

    let num1: f32 = -0.1;
    let num2: f32 = -0.8;
    let res: f32 = num1 / num2;
    assert_eq!(FiBin::from(res.to_string()), (FiBin::from(num1.to_string()) / FiBin::from(num2.to_string())).spruce_up()); 
}

#[test]
fn reg_rem() {
    let num1: i8 = 23;
    let num2: i8 = 11;
    let res: i8 = num1 % num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) % FiBin::from(num2)).spruce_up()); 
}

#[test]
fn neg_rem() {
    let num1: i8 = -15;
    let num2: i8 = 4;
    let res: i8 = num1 % num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) % FiBin::from(num2)).spruce_up()); 

    let num1: i8 = 18;
    let num2: i8 = -7;
    let res: i8 = num1 % num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) % FiBin::from(num2)).spruce_up()); 

    let num1: i8 = -19;
    let num2: i8 = -5;
    let res: i8 = num1 % num2;
    assert_eq!(FiBin::from(res), (FiBin::from(num1) % FiBin::from(num2)).spruce_up()); 
}

#[test]
fn float_rem() {
    let num1: f32 = -13.9; // i initially tried 13.0 but the modulu operation lead to a floating-point error
    let num2: f32 = 4.7;
    let res: f32 = num1 % num2;
    assert_eq!(FiBin::from(res.to_string()).spruce_up(), (FiBin::from(num1.to_string()) % FiBin::from(num2.to_string())).spruce_up()); // TODO: remove .to_string() once float implementation is finished (for the next test as well)
}

#[test]
fn neg_int_conv() {
    let num: i8 = -13;
    let fi: FiBin = num.into();
    assert_eq!(fi, FiBin::from(String::from("-13")));
}

#[test]
fn eq() {
    let num1: FiBin = 1.into();
    let num2: FiBin = 1.into();
    let res: bool = num1 == num2;
    assert_eq!(res, true);
}

#[test]
fn not_eq() {
    let num1: FiBin = 1.into();
    let num2: FiBin = 7.into();
    let res: bool = num1 != num2;
    assert_eq!(res, true);
}

#[test] 
fn lt() {
    let num1: FiBin = 2.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 < num2;
    assert_eq!(res, true);

    let num1: FiBin = 4.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 < num2;
    assert_eq!(res, false);
    
    let num1: FiBin = 9.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 < num2;
    assert_eq!(res, false);

}

#[test] 
fn le() {
    let num1: FiBin = 2.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 <= num2;
    assert_eq!(res, true);
    
    let num1: FiBin = 4.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 <= num2;
    assert_eq!(res, true);
    
    let num1: FiBin = 9.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 <= num2;
    assert_eq!(res, false);

}

#[test] 
fn gt() {
    let num1: FiBin = 2.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 > num2;
    assert_eq!(res, false);

    let num1: FiBin = 4.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 > num2;
    assert_eq!(res, false);
    
    let num1: FiBin = 9.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 > num2;
    assert_eq!(res, true);

}

#[test] 
fn ge() {
    let num1: FiBin = 2.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 >= num2;
    assert_eq!(res, false);
    
    let num1: FiBin = 4.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 >= num2;
    assert_eq!(res, true);
    
    let num1: FiBin = 9.into();
    let num2: FiBin = 4.into();
    let res: bool = num1 >= num2;
    assert_eq!(res, true);

}

#[test] 
fn round_last_digit_division() {
    let f1: FiBin = FiBin{sign: false, value: vec![true, true, true]};
    let f2: FiBin = (3 as i8).into();
    assert_eq!((f1 / f2).spruce_up(), FiBin{sign: false, value: vec![false, true]});
}