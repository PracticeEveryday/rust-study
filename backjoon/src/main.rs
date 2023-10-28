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

fn main () {
  let str1 = String::from("test");
  let str2 = String::from("test");

  println!("{:p}", str1.as_ptr());
  println!("{:p}", str2.as_ptr());

  dbg!(str2);
}