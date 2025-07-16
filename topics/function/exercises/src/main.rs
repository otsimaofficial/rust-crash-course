#![allow(unused)]

pub fn mul(x:u32, y:u32) -> u32 {
    return x*y;
}


pub fn div(x:u32, y:u32) -> u32 {
    return x/y;
}

fn main() {
    let x = 8;
    let y = 4;
    let a: u32 = mul(x,y);
    let b = div(x,y);

    println!("{}", a);
    println!("{}", b);
}