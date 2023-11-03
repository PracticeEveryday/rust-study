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

fn main () {
  // 55 -> 50 -> 05 -> 55
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("check input value");

  let cycle_num: usize = input.trim().parse().expect("check input type");
  let mut cycled_num = cycle_add(cycle_num);
  let mut count = 1;
  while cycle_num != cycled_num {
      cycled_num = cycle_add(cycled_num);
      count = count + 1;
  }
  println!("{}", count);
}

fn cycle_add(num: usize) -> usize {
    let mut result = Vec::new();
    let mut n = num;
    if num == 0 {
      return 0
    }

    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }

    result.reverse();
    
    if result.len() == 1 {
      return num * 10 + num
    } else {
      return result[1] * 10 + (result[0] + result[1]) % 10
    }
}