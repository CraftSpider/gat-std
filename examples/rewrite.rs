
use gat_std::gatify;

#[gatify]
fn main() {
    for a in 0..10 {
        println!("{a}");
    }

    let mut b = vec![0, 1, 2];
    println!("{}", &b[0]);

    b[1] = 2;

    let first = &mut b[0];
    *first = 1;
    println!("{:?}", b);
}
