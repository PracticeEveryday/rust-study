// fn main () {
//   // N * M 행렬의 합을 구하라

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_arr = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect::<Vec<usize>>();
//   let (procession_n, _) = (input_arr[0], input_arr[1]);

//   let mut procession_one: Vec<Vec<usize>> = Vec::new();
//   let mut procession_two: Vec<Vec<usize>> = Vec::new();

//   for i in 0..procession_n * 2 {
//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let procession_arr = input.split_whitespace().map(|val| val.trim().parse().expect("check puarsed value")).collect::<Vec<usize>>();
//     if i < procession_n {
//       procession_one.push(procession_arr)
//     } else {
//       procession_two.push(procession_arr)
//     }
//   }
  
//   let mut result_procession: Vec<Vec<usize>> = Vec::new();
//   for (key, vec) in procession_one.iter().enumerate() {
//     let mut factor: Vec<usize> = Vec::new();
//     for (size, _) in vec.iter().enumerate() {
//       factor.push(procession_one[key][size] + procession_two[key][size]);
//     }

//     result_procession.push(factor);
//   }

//       // 결과를 원하는 형식의 문자열로 변환
//   let result_str: String = result_procession.iter()
//       .map(|row| row.iter()
//           .map(|&value| value.to_string())
//           .collect::<Vec<String>>()
//           .join(" "))
//       .collect::<Vec<String>>()
//       .join("\n");

//     println!("{}", result_str);
// }

// fn main () {
//   // 9 X 9 행렬이 주어질 때 최대값과 위치를 구하라!
//   let mut procession_arr:Vec<Vec<usize>> = Vec::new();


//   for _ in 0..9 {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");
    
//     // 9x9 행렬 생성
//     let input_arr: Vec<usize> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect();
//     procession_arr.push(input_arr);

//     input.clear();
//   }

//   let mut max_val = 0;
//   let mut max_val_coordinate = String::new();

//   for (key, vec) in procession_arr.iter().enumerate() {
//     for (key_depth_two, _) in vec.iter().enumerate() {
//       if max_val <= procession_arr[key][key_depth_two] {
//         max_val = procession_arr[key][key_depth_two];
//         max_val_coordinate.clear();
//         max_val_coordinate.push_str(&format!("{} {}", key + 1, key_depth_two + 1));
//       }
//     }
//   }

//   println!("{}", max_val);
//   println!("{}", max_val_coordinate);

//   /*
//   3 23 85 34 17 74 25 52 65
//   10 7 39 42 88 52 14 72 63
//   87 42 18 78 53 45 18 84 53
//   34 28 64 85 12 16 75 36 55
//   21 77 45 35 28 75 90 76 1
//   25 87 65 15 28 11 37 28 74
//   65 27 75 41 7 89 78 64 39
//   47 47 70 45 23 65 3 41 44
//   87 13 82 38 31 12 29 29 90

//   90
//   5 7  
//    */
// }

// fn main () {
//   // 세로 읽기
//   // 빈칸은 지나치고 읽으라!
//   let mut input = String::new();
//   let mut vec_hashmap: std::collections::HashMap<usize, String> = std::collections::HashMap::new();

//   for _ in 0..5 {
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     let mut input_str = input.trim().to_string();
    
//     while input_str.len() == 6 {
//       input_str.push('*')
//     }

//     for (key, c) in input_str.chars().enumerate() {
//       // 가변 참조로 얻기..
//       if let Some(count) = vec_hashmap.get_mut(&key) {
//         // hashmap이 있으면 
//         count.push(c);
//       } else {
//         // 없으면
//         vec_hashmap.insert(key, String::from(c));
//       }
//     }
//     input.clear();
//   }
//   let mut result_str = String::new();

//   // dbg!(&vec_hashmap);
//   for key in 0..20 {
//       if let Some(value) = vec_hashmap.get(&key) {
//           result_str.push_str(value);
//       }
//   }

//   println!("{}", result_str.replace("*", ""));
//   /*
//   AABCDD
//   afzz
//   09121
//   a8EWg6
//   P5h3kx
  
//   Aa0aPAf985Bz1EhCz2W3D1gkD6x
//    */ 
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_count: usize = input.trim().parse().expect("check parsed value");

//   let mut word_vec: Vec<String> = Vec::new();

  
//   for _ in 0..input_count {
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let input_str = input.trim().to_string();
//     word_vec.push(input_str);

//     input.clear();
//   }

//   word_vec.sort_by(|a, b| {
//     let len_cmp = a.len().cmp(&b.len());
//     if len_cmp == std::cmp::Ordering::Equal {
//         // 길이가 같을 때, 문자열 내용을 오름차순으로 비교
//         a.cmp(b)
//     } else {
//         len_cmp
//     }
//   });

//   let result_str = word_vec.join("\n");
//   println!("{}", result_str);
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let try_count: i64 = input.trim().parse().expect("check parsed value");
//   let mut num_vec: Vec<i64> = Vec::new();
//   for _ in 0..try_count {
//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let input_num: i64 = input.trim().parse().expect("check parsed value");
//     num_vec.push(input_num);
//   }

//   num_vec.sort();

//   let result_str: String = num_vec
//     .iter()
//     .map(|num| num.to_string())
//     .collect::<Vec<String>>()
//     .join("\n");

//   println!("{}", result_str);
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   input.clear();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let mut input_vec: Vec<usize> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed input type")).collect();
//   input_vec.sort();

//   let mut result_num = 0;  
//   for (idx, value) in input_vec.iter().enumerate(){
//     result_num = result_num + value * (input_vec.len() - idx);
//   }

//   println!("{}", result_num);
//   /*
//   5
//   3 1 4 3 2

//   32
//  */
// }


// fn main () {
//   // 더하기 사이클
//   // 2자리이면 각 자리를 더한다.
  
//   // --condition
//   // 주어진 수가 한자리면 0을 붙인다.
//   // 2자리이면 각 자리를 더한다. -> 더한 수가 한자리면 앞의 숫자의 오른쪽 숫자를 더한다.

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input values");

  
//   let mut input_num: usize = input.trim().parse().expect("check parsed value");
//   // 처음 값 캐시
//   let cached_input = input_num; 

//   if cached_input == 1 {
//     println!("{}", 60);
//     return;
//   } 

//   if cached_input == 0 {
//     println!("{}", 1); 
//     return;
//   }

//   // 더하기 횟수
//   let mut add_count: usize = 0;

//   // 더한 값
//   let mut result_num:usize = 0;

//   let first_num = input_num.to_string().chars().nth(0).unwrap().to_owned().to_digit(10).unwrap() as usize;
//   let secone_num = input_num.to_string().chars().nth(1).unwrap().to_owned().to_digit(10).unwrap() as usize;

//   if input_num < 10 {
//     // result_num에 input_num + input_num을 준다.
//     result_num = input_num * 2;
//   } else {
//     // result_num에는 숫자의 첫번째 자리수를 더해준다.
//     result_num = first_num + secone_num;
//   }
//   // 10보다 작으면 count 에 1을 더하고
//   add_count = add_count + 1;
//   // input_num은 자동적으로 result_num으로 바뀐다.
//   input_num = result_num;

//   while cached_input != result_num {
//     // 더하기 횟수 증가
//     add_count = add_count + 1;    

//     if result_num > 10 {
//       // 각 자리 수 합 + 이전 값의 일의 자리수 붙여주기
//       let 십의자리수 = result_num.to_string().chars().nth(0).unwrap().to_owned().to_digit(10).unwrap() as usize;
//       let 일의자리수 = result_num.to_string().chars().nth(1).unwrap().to_owned().to_digit(10).unwrap() as usize;
//       let 새로운합 = 일의자리수 + 십의자리수;
//       if 새로운합 > 10 {
//         let 새로운합의일의자리수 = 새로운합.to_string().chars().nth(1).unwrap().to_owned().to_digit(10).unwrap() as usize;
//         result_num = 일의자리수 * 10 + 새로운합의일의자리수
//       } else {
//         result_num = 일의자리수 * 10 + 새로운합
//       }
//     } else if result_num == 10 {
//       // let 일의자리수 = result_num.to_string().chars().nth(1).unwrap().to_owned().to_digit(10).unwrap() as usize;
//       result_num = secone_num * 10;
//     } else {
//       // 10 보다 작다.
//       result_num = secone_num * 10 + result_num;
//     }
    
//   }

//   println!("{}", add_count - 1) 
// }

// fn main () {
//   // 55 -> 50 -> 05 -> 55
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let cycle_num: usize = input.trim().parse().expect("check input type");
//   let mut cycled_num = cycle_add(cycle_num);
//   let mut count = 1;
//   while cycle_num != cycled_num {
//       cycled_num = cycle_add(cycled_num);
//       count = count + 1;
//   }
//   println!("{}", count);
// }

// fn cycle_add(num: usize) -> usize {
//     let mut result = Vec::new();
//     let mut n = num;
//     if num == 0 {
//       return 0
//     }

//     while n > 0 {
//         result.push(n % 10);
//         n /= 10;
//     }

//     result.reverse();
    
//     if result.len() == 1 {
//       return num * 10 + num
//     } else {
//       return result[1] * 10 + (result[0] + result[1]) % 10
//     }
// }

// fn main () {
//   println!("{}", "강한친구 대한육군");
//   println!("{}", "강한친구 대한육군");
// }

// fn main () {
//   // background
//   // 요세푸스 순열 구하기
//   // 순서대로 K 번째 사람을 제거 -> 남은 사람들 끼리에서 원을 이어나간다! 모두 제거 될 때까지 제거 되는 순서를 출력해라


//   // --condition
//   // N K -> N: 사람 명수 K: 제거되는 번째 사람

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_arr: Vec<usize> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect();
//   let (peaple_num, mut die_num) = (input_arr[0], input_arr[1]);
//   die_num = die_num - 1;
//   let cached_die_num = die_num;
//   let mut sequence_vec: Vec<usize> = (1..=peaple_num).collect();
//   let mut result_vec :Vec<usize> = Vec::new();

//   while sequence_vec.len() != 0 {
//     if let Some(&peaple) = sequence_vec.get(die_num) {
//       // 죽은 사람
//       result_vec.push(peaple);
//       sequence_vec.remove(die_num);
//       die_num = die_num + cached_die_num;
//       // 다음에 죽을 사람 특정하기
//       while die_num >= sequence_vec.len() {
//       if sequence_vec.len() == 0 {
//         break;
//       }
//         die_num = die_num - sequence_vec.len();
//       }
//     }  
//   }
  
//   let result_str = result_vec.iter().map(|val| val.to_string()).collect::<Vec<String>>().join(", ");

//   println!("<{}>", result_str);

//   /*
//     7 3
//     <3, 6, 2, 7, 5, 1, 4>
//    */
// }

// fn main () {
//   // 주어진 N개 중 소수의 개수를 찾기
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   input.clear();
//   std::io::stdin().read_line(&mut input).expect("check input value");
//   let num_vec: Vec<u32> = input.split_ascii_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect();

//   let mut count = 0;

//   num_vec.iter().for_each(|&val| {
//     let is_decimal = is_prime(val);
//     if is_decimal {
//       count = count + 1;
//     }
//   });

//   println!("{}", count);
// }


// fn is_prime(n: u32) -> bool {
//     if n <= 1 {
//         return false; // 1과 0은 소수가 아닙니다.
//     }

//     if n <= 3 {
//         return true; // 2와 3은 소수입니다.
//     }

//     if n % 2 == 0 || n % 3 == 0 {
//         return false; // 2 또는 3으로 나누어지면 소수가 아닙니다.
//     }

//     let mut i = 5;
//     while i * i <= n {
//         if n % i == 0 || n % (i + 2) == 0 {
//             return false; // 5 이상의 숫자 중에서 소수가 아닌 것을 찾습니다.
//         }
//         i += 6;
//     }

//     true
// }


// fn main () {
//   let mut input = String::new();
//   for _ in 0..3 {
//     std::io::stdin().read_line(&mut input).expect("check input value");
//   }

//   let input_vec: Vec<usize> = input.trim_end().split("\n").map(|val| val.trim().parse().expect("check parsed value")).collect();
//   let multipled = input_vec[0] * input_vec[1] * input_vec[2];
//   let mut check_idx = vec![0;10];

//   multipled.to_string().chars().for_each(|c| {
//     let num = c.to_digit(10).unwrap() as usize;
//     check_idx[num] = check_idx[num] + 1;
//   });

//   let result_str = check_idx.iter().map(|val| val.to_string()).collect::<Vec<String>>().join("\n");

//   println!("{}", result_str);
// }

// fn main () {
//   // n의 팩토리얼을 구하라
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let mut num: usize = input.trim().parse().expect("check parsed value");
//   if num == 0 {
//     println!("{}", 1);  
//     return;
//   }

//   if num == 1 {
//     println!("{}", 1);  
//     return;
//   }


//   let mut result_num: usize = num;

//   while num != 1 {
//       result_num = result_num * (num -1);
//       num = num - 1;
//   } 

//   println!("{}", result_num);
// }

// fn main () {
//   // 1 / 2 ~ 7 / 8 ~ 19 / 20 ~ 37/ 38 ~
//   //      6        12       18      

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_num: usize = input.trim().parse().expect("check parsed value");

//   // 몇번 가야 하는 지
//   let mut count= 1;
//   // 바퀴 수
//   let mut cycle_count = 1;

//   //입력값보다 cycle_count 보다 커지면 안 된다
//   while input_num > cycle_count {
//     // i에다가 6 * i를해서 바퀴수를 센다
//     cycle_count = cycle_count + (6 * count);
//     count = count + 1;
//   }

//   println!("{}", count);
// }

// fn main () {
//   print!("|\\_/|\n");
//   print!("|q p|   /}}\n");
//   print!("( 0 )\"\"\"\\\n");
//   print!("|\"^\"`    |\n");
//   print!("||_/=\\\\__|\n");
  


//   /*
//   |\_/|
//   |q p|   /}
//   ( 0 )"""\
//   |"^"`    |
//   ||_/=\\__|
//    */
// }


// fn main() {
//   let address: *const () = std::ptr::null(); // ptr::null()은 메모리 주소 0을 나타냅니다.
//   println!("메모리 시작 주소: {:?}", address);

//   let null_varaible: *const () = std::ptr::null();
//   dbg!(null_varaible);
// }






// use std::process::Command;

// fn main() {
//   let ptr_1: *const i32 = std::ptr::null();
//   dbg!(ptr_1);
//   let output = Command::new("rustc")
//       .arg("--version")
//       .output().unwrap_or_else(|e| {
//           panic!("failed to execute process: {}", e)
//   });

//   if output.status.success() {
//       let s = String::from_utf8_lossy(&output.stdout);
//       let ptr_2: *const i32 = std::ptr::null();
//       dbg!(ptr_2);
//       dbg!(ptr_1 == ptr_2); // true

//       print!("rustc succeeded and stdout was:\n{}", s);
//   } else {
//       let s = String::from_utf8_lossy(&output.stderr);

//       print!("rustc failed and stderr was:\n{}", s);
//   }
// }


// fn main() {
//     // Define an array of integers
//     let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

//     // Use the std::mem::size_of function to get the size of the array
//     let size = std::mem::size_of_val(&arr);
//     arr[1] = 2147483647;

//     dbg!(arr);
//     println!("Array size: {} bytes", size);
//     println!("{:p}", &arr);
//     println!("{:p}", &arr[1])
// }

// fn main() {
//     let 이름 = "kim";
//     let 두번째_이름 = "kim";
//     let 이름_복사 = 이름;

//     let 힙에있는_이름 = String::from("kim");
//     let 두번째_힙에있는_이름 = String::from("kim");

    

//     println!("kim pointer {:p}", "kim");     // 0x104341d29
//     println!("kim pointer {:p}", 이름);      // 0x104341d29
//     println!("kim pointer {:p}", 두번째_이름);    // 0x104341d29
//     println!("kim pointer {:p}\n", 이름_복사);  // 0x104341d29

//     test(이름);

//     println!("name_4 pointer {:p}", 힙에있는_이름.as_str());    //0x151e06a30
//     println!("name_5 pointer {:p}\n", 두번째_힙에있는_이름.as_str());  //0x151e06a40 

//     println!("name pointer {:p}", &이름);               // 0x16bb72498
//     println!("name_2 pointer {:p}", &두번째_이름);           // 0x16bb724a8
//     println!("name_3 pointer {:p}", &이름_복사);           // 0x16bb724b8
//     println!("name_4 pointer {:p}", &힙에있는_이름);           // 0x16bb724c8
//     println!("name_5 pointer {:p}", &두번째_힙에있는_이름);           // 0x16bb724e0
// }

// fn test(name: &str) {
//   println!("test 함수 속 \"kim\" {:p}", "kim");                // 0x104341d29
//   println!("test 함수 속 name 매개 변수 {:p}", name);          // 0x104341d29
//   println!("test 함수 속 name 매개 변수 주소 {:p}\n", &name);    // 0x16baf61d8
// }


// fn main () {
//   let mut num = 1;
//   num = num + 1;
//   num = num + 2;

//   let added = add_num(num, num);

//   println!("{}", num);
//   println!("{}", added);
// }

// fn add_num (a:i32, b: i32) -> i32 {
//    a + b
// }

// const NAMES: [&'static str; 10] = [
//     "Kaladin", "Teft", "Drehy", "Skar", "Rock", "Sigzil", "Moash", "Leyten", "Lopen", "Hobber",
// ];

// fn main() {
//     roll_call();
// }

// pub fn roll_call() {
//     println!("SOUND OFF");
//     for name in NAMES.iter() {
//         println!("{}: HERE!", name);
//     }
//     let num_present = NAMES.len();
//     println!("All {} accounted for!", num_present);
// }

// fn main () {
//   println!("{}", "Hello World!");
// }

// fn main () {
//   // Input
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_num: usize = input.trim().parse().unwrap();

//   let mut result_str = String::new();
//   // Process
//   for i in 1..input_num + 1 {
//     result_str.push_str(&i.to_string());
//     result_str.push('\n')
//   }
//   // Output
//   println!("{}", result_str.trim_end());
// }


// fn main () {
//   // Input
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_num: usize = input.trim().parse().unwrap();

//   let mut result_str = String::new();
//   // Process
//   for i in (1..input_num + 1).rev() {
//     result_str.push_str(&i.to_string());
//     result_str.push('\n')
//   }
//   // Output
//   println!("{}", result_str.trim_end());
// }


// fn main () {
//   // [?] 구현: 스택
  
//   // 초기화
//   let mut input = String::new();
//   let mut stack: Vec<usize> = Vec::new();
  
//   // Input
//   std::io::stdin().read_line(&mut input).expect("check input value");
//   let try_count: usize = input.trim().parse().expect("check parsed value");
  
//   let mut result_str = String::new();

//   // Process
//   for _ in 0..try_count {
//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     if input.contains(" ") { // 스페이스바를 포함한다면 push 메서드임
//       let input_vec: Vec<&str> = input.split_whitespace().collect();
//       stack.push(input_vec[1].trim().parse().unwrap());
//     } else {
//       match input.trim() {
//         "size" => {
//           if stack.len() == 0 {
//             result_str.push_str("0");
//             result_str.push('\n')
//           } else {
//             result_str.push_str(&stack.len().to_string());
//             result_str.push('\n')
//           }
//         },
//         "pop" => {
//           if stack.len() == 0 {
//             result_str.push_str("-1");
//             result_str.push('\n')
//           } else {
//             result_str.push_str(&stack.pop().unwrap().to_string());
//             result_str.push('\n')
//           }
//         },
//         "empty" => {
//         if stack.len() == 0 {
//             result_str.push_str("1");
//             result_str.push('\n')
//           } else {
//             result_str.push_str("0");
//             result_str.push('\n')
//           }
//         },
//         "top" => {
//           if stack.len() == 0 {
//             result_str.push_str("-1");
//             result_str.push('\n')
//           } else {
//             result_str.push_str(&(stack[stack.len() -1]).to_string());
//             result_str.push('\n')
//           }
//         },
//         _ => println!("hello"),
//       }
//     }
//   } 
//   println!("{}", result_str.trim())
// }

// fn main () {
//   // [?] OX 퀴즈
//   // O는 맞은 것 X는 틀린 것
//   // 점수는 그 문제까지 연속된 O의 개수!!
//   // OOXXOXXOOO => 1 2 0 0 1 0 0 1 2 3

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let try_count: usize = input.trim().parse().unwrap();
//   let mut score_str = String::new();
  
//   for _ in 0..try_count {
//     let mut score :usize= 0;
//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let splited_by_x = input.trim().split("X").collect::<Vec<&str>>();

//     for i in splited_by_x {
//       if i != "" {
//         for j in 1..i.len()+1 {
//           score += j;
//         }
//       }
//     }
//     score_str.push_str(&format!("{}{}", &score.to_string(), "\n"));
    
//   }

//   println!("{}", score_str);
//   /*
//     5
//     OOXXOXXOOO
//     OOXXOOXXOO
//     OXOXOXOXOXOXOX
//     OOOOOOOOOO
//     OOOOXOOOOXOOOOX

//     10
//     9
//     7
//     55
//     30 
//    */
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_num: usize = input.trim().parse().unwrap();

//   for i in (1..=(input_num)).rev() {
//     println!("{}", "*".repeat(i));
//   }
// }

fn main () {
  // [?] 피보나치 수 구하기
  /*
    피보나치 수는 0과 1로 시작
    0번째 -> 0
    1번째 -> 1
    2번째 부터는 바로 앞 두 피보나치 수의 합
    // 0 1 1 2 3 5 8 13 21 34 ... 
    [!] n이 주어졌을 때 n번째 피보나치 수를 구하는 프로그램
   */
  
  // 초기화
  let mut present_fibonacci_num: usize = 1;
  let mut before_fibonacci_num: usize = 1;
  // 인풋
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("check input value");


  // 프로세스
  let input_num: usize = input.trim().parse().expect("check parsed value");
  if input_num == 0 {
    present_fibonacci_num = 0
  }

  if input_num == 1 {
    present_fibonacci_num = 1
  }

  if input_num == 2 {
    present_fibonacci_num = 1
  }

  if input_num >= 3 {
    for _ in 0..input_num - 2 {
      // 이전의 피보나치 수는 현재 피보나치 수
      let temp = present_fibonacci_num;
      present_fibonacci_num = before_fibonacci_num + present_fibonacci_num;
      before_fibonacci_num = temp;
      
    }
  }


  // 아웃풋
  println!("{}", present_fibonacci_num);
}