// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    let calculate = basic_calc(&[2,3],&"div");
    println!("Answer: {}",calculate)  
}


fn basic_calc(nums: &[i32;2], op:&str) -> i32 {
    let mut output = 0;
    match op {
        "add" => {
            for n in nums {
                output += n;

            }
        },
        "sub" => {
            for n in nums {
                output -= n;

            }
        },
        "mul" => {
            output += 1;
            for n in nums {
                output *= n;
            }
        },
        "div" => {
            output += 1;
            for n in nums {
              return  output % n
            }
        }
        &_ => println!("nothing happened")
    }
   println!("{}",output);
    output
}








