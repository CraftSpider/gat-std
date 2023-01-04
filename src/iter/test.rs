use super::*;

struct LendingIter(u8, u8);

impl LendingIter {
    fn new() -> LendingIter {
        LendingIter(0, 10)
    }
}

impl Iterator for LendingIter {
    type Item<'a> = &'a mut u8;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        match self.1.checked_sub(1) {
            Some(new) => {
                self.1 = new;
                Some(&mut self.0)
            }
            None => None,
        }
    }
}

#[test]
fn iter_filter() {
    let iter = LendingIter::new();

    let res = iter
        .touch(|a| **a += 1)
        .filter(|a| **a % 2 == 0)
        .fold(0, |acc, val| {
            assert!(*val % 2 == 0);
            acc + 1
        });

    assert_eq!(res, 5);
}

#[test]
fn iter_chain() {
    let iter = LendingIter::new();

    let res = iter.chain(LendingIter::new()).fold(0, |acc, a| {
        *a += 1;
        assert_eq!(*a, (acc % 10) + 1);
        acc + 1
    });

    assert_eq!(res, 20);
}
