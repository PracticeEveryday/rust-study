use std::io;

// fn main() {
//     let mut numbersArr = String::new();
//     dbg!(&numbersArr);
//     io::stdin().read_line(&mut numbersArr).unwrap();
    
//     let numbers: Vec<&str> = numbersArr.split_whitespace().collect();
//     dbg!(&numbers);
    
//     let number_first = numbers[0].parse::<i32>().unwrap();
//     let number_second = numbers[1].parse::<i32>().unwrap();

//     println!("{}", number_first + number_second);
// }

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number_arr:Vec<&str> = input.split_whitespace().collect();
    
    let a: i32 = number_arr[0].trim().parse().unwrap();
    let b: i32 = number_arr[1].trim().parse().unwrap();

    // dbg!(number_arr);
    // dbg!(a);
    // dbg!(b);

     println!("{}", a - b);
}