#[cfg(test)]
mod tests {
    use fi::fi::FiLong;
    use fi::operations::bitwise::*;
    #[test]
    fn and() {
        assert_eq!((FiLong{sign: false, value: vec![40]} & &FiLong::new()).spruce_up(), FiLong::new());
        assert_eq!(&FiLong{sign: false, value: vec![31]} & FiLong{sign: false, value: vec![17]}, FiLong{sign: false, value: vec![17]});
        assert_eq!(&FiLong{sign: false, value: vec![13]} & &FiLong{sign: false, value: vec![21]}, FiLong{sign: false, value: vec![5]});
        assert_eq!((FiLong::three() & FiLong::neg_six()) < FiLong::new(), false);
    }

    #[test]
    fn or() {
        assert_eq!((FiLong{sign: false, value: vec![15]} | &FiLong::new()).spruce_up(), FiLong{sign: false, value: vec![15]});
        assert_eq!(&FiLong{sign: false, value: vec![63]} | FiLong{sign: false, value: vec![54]}, FiLong{sign: false, value: vec![63]});
        assert_eq!(&FiLong{sign: false, value: vec![24]} | &FiLong{sign: false, value: vec![3]}, FiLong{sign: false, value: vec![27]});
        assert_eq!((FiLong::five() | FiLong::neg_eight()) < FiLong::new(), true);
    }

    #[test]
    fn xor() {
        assert_eq!((FiLong{sign: false, value: vec![11]} ^ &FiLong::new()).spruce_up(), FiLong{sign: false, value: vec![11]});
        assert_eq!(&FiLong{sign: false, value: vec![127]} ^ FiLong{sign: false, value: vec![89]}, FiLong{sign: false, value: vec![38]});
        assert_eq!(&FiLong{sign: false, value: vec![25]} ^ &FiLong{sign: false, value: vec![3]}, FiLong{sign: false, value: vec![26]});
        assert_eq!((FiLong::five() ^ FiLong::neg_eight()) < FiLong::new(), true);
    }

    #[test]
    fn shr() {
        assert_eq!(FiLong::four() >> 2, FiLong::one());
        assert_eq!((FiLong{sign: false, value: vec![23]} >> 3), FiLong{sign: false, value: vec![2]});
    }

    #[test]
    fn shl() {
        assert_eq!(FiLong::one_eighth() << 4, FiLong::two());
        assert_eq!((FiLong{sign: false, value: vec![14]} << 1), FiLong{sign: false, value: vec![28]});
    }

}