


fn main() {
  let n:i32 = 5;


  let big_n:i32 = 
        if n < 10 && n > -10 {
          println!("increase tenfold");
          10 * n

        } else {
          println!("halve the number");
          n/2
        };


  println!("{} -> {}", n , big_n)        
}