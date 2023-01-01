
use gat_std::gat_desugar;

#[gat_desugar]
fn main() {
    for a in 0..10 {
        println!("{a}");
    }

    let mut b = vec![0, 1, 2];
    println!("{}", &b[0]);

    let first = &mut b[0];
    *first = 1;
    println!("{:?}", b);
}
