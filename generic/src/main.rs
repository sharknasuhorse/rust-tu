fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    };

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    };
    largest
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    };
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let num_result = largest_i32(&number_list);
    println!("{}", num_result);

    let char_list = vec!['d', 'e', 'q', 'x'];
    let char_result = largest_char(&char_list);
    println!("{}", char_result);

    let gene_result = largest(&char_list);
    println!{"{}", gene_result};
}
