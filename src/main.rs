mod fi;
mod conversion;

fn main() {
    let fixed = fi::fi{sign: false, value: vec![true, true, false, false, true, true, true, true]};
    println!("{:?}", fixed.bin_bcd().value);
    println!("{:?}", fixed.bin_bcd().bcd_bin());
    // println!("{:?}", fi::fi1024{leading: 12, vec: vec![13, 31, 17], trailing: 123}.full_vec());
}
