use std::io;

// fn main() {
//     let mut numbers_arr = String::new();
//     dbg!(&numbers_arr);
//     io::stdin().read_line(&mut numbers_arr).unwrap();
    
//     let numbers: Vec<&str> = numbers_arr.split_whitespace().collect();
//     dbg!(&numbers);
    
//     let number_first = numbers[0].parse::<i32>().unwrap();
//     let number_second = numbers[1].parse::<i32>().unwrap();

//     println!("{}", number_first + number_second);
// }

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();

//     let number_arr:Vec<&str> = input.split_whitespace().collect();
    
//     let a: i32 = number_arr[0].trim().parse().unwrap();
//     let b: i32 = number_arr[1].trim().parse().unwrap();

//     // dbg!(number_arr);
//     // dbg!(a);
//     // dbg!(b);

//      println!("{}", a - b);
// }

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // dbg!(&input);
    let arr:Vec<&str> = input.split_whitespace().collect();
    // dbg!(&arr);

    let a: i32 = arr[0].trim().parse().unwrap();
    let b: i32 = arr[1].trim().parse().unwrap();

    println!("{}", a*b);
}