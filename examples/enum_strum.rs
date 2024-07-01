use strum::{self, Display, EnumCount, EnumDiscriminants, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, EnumString, EnumCount, VariantNames, EnumDiscriminants, IntoStaticStr)]
enum MyEnum {
    A(String),
    B,
    C,
}

#[allow(unused)]
#[derive(Display, Debug)]
enum Color {
    #[strum(serialize = "redred")]
    Red,
    Green {
        range: usize,
    },
    Blue(usize),
    Yellow,
    #[strum(to_string = "purple with {sat} saturation")]
    Purple {
        sat: usize,
    },
}

fn main() {
    // VariantNames	Adds an associated VARIANTS constant which is an array of discriminant names
    println!(
        "MyEnum has {} variants: {:?}",
        MyEnum::VARIANTS.len(),
        MyEnum::VARIANTS
    );

    //IntoStaticStr	Implements From<MyEnum> for &'static str on an enum
    let my_enum = MyEnum::A("hello".to_string());
    let s: &'static str = my_enum.into();
    println!("static str enum: {}", s);

    //
    let red = Color::Red;
    let green = Color::Green { range: 10 };
    let blue = Color::Blue(10);
    let yellow = Color::Yellow;
    let purple = Color::Purple { sat: 10 };
    println!(
        "red: {}, green: {}, blue: {}, yellow: {}, purple: {}",
        red, green, blue, yellow, purple
    );
}
