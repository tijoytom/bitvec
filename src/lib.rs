use crate::bitvect::BitVec;

mod  bitvect;

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn basic_test() {
        let mut v = vec![];
        v.set(0);
        v.set(56);
        assert!(v.is_set(56));
    }
    #[test]
    fn set_unset() {
        let mut v = vec![];
        v.set(89);
        v.unset(89);
        assert!(!v.is_set(89));
    }
}
