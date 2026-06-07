// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    let largest = find_largest_num(&[6,1,330,4,9]);
    println!("Answer: {}",largest)  
}


fn find_largest_num(arr: &[u32;5]) -> u32{
    let mut num = 0;
    for n in arr {
        if num < *n {
            num = *n;
        }
    }
    num
}








