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

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();

//     // dbg!(&input);
//     let arr:Vec<&str> = input.split_whitespace().collect();
//     // dbg!(&arr);

//     let a: i32 = arr[0].trim().parse().unwrap();
//     let b: i32 = arr[1].trim().parse().unwrap();

//     println!("{}", a*b);
// }


// fn main() {
//     let mut input = String::new();

//     io::stdin().read_line(&mut input).unwrap();

//     let arr: Vec<&str> = input.split_whitespace().collect();
    
//     let a :f64 = arr[0].trim().parse().unwrap();
//     let b :f64 = arr[1].trim().parse().unwrap();

//     println!("{}", a / b);
// }

// fn main () {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();

//     let arr:Vec<&str> = input.split_whitespace().collect();

//     let a :i32 = arr[0].parse().unwrap();
//     let b :i32 = arr[1].parse().unwrap();

//     println!("{}", a + b);
//     println!("{}", a - b);
//     println!("{}", a * b);
//     println!("{}", a / b);
//     println!("{}", a % b);
// }

// fn main () {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();

//     let trimed = input.trim();

//     println!("{}??!", trimed);
// }

// fn main () {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).unwrap();

//     // trim을 하지 않으면 \n 때문에 파싱 에러가 난다!!
//     //let num_input: i32 = input.parse().unwrap();
//     let num_input: i32 = input.trim().parse().unwrap();
//     // dbg!(num_input);

//     println!("{}", num_input - 543);
// }

// fn main () {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("please input");

//     let arr: Vec<&str> = input.split_whitespace().collect();

//     let a: i32 = arr[0].parse().unwrap();
//     let b: i32 = arr[1].parse().unwrap();
//     let c: i32 = arr[2].parse().unwrap();


//     println!("{}", (a+b)%c);
//     println!("{}", ((a%c)+(b%c))%c);
//     println!("{}", (a*b)%c);
//     println!("{}", (a%c)*(b%c)%c);
// }

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("please input");

    let arr: Vec<i32> = input.split_whitespace()
        .map(|s| s.trim().parse().expect("Failed to parse input"))
        .collect();

    let (a, b, c) = (arr[0], arr[1], arr[2]);

    println!("{}", (a + b) % c);
    println!("{}", ((a % c) + (b % c)) % c);
    println!("{}", (a * b) % c);
    println!("{}", (a % c) * (b % c) % c);
}