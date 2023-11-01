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

fn main () {
  // 9 X 9 행렬이 주어질 때 최대값과 위치를 구하라!
  let mut procession_arr:Vec<Vec<usize>> = Vec::new();


  for _ in 0..9 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("check input value");
    
    // 9x9 행렬 생성
    let input_arr: Vec<usize> = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect();
    procession_arr.push(input_arr);

    input.clear();
  }

  let mut max_val = 0;
  let mut max_val_coordinate = String::new();

  for (key, vec) in procession_arr.iter().enumerate() {
    for (key_depth_two, _) in vec.iter().enumerate() {
      if max_val <= procession_arr[key][key_depth_two] {
        max_val = procession_arr[key][key_depth_two];
        max_val_coordinate.clear();
        max_val_coordinate.push_str(&format!("{} {}", key + 1, key_depth_two + 1));
      }
    }
  }

  println!("{}", max_val);
  println!("{}", max_val_coordinate);

  /*
  3 23 85 34 17 74 25 52 65
  10 7 39 42 88 52 14 72 63
  87 42 18 78 53 45 18 84 53
  34 28 64 85 12 16 75 36 55
  21 77 45 35 28 75 90 76 1
  25 87 65 15 28 11 37 28 74
  65 27 75 41 7 89 78 64 39
  47 47 70 45 23 65 3 41 44
  87 13 82 38 31 12 29 29 90

  90
  5 7  
   */
}