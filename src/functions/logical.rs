use std::cmp::Ordering;
use crate::fi::{FiBin, FiLong};
use std::time::Instant; // TODO: remove


impl PartialOrd for FiBin {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiBin {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {

        match self.sign.cmp(&other.sign) {
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => (),
        }
        let v1 = self.spruce_up();    
        let v2 = other.spruce_up();
        let len_diff: isize = v1.len() as isize - v2.len() as isize;

        let mut b = false; // fix
        if len_diff == 0 {
            let len = v1.len();
            if len == 0 {
                return Ordering::Equal;
            }
            for i in 1..=len {
                if v1.value[len - i] != v2.value[len - i] {
                    b = v1.value[len - i];
                    break;
                } else if i == len {
                    return Ordering::Equal;
                }
            }
        } else {
            b = heaviside(&len_diff);
        }
        b ^= self.sign;
        if b {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialEq for FiBin {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.sign == other.sign
    }
}

impl Eq for FiBin {}

impl PartialOrd for FiLong {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FiLong {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.sign.cmp(&other.sign) {
            Ordering::Less => return Ordering::Greater,
            Ordering::Greater => return Ordering::Less,
            Ordering::Equal => (),
        }
        let v1 = self.spruce_up(); 
        let v2 = other.spruce_up();
        let len = v1.len();
        let len_diff: isize = len as isize - v2.len() as isize;
        if len_diff == 0 {
            if len == 0 {
                return Ordering::Equal;
            }
            for i in (0..len).rev() {
                if v1.value[i] != v2.value[i] {
                    if (v1.value[i] > v2.value[i]) ^ self.sign {
                        return Ordering::Greater;
                    } else {
                        return Ordering::Less;
                    }
                } 
            }
            return Ordering::Equal;
        } else {
            if heaviside(&len_diff) ^ self.sign {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        }
    }
}

impl PartialEq for FiLong {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.sign == other.sign
    }
}

impl Eq for FiLong {}

#[inline(always)]
fn heaviside(num: &isize) -> bool {
    if *num < 0 {
        false
    } else {
        true
    }
}

