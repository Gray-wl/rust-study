// trait上带类型参数
// trait TraitA<T> {}
// struct Atype;
// impl<T> TraitA<T> for Atype {}
// impl TraitA<u8> for Atype {}

// use std::fmt::Debug;

// trait TraitA<T> {}
// struct Atype<U> {
//     a: U,
// }
// impl<T, U> TraitA<T> for Atype<U>
// where
//     T: Debug,
//     U: PartialEq,
// {
// }

// impl示例
// trait Add<T> {
//     type Output;
//     fn add(self, rhs: T) -> Self::Output;
// }
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl Add<Point> for Point {
//     type Output = Self;
//     fn add(self, rhs: Point) -> Self::Output {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }
// impl Add<i32> for Point {
//     type Output = Self;
//     fn add(self, rhs: i32) -> Self::Output {
//         Point {
//             x: self.x + rhs,
//             y: self.y + rhs,
//         }
//     }
// }

// trait类型参数的默认实现
// trait TraitA<T = Self> {
//     fn func(t: T) {}
// }
// trait TraitB<T = i32> {
//     fn func2(t: T) {}
// }
// struct SomeType;
// impl TraitA for SomeType {
//     fn func(t: SomeType) {}
// }
// impl TraitB for SomeType {
//     fn func2(t: i32) {}
// }
// impl TraitA<String> for SomeType {
//     fn func(t: String) {}
// }
// impl TraitB<String> for SomeType {
//     fn func2(t: String) {}
// }

// trait中的类型参数与关联类型的区别
// use std::fmt::Debug;
// trait TraitA<T>
// where
//     T: Debug,
// {
//     fn play(&self, _t: T) {}
// }
// struct Atype;
// // impl<T> TraitA<T> for Atype where T: Debug + PartialEq {}
// impl TraitA<u32> for Atype {}

// trait Add {
//     type ToAdd;
//     type Output;
//     fn add(self, rhs: Self::ToAdd) -> Self::Output;
// }
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl Add for Point {
//     type ToAdd = Point;
//     type Output = Point;
//     fn add(self, rhs: Point) -> Point {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }
// impl Add for Point { // conflicting implementation for `Point`
//     type ToAdd = i32;
//     type Output = Point;
//     fn add(self, rhs: i32) -> Point {
//         Point {
//             x: self.x + rhs,
//             y: self.y + rhs,
//         }
//     }
// }

// trait object
struct Atype;
struct Btype;
struct Ctype;
// enum TotalType {
//     A(Atype),
//     B(Btype),
//     C(Ctype),
// }
// fn doit(i: u32) -> TotalType {
//     if i == 0 {
//         let a = Atype;
//         TotalType::A(a)
//     } else if i == 1 {
//         let b = Btype;
//         TotalType::B(b)
//     } else {
//         let c = Ctype;
//         TotalType::C(c)
//     }
// }

// trait TraitA {
//     fn new() -> Self;
// }
// impl TraitA for Atype {
//     fn new() -> Atype {
//         Atype
//     }
// }
// impl TraitA for Btype {
//     fn new() -> Btype {
//         Btype
//     }
// }
// impl TraitA for Ctype {
//     fn new() -> Ctype {
//         Ctype
//     }
// }
// fn doit<T: TraitA>() -> T {
//     T::new()
// }

// dyn
// trait TraitA {}
// impl TraitA for Atype {}
// impl TraitA for Btype {}
// impl TraitA for Ctype {}
// fn doit(i: u32) -> Box<dyn TraitA> {
//     if i == 0 {
//         let a = Atype;
//         Box::new(a)
//     } else if i == 1 {
//         let b = Btype;
//         Box::new(b)
//     } else {
//         let c = Ctype;
//         Box::new(c)
//     }
// }

// 利用trait object 传参
// trait TraitA {}
// impl TraitA for Atype {}
// impl TraitA for Btype {}
// impl TraitA for Ctype {}
// fn doit(x: impl TraitA) {}
// // 等价于
// // fn doit<T: TraitA>(x: T) {}

// dyn trait 示例
// trait TraitA {}
// impl TraitA for Atype {}
// impl TraitA for Btype {}
// impl TraitA for Ctype {}
// fn doit(x: &dyn TraitA) {}

// 利用 trait obj 将不同的类型装进集合里
trait TraitA {}
impl TraitA for Atype {}
impl TraitA for Btype {}
impl TraitA for Ctype {}

fn main() {
    // let p1 = Point { x: 1, y: 1 };
    // let p2 = Point { x: 2, y: 2 };
    // let p3 = p1.add(p2);
    // assert_eq!(p3.x, 3);
    // assert_eq!(p3.y, 3);
    // let p1 = Point { x: 1, y: 1 };
    // let delta = 2;
    // let p3 = p1.add(delta);
    // assert_eq!(p3.x, 3);
    // assert_eq!(p3.y, 3);

    // let a = Atype;
    // a.play(10u32);

    // let a = doit::<Atype>();
    // let b = doit::<Btype>();
    // let c = doit::<Ctype>();

    // let a = Atype;
    // doit(a);
    // let b = Btype;
    // doit(b);
    // let c = Ctype;
    // doit(c);

    // let a = Atype;
    // doit(&a);
    // let b = Btype;
    // doit(&b);
    // let c = Ctype;
    // doit(&c);
    let a = Atype;
    let b = Btype;
    let c = Ctype;
    let v: Vec<&dyn TraitA> = vec![&a, &b, &c];
}
