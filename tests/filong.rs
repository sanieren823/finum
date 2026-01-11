#[cfg(test)]
mod tests {
    use fi::fi::FiLong;
    #[test]
    fn create_new() {
        assert_eq!(FiLong::new(), FiLong{sign: false, value: Vec::new()});
    }

    #[test]
    fn gt() {
        assert_eq!(FiLong::one() > FiLong::new(), true);
        assert_eq!(FiLong::e() > FiLong::pi(), false);
        assert_eq!(FiLong::neg_eight() > FiLong::ten(), false);
    }

    #[test]
    fn lt() {
        assert_eq!(FiLong::four() < FiLong::seven(), true);
        assert_eq!(FiLong::two_pi() < FiLong::pi_div_two(), false);
        assert_eq!(FiLong::neg_six() < FiLong::one_half(), true);
    }

    #[test]
    fn ge() {
        assert_eq!(FiLong::ln2() >= FiLong::ln3(), false);
        assert_eq!(FiLong::one_quarter() >= FiLong::one_quarter(), true);
        assert_eq!(FiLong::neg_eight() >= -FiLong::million(), true);
    }

    #[test]
    fn le() {
        assert_eq!(FiLong::nine() <= FiLong::trillion(), true);
        assert_eq!(FiLong::new() <= FiLong::new(), true);
        assert_eq!(FiLong::five() <= FiLong::neg_eight(), false);
    }

    #[test]
    fn eq() {
        assert_eq!(FiLong::two() == FiLong::two(), true);
        assert_eq!(FiLong::nine() == FiLong::three(), false);
        assert_eq!(FiLong::neg_one() != FiLong::neg_one(), false);
    }

    #[test]
    fn abs() {
        assert_eq!(FiLong::two().abs(), FiLong::two());
        assert_eq!(FiLong::neg_ten().abs(), FiLong::ten());
    }

    #[test]
    fn arr_manipulation() {
        let mut one = FiLong::one();
        one.push(0);
        assert_eq!(FiLong::one(), one.spruce_up());
        one.remove(0);
        assert_eq!(one[0], 5);
        one.pop();
        assert_eq!(one, FiLong{sign: false, value: vec![one[0]]});
        one.insert(1, 13);
        assert_eq!(18, one[0] + one[1]);
    }

}