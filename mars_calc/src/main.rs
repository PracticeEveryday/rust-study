use std::io;

fn main() {
    println!("몸무게를 입력하세요 (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();
    dbg!(weight);

    let mars_weight = calculate_weight_on_mars(weight);
    println!("변환된 화성에서의 몸무게: {}kg", mars_weight);
}
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

