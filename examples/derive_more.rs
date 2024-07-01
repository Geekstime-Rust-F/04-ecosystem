use derive_more::{Add, Display, From, Into};

#[derive(PartialEq, From, Into, Add, Debug, Copy, Clone)]
struct MyInt(i32);

#[derive(PartialEq, From, Into)]
struct Point2D {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, From, Add, Display)]
enum MyEnum {
    #[display(fmt = "int: {}", _0)]
    Int(i32),
    Uint(u32),
    #[display(fmt = "nothing")]
    Nothing,
}

fn main() {
    let my_int: MyInt = 10.into();
    let result = my_int + 20.into();
    let result1: i32 = result.into(); // Into trait
    println!("my_int: {:?}", my_int);
    println!("result: {:?}", result);
    println!("result1: {:?}", result1);
    assert_eq!(result, MyInt(30));

    let e: MyEnum = 10i32.into();
    let e1: MyEnum = 10u32.into();
    let e2: MyEnum = MyEnum::Nothing;
    println!("e: {}, e1: {}, e2: {}", e, e1, e2);
    println!("e: {:?}, e1: {:?}, e2: {:?}", e, e1, e2);
}
