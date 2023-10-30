// fn main() {
//     let str_one = "김동현 아들";
//     let Str_one = String::from("김동현 딸");

//     let str_two = "김동현 아들 두번쨰";
//     let Str_two = String::from("김동현 딸 두번쨰");

//     let str_one_ptr = str_one.as_ptr();
//     let Str_one_ptr = Str_one.as_ptr();

//     let str_two_ptr = str_two.as_ptr();
//     let Str_two_ptr = Str_two.as_ptr();

//     let stack_address = &Str_one as *const String;
//     let stack_address2 = &Str_two as *const String;
//     println!("Stack address of input: {:?}", stack_address);
//     println!("Stack address of input: {:?}", stack_address2);
//     // 주의: 이 코드는 `unsafe` 블록 내에서 실행됩니다.
//     // unsafe {
//     //     println!("Stack address of input: {:?}", stack_address);
//     // }
//     // Print the addresses
//     println!("Address of str_one: {:p}", str_one_ptr);
//     println!("Address of Str_one: {:p}", Str_one_ptr);
//     println!("Address of str_two: {:p}", str_two_ptr);
//     println!("Address of Str_two: {:p}", Str_two_ptr);
// }

// fn main () {
//   println!("{}", '\n');
//   // 1. String 타입의 문자열 객체 선언
//   let string_타입 = String::from("힙"); 

//   // 2. 선언된 String 타입의 주소를 String_타입_주소에 복사합니다.
//   let string_타입_주소 = &string_타입; 

//   // 3. 변수의 주소를 출력합니다.

//   // 3-1. 힙에 있는 실제 객체의 주소
//   println!("힙에 있는 실제 객체의 주소 {:p} \n", string_타입.as_ptr()); 
//   // 3-2. 스택에 있는 변수의 실제 주소
//   println!("스택에 있는 변수의 실제 주소 {:p} \n", string_타입_주소);

//   // 4. String 타입의 소유권을 이동시킵니다.
//   let string_타입_새로운_소유자 = string_타입;
//   // 5. 이동시킨 String 타입의 주소를 복사합니다.
//   let string_타입_새로운_소유자_주소 = &string_타입_새로운_소유자;

//   // 6. 변수의 주소를 출력합니다.
//   // 6-1. 힙에 있는 실제 객체의 주소
//   println!("힙에 있는 실제 객체의 주소 {:p} \n", string_타입_새로운_소유자.as_ptr());

//   // 6-2. 새로 생성된 소유자의 스택 주소
//   println!("스택에 있는 변수의 실제 주소 {:p} \n", string_타입_새로운_소유자_주소);
  
//   println!("{}", '\n');


//   // let 스트링_타입 = String::new();
//   // let 이것도_스트링_타입 = "스트링타입".to_string();

//   // let 난_str_타입 = "data";
//   // let 이렇게하면_스트링_타입 = 난_str_타입.to_string();

//   // let 바로_값_넣고_할당 = String::from("스트링 타입이에요");

//   // let mut string = String::new();
//   // string.push('일');
//   // string.push_str("하나");
//   // dbg!(string);


//   // let str1 = String::from("Hello, ");
//   // let str2 = String::from("world!");
//   // let str3 = str1 + str2; // 여기서 s1 은 '이동' 이 발생함. 변수 s1 은 이제 사용 불가.

//   // dbg!(str1); // use of moved value: `s1` value used hear after move
//   // dbg!(str2);
//   // dbg!(str3);

//     // let string1 = String::from("김");
//     // let string2 = String::from("동");
//     // let string3 = String::from("현");
    
//     // let string_add = format!("{}{}{}", string1, string2 ,string3);

//     // dbg!(string1);
//     // dbg!(string2);
//     // dbg!(string3);
//     // dbg!(string_add);

//     // let string1 = String::from("hello");
//     // let idx = string1[0];
//     // //the type `String` cannot be indexed by `{integer}` the trait `Index<{integer}>` is not implemented for `String` the following other types implement trait `Index<Idx>`:

//     let string1 = String::from("abcde");
//     dbg!(string1.len()); // 5

//     let string2 = String::from("ㄱㄴㄷㄹㅁ");
//     dbg!(string2.len()); // 15
// }

// fn main () {
//     let string2 = String::from("ㄱㄴ");
//     dbg!(&string2); // 15

//     for c in string2.chars() {
//       println!("{}", c);
//     }
//     for c in string2.bytes() {
//       println!("{}", c);
//     }

//     dbg!(&string2[0..3]); // ㄱ
//     dbg!(&string2[0..4]); // it is inside 'ㄴ' (bytes 3..6) of `ㄱㄴ`'
// }

// fn main () {
//   let str1 = String::from("test");
//   let str2 = String::from("test");

//   println!("{:p}", str1.as_ptr());
//   println!("{:p}", str2.as_ptr());

//   dbg!(str2);
// }


// fn main() {
//   print!("         ,r'\"7\n");
//   print!("r`-_   ,'  ,/\n");
//   print!(" \\. \". L_r'\n");
//   print!("   `~\\/\n");
//   print!("      |\n");
//   print!("      |\n");
// }

/*
         ,r'"7
r`-_   ,'  ,/
 \. ". L_r'
   `~\/
      |
      |

*/


// fn main () {
//   // background
//   // 흰색 체스말에는 여러개가 없다.
//   // 킹 퀸 룩 비슙 나이트 폰
//   // 1  1  2   2     2    8
//   // 개수가 주어 질 때 몇 개를 더하고 빼야 되는 지 출력해라

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   // 원래 있어야 하는 체스의 개수
//   let collect_arr = [1, 1, 2, 2, 2, 8];

//   let white_chess_arr: Vec<i32> = input.split_whitespace()
//                                         .map(|val| val.trim().parse().expect("check parsed type")).collect();

//   let mut result_str = String::new();

//   white_chess_arr.iter().enumerate().for_each(|(idx, val)| {
//     if val == &collect_arr[idx] {
//       result_str.push_str("0 ");
//     } else {
//       let sub_num: i32 = &collect_arr[idx] - val;
//       result_str.push_str(&format!("{} ", &sub_num.to_string()));
//     }
//   });

//   println!("{}", result_str);
//   // 0 1 2 2 2 7 
//   // -> 1 0 0 0 1
// }

// use std::thread;

// fn main() {
//     let mut shared_data = 0;

//     let thread1 = thread::spawn(move || {
//         for _ in 0..10 {
//             shared_data = shared_data + 1;
//             println!("Thread 1: {}", shared_data);
//         }
//     });

//     let thread2 = thread::spawn(move || {
//         for _ in 0..10 {
//             shared_data = shared_data - 1;
//             println!("Thread 2: {}", shared_data);
//         }
//     });

//     thread1.join().unwrap();
//     thread2.join().unwrap();

//     dbg!(shared_data);
// }

// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let shared_data = Arc::new(Mutex::new(0));

//     let thread1 = {
//         let shared_data = Arc::clone(&shared_data);
//         thread::spawn(move || {
//             for _ in 0..10 {
//                 let mut data = shared_data.lock().unwrap();
//                 *data += 1;
//                 println!("Thread 1: {}", *data);
//             }
//         })
//     };

//     let thread2 = {
//         let shared_data = Arc::clone(&shared_data);
//         thread::spawn(move || {
//             for _ in 0..10 {
//                 let mut data = shared_data.lock().unwrap();
//                 *data -= 1;
//                 println!("Thread 2: {}", *data);
//             }
//         })
//     };

//     thread1.join().unwrap();
//     thread2.join().unwrap();

//     let data = shared_data.lock().unwrap();
//     println!("Final data: {}", *data);
// }

// fn main () {
//   let char_en = 'a';
//   let char_ko = '한';
//   dbg!(char_en);
//   dbg!(char_ko);

//   let 라틴_A = char::from_u32(0x0041).unwrap();     // U+0041
//   let 그리스_A = char::from_u32(0x0391).unwrap(); // U+0391
  

//   println!("Latin A: {}", 라틴_A);  
//   println!("Greek Alpha: {}", 그리스_A);

//   let char_김: char = '김';
    
//   println!("알파벳 여부 {}", char_김.is_alphabetic()); // true
//   println!("숫자 여부 {}", char_김.is_numeric()); // false
//   println!("숫자 여부 {}", char_김.is_digit(10)); // false
//   println!("알파벳이나 숫자인지 아닌지 여부 {}", char_김.is_alphanumeric()); // true
//   println!("소문자 여부 {}", char_김.is_lowercase());  // false
//   println!("대문자 여부 {}", char_김.is_uppercase());  // false
//   println!("공백 여부 {}", char_김.is_whitespace()); // false
//   println!("아스키 코드 여부 {}", char_김.is_ascii());  // false
//   println!("대문자로 변환 {}", char_김.to_uppercase());  // 김
//   println!("utf-8 바이트 길이 반환 {}", char_김.len_utf8());  // 3
//   // println!("숫자로 변환 Node {}", char_김.to_digit(10).expect("check char value")); // check char value


//   println!("{}", 'a'.is_alphabetic());
//   println!("{}", '京'.is_alphabetic());

//   let c = '💝';
  
//   println!("{}", c.is_alphabetic());
// }


// fn main () {
//   let latin_a = char::from_u32(0x0041).unwrap();
//   let char_2 = char::from_u32(0x0042).unwrap();
//   let char_3 = char::from_u32(0x0060).unwrap();

//   dbg!(latin_a);                  // A
//   dbg!(latin_a.is_alphabetic());  // true
//   dbg!(char_2);                   // B
//   dbg!(char_2.is_alphabetic());   // true
//   dbg!(char_3);                   // `
//   dbg!(char_3.is_alphabetic());   // false
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let word = input.trim();
//   let reversed: String = word.chars().rev().collect();
  
//   if word == reversed {
//     println!("{}", 1)
//   } else {
//     println!("{}", 0)
//   }
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let word = input.trim();
//   let word_arr: Vec<char> = word.chars().collect();

//   let mut word_hashmap: std::collections::HashMap<&char, usize> = std::collections::HashMap::new();

//   for i in word_arr.iter() {
//     if let Some(_) = word_hashmap.get(i) {
//         // 해당 키가 존재하는 경우 처리
//         word_hashmap.insert(i, word_hashmap.get(i).unwrap() + 1);
//       } else {
//         // 해당 키가 없는 경우 처리
//         word_hashmap.insert(i, 1);
//       }
//   }
//   let mut max_value = 0;
//   let mut max_vec: Vec<char> = Vec::new();
//   for (&key, &value) in word_hashmap.iter() {
//     if value > max_value {
//       max_value = value
//     } else {
//       max_vec.push(key.clone())
//     }
//   }
  
//   if max_vec.len() < 1 {
//     println!("{}", "?");
//   } else {
//     println!("{}", max_value);
//   }
// }

//TODO: 한번 더 보기!! * 활용
// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let word = input.trim();
//   let word_arr: Vec<char> = word.to_lowercase().chars().collect();
  
//   let mut word_hashmap: std::collections::HashMap<char, usize> = std::collections::HashMap::new();

//   for c in word_arr {
//     if let Some(count) = word_hashmap.get(&c) {
//       word_hashmap.insert(c, count + 1);
//     } else {
//       word_hashmap.insert(c, 1);
//     }
//   }

//   let mut max_value: usize = 0;
//   let mut max_vec: Vec<char> = Vec::new();

//   for (key, value) in word_hashmap.iter() {
//     if *value > max_value {
//       max_value = *value;
//       max_vec = Vec::new();
//       max_vec.push(*key);
//     } else if *value == max_value {
//       max_vec.push(*key);
//     } else {
      
//     }
//   }

//   if max_vec.len() > 1 {
//     println!("{}", "?");
//   } else {
//     println!("{}", max_vec[0].to_uppercase());
//   }
// }

// fn main () {
//   // 개수를 구하라

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   // 이 목록에 없는 알파벳은 하나씩 센다.
//   let croatia_alpha = ["c=", "c-", "dz=", "d-", "lj", "nj", "s=", "z="];

//   let mut word = input.trim().to_string();

//   for i in croatia_alpha {
//     dbg!(&i);
//     word = word.replace(i, "*");
//   }

//   println!("{}", word.len());
// }


// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let try_count: usize = input.trim().parse().expect("check parsed value");

//   let mut group_word_count: usize = 0;

//   for _ in 0..try_count {
//     let mut is_grouped_word: bool = true;
//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input value");

//     let word = input.trim();
//     let mut exist_vec: Vec<char> = Vec::new();
//     let mut cached_char: char = 'a';

//     for (idx, c) in word.chars().enumerate() {
//       // 첫번째는 캐시데이터와 내역을 푸쉬한다.
//       if idx == 0 {
//         cached_char = c;
//         exist_vec.push(c);
//       } 

//       // 첫번째 부터는 비교한다.
//       if cached_char != c {
//         // c가 exist_vec에 존재하면 그룹 단어가 아니다!
//         if exist_vec.contains(&c) {
//           is_grouped_word = false;
//           cached_char = c;
//         }else {
//           exist_vec.push(c);
//           cached_char = c;
//         }
//       }
//     }
//     if is_grouped_word {
//         group_word_count = group_word_count + 1;
//     }
//   }
//   println!("{}", group_word_count);
// }

fn main () {
  // -- background
  // 전공 평점은 학점 * 과목 평점의 합 / 학점의 총 합
  

  // -- condition
  // 20 줄에 걸쳐 과목명, 학점, 등급이 공백으로 구분되어 주어진다.
  // pass 과목은 계산에서 제외한다!


}