// ---- 문자열 시작

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   let input_str = &input;

//   input.clear();
//   std::io::stdin().read_line(&mut input).expect("check input value");
//   let input_num: usize = input.trim().parse().expect("check input type");

//   let input_idx_char = input_str.chars().nth(input_num - 1).unwrap();

//   println!("{}", input_idx_char);
// }

// fn main () {
//   let mut input= String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   println!("{}", input.trim().len());
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");
  
//   let try_count: i32 = input.trim().parse().expect("check parsed input value");
//   let mut result_str = String::new();
//   for _ in 0..try_count {
//     input.clear();
//     std::io::stdin().read_line(&mut input).expect("check input value");
//     let input_str = input.trim();

//     let first_char = input_str.chars().nth(0).unwrap();
//     result_str.push(first_char);
//     let last_char = input_str.chars().nth(input_str.len() -1).unwrap();
//     result_str.push(last_char);
//     result_str.push('\n');
//   }

//   println!("{}", result_str);
// }

//TODO: u8으로 바꾸면 아스키 코드 값이 나온다!
// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");

//   println!("{}", input.trim().chars().next().unwrap() as u8);
// }

// fn main () {
//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check ");

//   input.clear();
//   std::io::stdin().read_line(&mut input).expect("check ");

//   let input_str = input.trim();
//   let input_arr = input_str.chars().map(|val| val.to_digit(10).unwrap() as i32).collect::<Vec<i32>>();
//   let sum: i32 = input_arr.iter().sum();

//   println!("{}", sum);
// }

// fn main () {
//   let alphabet = "abcdefghijklmnopqrstuvwxyz";

//   let mut input = String::new();
//   std::io::stdin().read_line(&mut input).expect("check input value");
//   let input_str = input.trim();
//   let mut result_str = String::new();

//   for i in 0..alphabet.len() {
//     // input str에 알파벳이 있다면
//     let char = input_str.find(alphabet.chars().nth(i).unwrap());
//     match char {
//         Some(value) => {
//             dbg!(&value);
//             result_str.push_str(&format!("{} ", value));
//         },
//         None => {
//             result_str.push_str(&format!("{} ", "-1"));
//         },
//     }

//   }

//   println!("{}", result_str);
// }

fn main () {
  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("check input value");

  let try_count: usize = input.trim().parse().expect("check parsed type");

  let mut result_str = String::new();
  for _ in 0..try_count {
    input.clear();
    std::io::stdin().read_line(&mut input).expect("check input value");

    let input_vec:Vec<_> = input.split_whitespace().collect();
    // 카운트할 횟수 num, 글자마자 반복할 문자열 repeat_str
    let (num, repeat_str) = (input_vec[0], input_vec[1]);

    let repeat_count: usize = num.trim().parse().expect("check parsed value");
    let repeated_vec: Vec<String> = repeat_str.chars().map(|val| val.to_string().repeat(repeat_count)).collect();
    let repeated_str = repeated_vec.join("");
    result_str.push_str(&repeated_str);
    result_str.push('\n')
  }

  println!("{}", result_str);
}