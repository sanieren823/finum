#[cfg(test)]
mod tests {
    use fi::fi::FiLong;
    use fi::operations::arithm::*;
    #[test]
    fn add() {
        assert_eq!(FiLong::four() + &FiLong::two(), FiLong::six());
        assert_eq!(&FiLong::from("3.13") + FiLong::from("3.87"), FiLong::seven());
        assert_eq!(&FiLong::from("-5.7") + &FiLong::from("-4.3"), FiLong::neg_ten());
        assert_eq!(FiLong::one() + FiLong::neg_five(), FiLong::neg_four());
    }

    #[test]
    fn sub() {
        assert_eq!(FiLong::six() - &FiLong::four(), FiLong::two());
        assert_eq!(&FiLong::from("56.92") - FiLong::from("82.42"), FiLong::from("-25.5"));
        assert_eq!(&FiLong::from("-3.4") - &FiLong::from("-8.6"), FiLong::from("5.2"));
        assert_eq!(FiLong::five() - FiLong::neg_three(), FiLong::eight());
    }

    #[test]
    fn mul() {
        assert_eq!(FiLong::three() * &FiLong::two(), FiLong::six());
        assert_eq!(&FiLong::from("2.5") * FiLong::from("1.5"), FiLong::from("3.75"));
        assert_eq!(&FiLong::from("12") * &FiLong::from("-4"), FiLong::from("-48"));
        assert_eq!(FiLong::neg_one() * FiLong::neg_five(), FiLong::five());
    }

    #[test]
    fn div() {
        assert_eq!(FiLong::four() / &FiLong::two(), FiLong::two());
        assert_eq!(&FiLong::from("6.25") / FiLong::from("2.5"), FiLong::from("2.5"));
        assert_eq!(&FiLong::from("-12") / &FiLong::from("-0.5"), FiLong::from("24"));
        assert_eq!(FiLong::one() / FiLong::neg_five(), FiLong::from("-0.2"));
    }

    #[test]
    fn rem() {
        assert_eq!(FiLong::four() % &FiLong::eight(), FiLong::four());
        assert_eq!(&FiLong::from("4.5") % FiLong::from("0.5"), FiLong::new());
        assert_eq!(&FiLong::from("-5.7") % &FiLong::from("-4.3"), FiLong::from("-1.4"));
        assert_eq!(FiLong::from("-7.8") % FiLong::from("4.2"), FiLong::from("-3.6"));
    }

    #[test]
    fn floor() {
        assert_eq!(FiLong::two() == FiLong::two(), true);
        assert_eq!(FiLong::nine() == FiLong::three(), false);
        assert_eq!(FiLong::neg_one() != FiLong::neg_one(), false);
    }

    #[test]
    fn ceil() {
        assert_eq!(FiLong::e().ceil(), FiLong::three());
        assert_eq!(FiLong::nine().ceil(), FiLong::nine());
    }

    #[test]
    fn round() {
        assert_eq!(FiLong::ln2().round(), FiLong::one());
        assert_eq!(FiLong::seven().round(), FiLong::seven());
        assert_eq!(FiLong::half_e().round(), FiLong::one());
    }

    #[test]
    fn round_n() {
        assert_eq!(FiLong::e().round_n(1), FiLong::from("2.7"));
        assert_eq!(FiLong::pi().round_n(4), FiLong::from("3.1416"));
        assert_eq!(FiLong::ln5().round_n(0), FiLong::ln5().round());
        assert_eq!(FiLong::ln10().round_n(20), FiLong::ln10());
    }

}