// use std::mem::size_of_val;
#[allow(unused_variables)]


struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn main(){
    let v:Point = Point(0,127,255);
    check_color(v);

    println!("Success");
}

fn check_color(p:Point){
    let Point(x,_y,_z) = p;

    assert_eq!(x,0);
    assert_eq!(p.1,127);
    assert_eq!(p.2,255);
}




