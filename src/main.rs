#[allow(unused_variables)]
fn main() {

    let mut sum:i32 = 0;

    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    for c in 'A'..='z' {
        println!("{}", c);
    }
}

