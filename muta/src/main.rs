fn main() {
    let mut x = 5;
    let y = &mut x;
    mut y = y + 10;
    println!("{}", y)
}
