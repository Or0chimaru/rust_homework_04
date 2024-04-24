/*
为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用Trait Object实现类型方法的调用。
*/

use std::ops::Add;

struct MyType {
    value: i32,
}

impl Add for MyType {
    type Output = MyType;

    fn add(self, other: MyType) -> MyType {
        MyType {
            value: self.value + other.value,
        }
    }
}

trait AddTrait {
    fn addition(&self, other: &Self) -> Self;
}

impl AddTrait for MyType {
    fn addition(&self, other: &Self) -> Self {
        MyType {
            value: self.value + other.value,
        }
    }
}

fn perform_addition(obj1: &dyn AddTrait, obj2: &dyn AddTrait) -> MyType {
    obj1.addition(obj2)
}

fn main() {
    let my_type1 = MyType { value: 10 };
    let my_type2 = MyType { value: 20 };

    let result = perform_addition(&my_type1, &my_type2);
    println!("Result of addition: {}", result.value);
}
