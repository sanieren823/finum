use crate::fi::Fi;
use crate::fi::Parsing;

#[test]
fn create_new() {
    assert_eq!(Fi::new(), Fi{sign: false, value: Vec::new()});
}


#[test]
fn reg_add() {
    let num1: i8 = 7;
    let num2: i8 = 9;
    let res: i8 = num1 + num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) + Fi::from(num2)).spruce_up()); 
}

#[test]
fn neg_add() {
    let num1: i8 = -7;
    let num2: i8 = 9;
    let res: i8 = num1 + num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) + Fi::from(num2)).spruce_up()); 
    
    let num1: i8 = 8;
    let num2: i8 = -3;
    let res: i8 = num1 + num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) + Fi::from(num2)).spruce_up()); 

    let num1: i8 = -13;
    let num2: i8 = -5;
    let res: i8 = num1 + num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) + Fi::from(num2)).spruce_up()); 
}

#[test]
fn reg_sub() {
    let num1: i8 = 12;
    let num2: i8 = 6;
    let res: i8 = num1 - num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) - Fi::from(num2)).spruce_up()); 
}

#[test]
fn neg_sub() {
    let num1: i8 = 12;
    let num2: i8 = -6;
    let res: i8 = num1 - num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) - Fi::from(num2)).spruce_up()); 

    let num1: i8 = -8;
    let num2: i8 = 3;
    let res: i8 = num1 - num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) - Fi::from(num2)).spruce_up()); 

    let num1: i8 = -45;
    let num2: i8 = -87;
    let res: i8 = num1 - num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) - Fi::from(num2)).spruce_up()); 
}

#[test]
fn reg_mul() {
    let num1: i8 = 5;
    let num2: i8 = 6;
    let res: i8 = num1 * num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) * Fi::from(num2)).spruce_up()); 
}

#[test]
fn neg_mul() {
    let num1: i8 = -2;
    let num2: i8 = 7;
    let res: i8 = num1 * num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) * Fi::from(num2)).spruce_up()); 

    let num1: i8 = -8;
    let num2: i8 = 3;
    let res: i8 = num1 * num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) * Fi::from(num2)).spruce_up()); 

    let num1: i8 = -6;
    let num2: i8 = -2;
    let res: i8 = num1 * num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) * Fi::from(num2)).spruce_up()); 
}

#[test]
fn reg_div() { // float division
    let num1: i8 = 12;
    let num2: i8 = 3;
    let res: i8 = num1 / num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) / Fi::from(num2)).spruce_up()); 
}

#[test]
fn float_div() {
    let num1: f32 = 9.0;
    let num2: f32 = 16.0;
    let res: f32 = num1 / num2;
    assert_eq!(res.to_string().parse_fi(), (num1.to_string().parse_fi() / num2.to_string().parse_fi()).spruce_up()); 
}

#[test]
fn neg_div() {
    let num1: f32 = 4.25;
    let num2: f32 = -12.5;
    let res: f32 = num1 / num2;
    assert_eq!(res.to_string().parse_fi(), (num1.to_string().parse_fi() / num2.to_string().parse_fi()).spruce_up()); 

    let num1: f32 = -3.9;
    let num2: f32 = 0.2;
    let res: f32 = num1 / num2;
    assert_eq!(res.to_string().parse_fi(), (num1.to_string().parse_fi() / num2.to_string().parse_fi()).spruce_up()); 

    let num1: f32 = -0.1;
    let num2: f32 = -0.8;
    let res: f32 = num1 / num2;
    assert_eq!(res.to_string().parse_fi(), (num1.to_string().parse_fi() / num2.to_string().parse_fi()).spruce_up()); 
}

#[test]
fn reg_rem() {
    let num1: i8 = 23;
    let num2: i8 = 11;
    let res: i8 = num1 % num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) % Fi::from(num2)).spruce_up()); 
}

#[test]
fn neg_rem() {
    let num1: i8 = -15;
    let num2: i8 = 4;
    let res: i8 = num1 % num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) % Fi::from(num2)).spruce_up()); 

    let num1: i8 = 18;
    let num2: i8 = -7;
    let res: i8 = num1 % num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) % Fi::from(num2)).spruce_up()); 

    let num1: i8 = -19;
    let num2: i8 = -5;
    let res: i8 = num1 % num2;
    assert_eq!(res.to_string().parse_fi(), (Fi::from(num1) % Fi::from(num2)).spruce_up()); 
}

#[test]
fn neg_int_conv() {
    let num: i8 = -13;
    let fi: Fi = num.into();
    assert_eq!(fi, String::from("-13").parse_fi());
}

#[test]
fn eq() {
    let num1: Fi = 1.into();
    let num2: Fi = 1.into();
    let res: bool = num1 == num2;
    assert_eq!(res, true);
}

#[test]
fn not_eq() {
    let num1: Fi = 1.into();
    let num2: Fi = 7.into();
    let res: bool = num1 != num2;
    assert_eq!(res, true);
}

#[test] 
fn lt() {
    let num1: Fi = 2.into();
    let num2: Fi = 4.into();
    let res: bool = num1 < num2;
    assert_eq!(res, true);

    let num1: Fi = 4.into();
    let num2: Fi = 4.into();
    let res: bool = num1 < num2;
    assert_eq!(res, false);
    
    let num1: Fi = 9.into();
    let num2: Fi = 4.into();
    let res: bool = num1 < num2;
    assert_eq!(res, false);

}

#[test] 
fn le() {
    let num1: Fi = 2.into();
    let num2: Fi = 4.into();
    let res: bool = num1 <= num2;
    assert_eq!(res, true);
    
    let num1: Fi = 4.into();
    let num2: Fi = 4.into();
    let res: bool = num1 <= num2;
    assert_eq!(res, true);
    
    let num1: Fi = 9.into();
    let num2: Fi = 4.into();
    let res: bool = num1 <= num2;
    assert_eq!(res, false);

}

#[test] 
fn gt() {
    let num1: Fi = 2.into();
    let num2: Fi = 4.into();
    let res: bool = num1 > num2;
    assert_eq!(res, false);

    let num1: Fi = 4.into();
    let num2: Fi = 4.into();
    let res: bool = num1 > num2;
    assert_eq!(res, false);
    
    let num1: Fi = 9.into();
    let num2: Fi = 4.into();
    let res: bool = num1 > num2;
    assert_eq!(res, true);

}

#[test] 
fn ge() {
    let num1: Fi = 2.into();
    let num2: Fi = 4.into();
    let res: bool = num1 >= num2;
    assert_eq!(res, false);
    
    let num1: Fi = 4.into();
    let num2: Fi = 4.into();
    let res: bool = num1 >= num2;
    assert_eq!(res, true);
    
    let num1: Fi = 9.into();
    let num2: Fi = 4.into();
    let res: bool = num1 >= num2;
    assert_eq!(res, true);

}

#[test] 
fn round_last_digit_division() {
    let f1: Fi = Fi{sign: false, value: vec![true, true, true]};
    let f2: Fi = (3 as i8).into();
    assert_eq!((f1 / f2).spruce_up(), Fi{sign: false, value: vec![false, true]});
}