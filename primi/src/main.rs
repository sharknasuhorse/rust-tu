fn main() {
    let x = true;
    println!("{}", x);

    let y = '💕';
    println!("{}", y);

    let a = [1, 2, 3, 4, 5, 6];
    let b = [4, 5, 6];
    
    println!("a is: {}", a[0]);
    println!("b is: {}", b[2]);

    let complete = &a[1..];
    println!("{}", complete[0]);
    println!("");

    let c: (i32, &str) = (10, "test");
    println!("{}", c.1)
    
}




