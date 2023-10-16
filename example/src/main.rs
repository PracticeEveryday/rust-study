fn main () {
    // i32 -> 정수형 32비트를 뜻한다.
    let a: i32 = 2;

    let result = stack_only(a);
    dbg!(result);

}

fn stack_only(b: i32) -> i32 {
    let c = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let d = 5;
    // Box는 러스트의 스마트 포인터 타입이다.
    let e = Box::new(7);
    return d + *e;
}