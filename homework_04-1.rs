/* 
使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

两种不同实现方法的区别
    枚举和Trait Object的区别在于，枚举是一个封闭的类型集合，所有可能的类型在编译时就已知，
    而Trait Object允许在运行时动态选择不同类型的实现。
    枚举更适合在类型有限且已知的情况下使用，而Trait Object更适合在需要动态分发不同类型方法的情况下使用。
*/

trait MyTrait {
    fn do_something(&self);
}

struct Type1(u32);
impl MyTrait for Type1 {
    fn do_something(&self) {
        println!("Type1: {}", self.0);
    }
}

struct Type2(String);
impl MyTrait for Type2 {
    fn do_something(&self) {
        println!("Type2: {}", self.0);
    }
}

struct Type3(bool);
impl MyTrait for Type3 {
    fn do_something(&self) {
        println!("Type3: {}", self.0);
    }
}

enum MyEnum {
    Type1(u32),
    Type2(String),
    Type3(bool),
}

fn main() {
    let enum_vec: Vec<MyEnum> = vec![
        MyEnum::Type1(42),
        MyEnum::Type2(String::from("Hello")),
        MyEnum::Type3(true),
    ];

    for item in &enum_vec {
        match item {
            MyEnum::Type1(val) => {
                println!("Type1: {}", val);
                // Call Type1 specific method
            }
            MyEnum::Type2(val) => {
                println!("Type2: {}", val);
                // Call Type2 specific method
            }
            MyEnum::Type3(val) => {
                println!("Type3: {}", val);
                // Call Type3 specific method
            }
        }
    }

    let trait_objects: Vec<Box<dyn MyTrait>> = vec![
        Box::new(Type1(42)),
        Box::new(Type2(String::from("Hello"))),
        Box::new(Type3(true)),
    ];

    for item in &trait_objects {
        item.do_something();
    }
}
