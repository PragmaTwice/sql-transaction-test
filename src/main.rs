mod permutation;

use permutation::BinaryPermutation;

fn main() {
    let mut a = BinaryPermutation::new(2, 3);

    while {
        println!("{:?}", a); 
        a.next()
    } {}

    let x = vec![1, 2];
    let y = vec![3, 4, 5];

    while {
        a.process_iter(x.iter(), y.iter(), |i| print!("{:?} ", i));
        println!();
        a.next()
    } {}
}
