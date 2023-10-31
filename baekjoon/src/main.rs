fn main () {
  // N * M 행렬의 합을 구하라

  let mut input = String::new();
  std::io::stdin().read_line(&mut input).expect("check input value");

  let input_arr = input.split_whitespace().map(|val| val.trim().parse().expect("check parsed value")).collect::<Vec<usize>>();
  let (procession_n, _) = (input_arr[0], input_arr[1]);

  let mut procession_one: Vec<Vec<usize>> = Vec::new();
  let mut procession_two: Vec<Vec<usize>> = Vec::new();

  for i in 0..procession_n * 2 {
    input.clear();
    std::io::stdin().read_line(&mut input).expect("check input value");

    let procession_arr = input.split_whitespace().map(|val| val.trim().parse().expect("check puarsed value")).collect::<Vec<usize>>();
    if i < procession_n {
      procession_one.push(procession_arr)
    } else {
      procession_two.push(procession_arr)
    }
  }
  
  let mut result_procession: Vec<Vec<usize>> = Vec::new();
  for (key, vec) in procession_one.iter().enumerate() {
    let mut factor: Vec<usize> = Vec::new();
    for (size, _) in vec.iter().enumerate() {
      factor.push(procession_one[key][size] + procession_two[key][size]);
    }

    result_procession.push(factor);
  }

      // 결과를 원하는 형식의 문자열로 변환
  let result_str: String = result_procession.iter()
      .map(|row| row.iter()
          .map(|&value| value.to_string())
          .collect::<Vec<String>>()
          .join(" "))
      .collect::<Vec<String>>()
      .join("\n");

    println!("{}", result_str);
}