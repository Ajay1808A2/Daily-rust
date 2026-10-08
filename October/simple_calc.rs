fn add(a: i32, b: i32) -> i32 {
    let sum: i32 = a + b;
    sum
}

fn sub(a: i32, b: i32) -> i32 {
    let sub: i32 = a - b;
    sub
}

fn mul(a: i32, b: i32) -> i32 {
    let mul: i32 = a * b;
    mul
}

fn div(a: i32, b: i32) -> i32 {
    let div: i32 = a / b;
    div
}

fn main(){
    let sum: i32 = add(5, 10);
    println!("{}", sum);
}
