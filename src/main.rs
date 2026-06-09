// use std::mem::size_of_val;

#[allow(unused_variables)]
fn main() {
    let url: &str = "https://google.com";
    let domain = slice_domain(&url);

    println!("Success {}", domain)
}

fn slice_domain(url: &str) -> &str {
    let len = url.len();
    let item = &url[8..len];
    item
}



