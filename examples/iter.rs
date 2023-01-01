
use gat_std::iter::Iterator;

struct Custom(i32, i32);

impl Custom {
    fn new() -> Custom {
        Custom(10, 0)
    }
}

impl Iterator for Custom {
    type Item<'a> = &'a mut i32
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if self.0 > 0 {
            self.0 -= 1;
            Some(&mut self.1)
        } else {
            None
        }
    }
}

fn main() {
    let iter: Custom = Custom::new();

    iter.touch(|val| { **val += 1; })
        .for_each(|val| {
            println!("{}", *val)
        });
}
