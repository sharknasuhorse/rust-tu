fn main() {

    print_nm(10);
    print_sum(14, 14);
    let x = add_one(15);
    println!("{}", x);
    
    let f: fn(i32) -> i32 = plus_one;
    //let f = plus_one;
    let six  = f(10);
    print!("{}", six)
}

fn plus_one(y: i32) -> i32{
    y + 1
}


fn print_nm(x: i32){
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32){
    println!("x + y is: {}", x + y);
}

fn add_one(x: i32) -> i32{
    return x + 1;
}

fn diverges() -> ! {
    panic!("This function never returns!");
}


