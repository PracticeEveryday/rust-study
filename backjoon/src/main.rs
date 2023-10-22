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

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("please input");

//     let arr: Vec<i32> = input.split_whitespace()
//         .map(|s| s.trim().parse().expect("Failed to parse input"))
//         .collect();

//     let (a, b, c) = (arr[0], arr[1], arr[2]);

//     println!("{}", (a + b) % c);
//     println!("{}", ((a % c) + (b % c)) % c);
//     println!("{}", (a * b) % c);
//     println!("{}", (a % c) * (b % c) % c);
// }


// fn main() {
//     let mut input = String::new();
    

//     for _ in 0..2 {
//         io::stdin().read_line(&mut input).expect("pleas input");
//     }

//     let arr :Vec<u32> = input.split_whitespace()
//         .map(|s| s.trim().parse().expect("파싱에 문제가 있습니다."))
//         .collect();

//     let num_1 = arr[0];
//     let num_2 = arr[1];

//     let num_2_str = num_2.to_string();
//     let num_2_str_arr: Vec<char> = num_2_str.chars().collect();
//     let num_2_num_arr:Vec<u32> = num_2_str_arr.iter().map(|s| s.to_digit(10).expect("숫자로 파싱합니다.")).collect();

//     let reversed_result: Vec<u32> = num_2_num_arr.into_iter().rev().collect(); // 벡터를 뒤집음

//     reversed_result.iter().for_each(|s| println!("{}", s * num_1));
//     println!("{}", num_1 * num_2);
// }


// fn main () {
//     let mut input = String::new();

//     io::stdin().read_line(&mut input).expect("인풋에 실패하였습니다.");

//     let arr: Vec<u64> = input.split_whitespace().map(|s| s.trim().parse().expect("파싱에러")).collect();
//     let mut result:u64 = 0;
//     for i in arr {
//         result += i;
//     }

//     println!("{}", result);
// }

fn main() {
    println!("{0}    {1}{0}", "\\", "/");
    println!(" {0}  {1} {2}{0}", ")", "(", "'");
    println!("{0}  {1}  {2}", "(", "/", ")");
    println!(" {0}{1}{2}{2}{3}{4}", "\\", "(", "_", ")", "|",);
}

// \    /\
//  )  ( ')
// (  /  )
//  \(__)|