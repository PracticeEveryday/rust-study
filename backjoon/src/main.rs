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

// fn main() {
//     println!("{0}    {1}{0}", "\\", "/");
//     println!(" {0}  {1} {2}{0}", ")", "(", "'");
//     println!("{0}  {1}  {2}", "(", "/", ")");
//     println!(" {0}{1}{2}{2}{3}{4}", "\\", "(", "_", ")", "|",);
// }

// \    /\
//  )  ( ')
// (  /  )
//  \(__)|


// fn main() {
//     let mut input = String::new();

//     io::stdin().read_line(&mut input).expect("input 값을 확인해주세요");

//     let arr:Vec<i32> = input.split_whitespace()
//         .map(|s| s.trim().parse().expect("파싱 에러입니다.")).collect();

//     let (a,  b) = (arr[0], arr[1]);

//     if a > b {
//         println!(">");
//     } else if a == b {
//         println!("==");
//     } else {
//         println!("<");
//     }
// }

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("input 값을 확인해주세요");

//     let grade: i32 = input.trim().parse().expect("parse error!!.");
//     // dbg!(grade);
//     if grade >=90 && grade <= 100 {
//         println!("A")
//     } else if grade >=80 && grade <= 89 {
//         println!("B")
//     } else if grade >= 70 && grade <=79 {
//         println!("C")
//     } else if grade >= 60 && grade <= 69 {
//         println!("D")
//     } else {
//         println!("F")
//     }
// }

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("입력을 확인해주세요");

//     let grade: Result<i32, std::num::ParseIntError> = input.trim().parse();
    
//     match grade {
//         Ok(grade_value) => {
//             if grade_value >= 90 && grade_value <= 100 {
//                 println!("A");
//             } else if grade_value >= 80 && grade_value <= 89 {
//                 println!("B");
//             } else if grade_value >= 70 && grade_value <= 79 {
//                 println!("C");
//             } else if grade_value >= 60 && grade_value <= 69 {
//                 println!("D");
//             } else {
//                 println!("F");
//             }
//         }
//         Err(err) => {
//             eprintln!("파싱 오류: {:?}", err);
//         }
//     }
// }

// fn main () {
//     //윤년은 4의 배수 이면서 100의 배수가 아닐때 
//     // 400의 배수일 때
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("input 값을 확인해주세요");

//     let year:Result<i32, ParseIntError> = input.trim().parse();

//     match year {
//         Ok(year) => {
//             if year % 400 == 0 {
//                 println!("{}", 1);
//             } else if year % 4  == 0 && year % 100 != 0 {
//                 println!("{}", 1);
//             } else {
//                 println!("{}", 0);
//             }
//         }

//         Err(error) => {
//             eprint!("err {}", error);
//         }
//     }
// }

// fn main () {
//     let mut input = String::new();
//     for _ in 0..2 {
//         io::stdin().read_line(&mut input).expect("input 값을 확인해주세요");
//     }

//     let arr:Vec<i32> = input.trim_end().split("\n").map(|s| s.trim().parse().expect("파싱에러입니다.")).collect();

//     let (x, y) = (arr[0], arr[1]);

//     if x > 0 {
//         if y > 0 {
//             println!("1");
//         } else {
//             println!("4");
//         }
//     } else {
//         if y > 0 {
//             println!("2");
//         } else {
//             println!("3");
//         }
//     }
// }

// fn main () {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("input 값을 확인해주세요");

//     let arr: Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("파싱 에러입니다.")).collect();

//     let (mut a, b) = (arr[0], arr[1]);
//     let mut min: i32 = b - 45;

//     if min < 0 {
//         min = min + 60;
//         if a == 0 {
//             a = 23
//         } else {
//             a = a - 1
//         }
//     }

//     println!("{} {}", a, min);
// }

// fn main () {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("check input value"); 
//     let arr:Vec<i32> = input.trim().split_whitespace().map(|val| val.trim().parse().expect("check value type")).collect();

//     let (mut hour, mut minute) = (arr[0], arr[1]);

//     input.clear();
//     io::stdin().read_line(&mut input).expect("check input value"); 
//     let spent_time: i32 = input.trim().parse().expect("check input type");

//     // for _ in 0..2 {
//         // io::stdin().read_line(&mut input).expect("check input value"); 
//     // }

//     // let arr:Vec<i32> = input.trim_end().split("\n").map(|val| val.trim().parse().expect("check value type")).collect();
//     //let (mut hour, mut minute, spent_time) = (arr[0], arr[1], arr[2]);
    
//     let spend_hour = spent_time / 60;
//     let spend_minute = spent_time % 60;

//     if spend_hour > 0 {
//         hour += spend_hour
//     }
    
//     if minute + spend_minute >= 60 {
//         hour += 1;
//         minute += spend_minute - 60;
//         if minute >=60 {
//             hour += 1;
//             minute -= 60;
//         }
//     } else {
//         minute += spend_minute
//     }

//     if hour >= 24 {
//         hour -= 24
//     }

//     print!("{} {}", hour, minute);
// }

// //TODO: 한 번 더보기
// fn main () {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let vec: Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check value type")).collect();
    
//     // 3개 다 같을때
//     if vec.iter().all(|&val| val == vec[0]) {
//         println!("{}", 10000 + vec[0]*1000)
//     } else {
//         // 3개다 다를 때
//         let hashset:std::collections::HashSet<_> = vec.iter().collect();
//         if hashset.len() == vec.len() {
//             println!("{}", vec.iter().max().unwrap() * 100)
//         } else {
//             // 2개만 같을 때
//             let mut hashmap = std::collections::HashMap::new();
//             let mut same_val = 0;
//             for &val in &vec {
//                 if hashmap.get(&val).is_some() {
//                     same_val = val;
//                 } else {
//                     hashmap.insert(val, 1);
//                 }
//             }

//             println!("{}", same_val * 100 + 1000)
//         }    
//     }
// }

// fn main () {
//     let mut input = String::new();

//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let num:i32 = input.trim().parse().expect("check value type");

//     for i in 1..10 {
//         println!("{0} * {1} = {2}", num, i, num*i);
//     }
// }

// fn main() {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let mut answer = String::new();

//     let num:i32 = input.trim().parse().expect("check input type");

//     for _ in 0..num {
//         input.clear();
//         std::io::stdin().read_line(&mut input).expect("check input value");

//         let arr:Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check value type")).collect();
//         let added = arr[0] + arr[1];
//         let added_str = added.to_string();
//         // let added_char = char::from_digit(added as u32, 10).expect("Failed to convert to char");

//         answer.push_str(&added_str);
//         answer.push('\n');
//     }   
//     println!("{}", answer);
// }


// fn main() {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check the input values");

//     let num: i64 = input.trim().parse().expect("check input type");
    
//     let mut result = 0;
//     for i in 1..num+1 {
//         result += i;
//     }

//     println!("{}", result);
// }

// fn main() {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input values");
//     let total_price: i32 = input.trim().parse().expect("check value types");

//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input values");
//     let num: i32 = input.trim().parse().expect("check value types");
//     let mut result = 0;

//     for _ in 0..num {
//         input.clear();
//         std::io::stdin().read_line(&mut input).expect("check input values");

//         let arr:Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check parse types")).collect();
//         let (price, count) = (arr[0], arr[1]);
//         result += price * count;
//     }

//     if total_price == result {
//         println!("{}", "Yes");
//     } else {
//         println!("{}", "No");
//     }
// }

// fn main() {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let bytes: i32 = input.trim().parse().expect("check pased value type");

//     let mut result = String::from("int");

//     let long_count = bytes / 4;

//     for _ in 0..long_count {
//         let concat_str = "long ".to_string();
//         result = concat_str + &result;
//     }

//     println!("{}", result);
// }

// fn main () {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     // std::io::stdin().read_line(&mut input).expect("check input value");
//     let num: i32 = input.trim().parse().expect("check value type");

//     let mut result = String::new();

//     for _ in 0..num {
//       input.clear();

//       std::io::stdin().read_line(&mut input).expect("check input value");
//       let arr: Vec<i64> = input.split_whitespace().map(|val| val.trim().parse().expect("check valud type")).collect();

//       let (a, b) = (arr[0], arr[1]);
//       let added = a + b;

//       result.push_str(&added.to_string());
//       result.push('\n');
//     }

//     println!("{}", result);
// }

// fn main () {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     // std::io::stdin().read_line(&mut input).expect("check input value");
//     let num: i32 = input.trim().parse().expect("check value type");

//     let mut result = Vec::new();

//     for _ in 0..num {
//       input.clear();

//       std::io::stdin().read_line(&mut input).expect("check input value");
//       let arr: Vec<i64> = input.split_whitespace().map(|val| val.trim().parse().expect("check valud type")).collect();

//       let (a, b) = (arr[0], arr[1]);
//       let added = a + b;
//       result.push(added);
//     }

//     for (idx, value) in result.iter().enumerate()  {
//       println!("Case #{}: {}", idx + 1, value);
//     }
// }


// fn main () {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     // std::io::stdin().read_line(&mut input).expect("check input value");
//     let num: i32 = input.trim().parse().expect("check value type");

//     for i in 0..num {
//       input.clear();

//       std::io::stdin().read_line(&mut input).expect("check input value");
//       let arr: Vec<i64> = input.split_whitespace().map(|val| val.trim().parse().expect("check valud type")).collect();

//       let (a, b) = (arr[0], arr[1]);
//       println!("Case #{}: {} + {} = {}", i + 1, a, b, a + b);
//     }
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let num: usize = input.trim().parse().expect("check input value for parse");


//   for i in 1..num + 1 {
//     let star = "*".repeat(i);
//     println!("{}", star);
//   }
// }

//TODO: 별찍기 다시보기
// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("");
//   let num: usize = input.trim().parse().expect("check the value type");

//   for i in 1..num+1 {
//     let space_count = " ".repeat(num - i);
//     let star_count = "*".repeat(i);

//     println!("{}{}", space_count, star_count);
//   }
// }

// fn main() {
//   loop {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     let arr: Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check the pased type")).collect();

//     if input.trim() == "0 0"{
//       break;
//     }

//     println!("{}", arr[0] + arr[1]);
//   }
// }

// fn main() {
//   loop {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     let arr: Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check the pased type")).collect();
//     if arr.len() == 0 {
//         break;
//     }
//     println!("{}", arr[0] + arr[1]);
//   }
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check the input value");

//   let total_count: i32 = input.trim().parse().expect("check the parse value");
//   input.clear();

//   std::io::stdin().read_line(&mut input).expect("check the input value");
//   let num_arr: Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check the parsed value")).collect();

//   input.clear();
//   std::io::stdin().read_line(&mut input).expect("check the input value");
//   let check_value:i32 = input.trim().parse().expect("check input value");

//   let mut result_count = 0;

//   for i in num_arr {
//     if i == check_value {
//       result_count += 1;
//     }
//   }
//   println!("{}", result_count);
// }

// fn main() {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check the input value");

//   let arr :Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check input parse type")).collect();
//   let (_, standard) = (arr[0], arr[1]);

//   input.clear();
//   std::io::stdin().read_line(&mut input).expect("check the input value");

//   let mut result = String::new();
//   let comparison_arr :Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check input parse type")).collect();
//   for i in 0..comparison_arr.len() {
//     if comparison_arr[i] < standard {
//       let str_i = comparison_arr[i].to_string();
//         result.push_str(&format!("{} ", str_i));  
      
//     }
//   }
//   println!("{}", result);
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let num: i32 = input.trim().parse().expect("check value for parsing");
//   input.clear();
//   std::io::stdin().read_line(&mut input).expect("check input value");

  // let num_arr: Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect();
//   let max_num = num_arr.iter().max().unwrap();
//   let min_num = num_arr.iter().min().unwrap(); 

//   println!("{} {}", min_num, max_num);
// }

// fn main () {
//   // 첫째줄에 최대값, 둘째줄에 위치 출력
//   let mut vec: Vec<i32> = Vec::new();// --> heap
  
//   for _ in 0..9 {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");
    
//     vec.push(input.trim().parse().unwrap());
//   }

//   let max_num = vec.iter().max().expect("not iterator type");
//   let max_idx = vec.iter().position(|val| val == max_num ).expect("not found index");
  
//   println!("{}", max_num);
//   println!("{}", max_idx+1);
// }

// fn main () {
//   // N개의 바구니가 있다.
//   // 1~N까지 번호가 매겨져 있꼬
//   // M번 공을 넣을 것이다.
    
//   // -- condition
//   // 가장 처음 바구니에는 공이 없다.
//   // 한 번 공을 넣을 때 공 넣을 바구니 범위를 정하고, 모드 같은 번호를 넣는다.
//   // 만약 바구니에 공이 들어 있으면 들어 있는 공을 빼고 새 공을 넣는다.

//   // --input
//   // 첫 줄에 바구니 개수, 몇 번 넣을 건지 나온다 -> 5 4 // 5개, 4번 넣을 것
//   // 두번째 줄 부터 i j k 공넣는 방법이 나온다 -> 2 5 6 // 2번 바구니부터 5번 바구니까지 6번 공을 넣는다. -> 공이 들어 있으면 빼고 넣을 공을 넣어라.

//   // --result
//   // 1번 바구니부터 N번 바구니까지 들어 있는 공의 숫자를 공백을 구분해 출력해라 // 공이 없으면 0을 출력
  

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_arr: Vec<usize> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed type")).collect();
      
//   let (basket_count, input_count) = (input_arr[0], input_arr[1]);
//   // 바스킷을 처음에 0으로 채운다.
//   let mut basket = vec![0; basket_count];
  
//   for _ in 0..input_count {
//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let input_arr: Vec<i32> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed type")).collect();

//     let (start_input_basket_number, end_input_basket_number, ball_number) = (input_arr[0], input_arr[1], input_arr[2]);
    
//     for bascket_number in start_input_basket_number-1..end_input_basket_number {
//       if basket[bascket_number as usize] == 0 {
//         basket[bascket_number as usize] = ball_number
//       } else {
//         basket[bascket_number as usize] = ball_number
//       }
//     }
//   }
//   // println!("{:?}", basket);
//   let formatted_string: String = basket.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" ");
    
//   println!("{}", formatted_string);
  
// }

// fn main() {
//   // 첫 번째 줄에 N 과 M이 주어진다.
//   // N은 바구니 개수 M은 변경할 케이스 개수
//   // 2번째 줄부터 공을 교환할 방법이 주어진다.
  
//   // --condition
//   // i j는 i번 바구니와 j번 바구니를 교환

//   // -- result
//   // 들어 있는 공의 번호를 공백으로 출력

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_arr:Vec<usize> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect();
//   let (bascket_count, try_count) = (input_arr[0], input_arr[1]);
//   let mut bascket = Vec::new();

//   for i in 0..bascket_count {
//     bascket.push(i+1);
//   }
  
//   for _ in 0..try_count {
//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     let input_arr:Vec<usize> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect();
//     let (bascket_one, bascket_two) = (input_arr[0], input_arr[1]);

//     let temp = bascket[bascket_one - 1];
//     bascket[bascket_one - 1] = bascket[bascket_two - 1];
//     bascket[bascket_two - 1] = temp;
//   }

//   let mut result_str = String::new();
//   bascket.iter().for_each(|val| result_str.push_str(&format!("{} ", val)));


//   println!("{}", result_str);

//   /*
//     5 4
//     1 2
//     3 4
//     1 4
//     2 2
//     3 1 4 2 5
//    */
// }

// fn main () {
//   let mut i: usize = 1;
//   // let mut vec = vec![0; 29];
//   // vec.iter_mut().enumerate().for_each(|(i, el)| *el = i + 1);
//   let mut vec: Vec<usize> = (1..=30).collect();
//   let mut input = String::new();
//   while i < 29 {
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     let num: usize = input.trim().parse().expect("check parsed value");

//     let vec_idx = vec.iter().position(|val| val == &num).expect("not found idx");

//     vec.remove(vec_idx);

//     input.clear();
//     // i++; >> i++를 안전하지 않은 것으로 간주함.
//     i += 1;
//   }

//   // collect::<Vec<String>> => 백터로 문자열을 모으는 것!!!
//   let my_string: String = vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "); 
//   println!("{}", my_string);
// }