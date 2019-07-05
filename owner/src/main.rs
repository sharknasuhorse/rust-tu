fn take(v: Vec<i32>){
    println!("take fn v[0] is :{}", v[0]);
}

fn double(x: i32) -> i32{
    x * 2
}

fn main() {
    let v = vec![1,2,3];
    //let v2 = v;
    println!("main v[0] is :{}", v[0]);
    take(v);
    //println!("main v[0] is :{}", v[0]);
    let a = 10;
    let _y = double(a);
    println!("{}", a);

}
