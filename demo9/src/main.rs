// trait 是一种约束
// struct Point<T> {
//     x: T,
//     y: T,
// }

// struct Foo;

// fn print<T: std::fmt::Display>(p: Point<T>) {
//     println!("Point {}, {}", p.x, p.y);
// }

// trait TraitA {}
// struct Atype;
// impl TraitA for Atype {}

// 关联函数
// trait Sport {
//     fn play(&self);
//     fn play_mut(&mut self);
//     fn play_own(self);
//     fn play_some() -> Self;
// }

// struct Football;
// impl Sport for Football {
//     fn play(&self) {}
//     fn play_mut(&mut self) {}
//     fn play_own(self) {}
//     fn play_some() -> Self {
//         Self
//     } // 这里的Self指Football这个类型
// }

// 关联类型
// pub trait Sport {
//     type SportType;
//     fn play(&self, st: Self::SportType);
// }
// pub enum SportType {
//     Land,
//     Water,
// }
// impl Sport for Football {
//     type SportType = SportType;
//     fn play(&self, st: Self::SportType) {}
// }

// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// trait TraitA {
//     type MyType;
// }
// fn doit<T: TraitA>(a: T::MyType) {}
// struct TypeA;
// impl TraitA for TypeA {
//     type MyType = String;
// }

// 在约束中具化关联类型
// trait TraitA {
//     type Item;
// }
// struct Foo<T: TraitA<Item=String>> {
//     x: T
// }
// struct A;
// impl TraitA for A {
//     type Item = String;
// }

// 对关联类型的约束
// use std::fmt::Debug;
// trait TraitA {
//     type Item: Debug;
// }
// #[derive(Debug)]
// struct A;
// struct B;
// impl TraitA for B {
//     type Item = A;
// }
// fn doit<T>()
// where
//     T: TraitA,
//     T::Item: Debug + PartialEq,
// {
// }

// 关联常数
// trait TraitA {
//     const LEN: u32 = 10;
// }
// struct A;
// impl TraitA for A {
//     const LEN: u32 = 12;
// }

// Where
// fn doit<T: TraitA + TraitB + TraitC + TraitD + TraitE>(t: T) -> i32 {}
// // 可以改成
// fn doit<T>(t: T) -> i32
// where
//     T: TraitA + TraitB + TraitC + TraitD + TraitE,
// {
// }

// 约束依赖
// trait TraitA + TraitB {}
// trait Shape { fn area(&self) -> f64; }
// trait Circle: Shape { fn radius(&self) -> f64; }
// // 等价于
// trait Shape { fn area(&self) -> f64; }
// trait Circle where Self: Shape { fn radius(&self) -> f64; }

// 约束中同名方法的访问
// trait Shape {
//     fn play(&self) {
//         println!("1");
//     }
// }
// trait Circle: Shape {
//     fn play(&self) {
//         println!("2");
//     }
// }
// struct A;
// impl Shape for A {}
// impl Circle for A {}
// impl A {
//     fn play(&self) {
//         println!("3");
//     }
// }

// mod module_a {
//     pub trait Shape {
//         fn play(&self) {
//             println!("1");
//         }
//     }
//     pub struct A;
//     impl Shape for A {}
// }
// mod module_b {
//     use super::module_a::Shape;
//     use super::module_a::A;
//     fn doit() {
//         let a = A;
//         a.play();
//     }
// }

// use std::fmt::Display;
// struct Pair<T> {
//     x: T,
//     y: T,
// }
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x + y }
//     }
// }
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {}", self.x);
//         } else {
//             println!("The largest member is y = {}", self.y);
//         }
//     }
// }

// use std::fmt::Display;
// struct A;
// impl Display for A {}

// impl Display for u32 {} // `u32` is not defined in the current crate

// use std::fmt::Display;
// struct MyU32(u32);
// impl Display for MyU32 {}
// impl MyU32 {
//     fn get(&self) -> u32 {
//         self.0
//     }
// }

// Blanket Implementation
trait TraitA {}
trait TraitB {}
impl<T: TraitB> TraitA for T {}
// impl TraitB for u32 {}
// impl TraitA for u32 {} // conflicting implementation for `u32`
impl TraitA for u32 {}

fn main() {
    // let p = Point { x: 10, y: 20 };
    // print(p);
    // let p = Point { x: 10.2, y: 20.4 };
    // print(p);
    // let p = Point { x: Foo, y: Foo }; // required by this bound in `print`
    // print(p);

    // let mut f = Football;
    // f.play();
    // f.play_mut();
    // f.play_own();
    // let _g = Football::play_some();
    // let _g = <Football as Sport>::play_some();

    // let f = Football;
    // f.play(SportType::Land);

    // doit::<TypeA>("abc".to_string());

    // let a = Foo {
    //     x: A,
    // };

    // println!("{:?}", A::LEN);
    // println!("{:?}", <A as TraitA>::LEN);

    // let a = A;
    // a.play();
    // <A as Circle>::play(&a);
    // <A as Shape>::play(&a);
}
