use backtrace::Backtrace;

fn main() {
    // let heap = String::from("test");
    // let ptr_1 = &heap as *const String;

    // let stack_1 = 123;
    // let ptr_2 = &stack_1 as *const i32;
    // dbg!(*ptr_2);
    // let stack_2 = "hello";
    // let ptr_3 = &stack_2 as *const &str;

    // let x = 42;
    // let ptr_4 = &x as *const i32;

    // dbg!(&heap);
    // dbg!(&stack_1);
    // dbg!(&stack_2);

    // dbg!(ptr_1);
    // dbg!(ptr_2);
    // dbg!(ptr_3);
    // dbg!(ptr_4);
    
    let a = String::from("test");
    let b = a;

    println!("{}", &b);
    

    struct Foo{}

    let z = Foo{};
    let ptr_z = std::ptr::addr_of!(z);
    println!("Address of ptr_z: {:p}", ptr_z);
    println!("z addr {:p}", &z);
    println!("{}", std::mem::size_of_val(&z));
    let bt = Backtrace::new();

    // do_some_work();

    // println!("{:?}", bt);

    let n = 27;
    let m = &n;

    println!("{}", *m);
    println!("{}", &m);

    let name = "Alice";
    let ptr = &name as *const &str;
    println!("{}", std::mem::size_of_val(&name));
    println!("My name is {}.", name); // Prints: My name is Alice.
    println!("Pointer address: {:p}", ptr); // Prints: Pointer address: 0x7ffee0d3b0a0 (the actual address will vary)

    let value = 42;
    let ptr1 = std::ptr::addr_of!(value);

    println!("Value: {}", value);
    println!("Address of value: {:p}", ptr1);
}
