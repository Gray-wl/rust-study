// 标准库中的常用trait
// Default
// trait Default {
//     fn default() -> Self;
// }
// #[derive(Debug)]
// struct Color(u8, u8, u8);
// impl Default for Color {
//     fn default() -> Self {
//         Color(0, 0, 0)
//     }
// }

// fn paint(color: Option<Color>) {
//     let color = color.unwrap_or_default();
// }
// fn guarantee_length<T: Default>(mut vec: Vec<T>, min_len: usize) -> Vec<T> {
//     for _ in 0..min_len.saturating_sub(vec.len()) {
//         vec.push(T::default());
//     }
//     vec
// }

// impl Color {
//     fn new(r: u8, g: u8, b: u8) -> Self {
//         Color { r, g, b }
//     }
// }
// impl Color {
//     fn red(r: u8) -> Self {
//         Color {
//             r,
//             ..Color::default()
//         }
//     }
//     fn green(g: u8) -> Self {
//         Color {
//             g,
//             ..Color::default()
//         }
//     }
//     fn blue(b: u8) -> Self {
//         Color {
//             b,
//             ..Color:default()
//         }
//     }
// }

// Display
// trait Display {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result;
// }
// use std::fmt;
// #[derive(Default)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

// ToString
// trait ToString {
//     fn to_string(&self) -> String;
// }
// impl<T: Display> ToString for T {}

// fn display_point() {
//     let origin = Point::default();
//     assert_eq!(format!("{}", origin), "(0, 0)");
// }
// fn point_to_string() {
//     let origin = Point::default();
//     assert_eq!(origin.to_string(), "(0, 0)");
// }
// fn display_equals_to_string() {
//     let origin = Point::default();
//     assert_eq!(format!("{}", origin), origin.to_string());
// }

// Debug
// #[derive(Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// PartialEq 和 Eq
// #[derive(PartialEq, Debug)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn example_assert(p1: Point, p2: Point) {
//     assert_eq!(p1, p2);
// }
// fn example_compare_collection<T: PartialEq>(vec1: Vec<T>, vec2: Vec<T>) {
//     if vec1 == vec2 {
//     } else {
//     }
// }

// PartialOrd 和 Ord
// #[derive(PartialEq, PartialOrd)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// #[derive(PartialEq, PartialOrd)]
// enum Stoplight {
//     Red,
//     Yellow,
//     Green,
// }

// use std::collections::BTreeSet;
// #[derive(Ord, PartialOrd, PartialEq, Eq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// fn example_btreeset() {
//     let mut points = BTreeSet::new();
//     points.insert(Point { x: 0, y: 0 });
// }
// fn example_sort<T: Ord>(mut sortable: Vec<T>) -> Vec<T> {
//     sortable.sort();
//     sortable
// }

//运算符重载
// trait Add<Rhs = Self> {
//     type Output;
//     fn add(self, rhs: Rhs) -> Self::Output;
// }
// use std::ops::Add;
// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl Add for Point {
//     type Output = Point;
//     fn add(self, rhs: Point) -> Point {
//         Point {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

// Clone
// trait Clone {
//     fn clone(&self) -> Self;
// }
// #[derive(Clone)]
// struct Point {
//     x: u32,
//     y: u32,
// }

// Copy
// trait Copy: Clone {}
// impl Copy for Point {} // 这是不行的
// #[derive(Copy, Clone)]
// struct SomeType;

// #[derive(Clone, Debug)]
// struct Atype {
//     num: u32,
//     a_vec: Vec<u32>,
// }

// ToOwned
// let a: &str = "123456";
// let s: String = a.to_owned();

// Deref

// Drop
// trait Drop {
//     fn drop(&mut self);
// }
// struct A;
// impl Drop for A {
//     fn drop(&mut self) {}
// }

// 闭包相关trait
// trait FnOnce<Args> {
//     type Output;
//     fn call_once(self, args: Args) -> Self::Output;
// }
// trait FnMut<Args>: FnOnce<Args> {
//     fn call_mut(&mut self, args: Args) -> Self::Output;
// }
// trait Fn<Args>: FnMut<Args> {
//     fn call(&self, args: Args) -> Self::Output;
// }

// From<T> 和 Into<T>
// trait From<T> {
//     fn from(T) -> Self;
// }
// trait Into<T> {
//     fn into(self) -> T;
// }
// impl<T, U> Into<U> for T
// where
//     U: From<T>,
// {
//     fn into(self) -> U {
//         U::from(self)
//     }
// }
// fn function<T>(t: T)
// where
//     T: From<i32>,
//     i32: Into<T>,
// {
//     let example: T = T::from(0);
//     let example: T = 0.into();
// }
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl From<(i32, i32)> for Point {
//     fn from((x, y): (i32, i32)) -> Self {
//         Point { x, y }
//     }
// }
// impl From<[i32; 2]> for Point {
//     fn from([x, y]: [i32; 2]) -> Self {
//         Point { x, y }
//     }
// }
// fn example() {
//     let origin = Point::from((0, 0));
//     let origin = Point::from([0, 0]);
//     let origin: Point = (0, 0).into();
//     let origin: Point = [0, 0].into();
// }

// TryFrom TryInto
// trait TryFrom<T> {
//     type Error;
//     fn try_from(value: T) -> Result<Self, Self::Error>;
// }
// trait TryInto<T> {
//     type Error;
//     fn try_into(self) -> Result<T, Self::Error>;
// }

// FromStr
// trait FromStr {
//     type Err;
//     fn from_str(s: &str) -> Result<Self, Self::Err>;
// }
// use std::str::FromStr;
// fn example<T: FromStr>(s: &str) {
//     let t: Result<T, _> = FromStr::from_str(s);
//     let t = T::from_str(s);
//     let t: Result<T, _> = s.parse();
//     let t = s.parse::T();
// }

// AsRef<T>
// trait AsRef<T> {
//     fn as_ref(&self) -> &T;
// }
// fn takes_str(s: &str) {
//     // use &str
// }
// fn takes_asref_str<S: AsRef<str>>(s: S) {
//     let s: &str = s.as_ref();
// }
// fn example(slice: &str, borrow: &String, owned: String) {
//     takes_str(slice);
//     takes_str(borrow);
//     takes_str(owned);
//     takes_asref_str(slice);
//     takes_asref_str(borrow);
//     takes_asref_str(owned);
// }

fn main() {
    // let color = Color::default();
    // let color: Color = Default::default();
    // println!("{:?}", color);
    // println!("orgin: {}", Point::default());
    // let stringified = format!("{}", Point::default());
    // assert_eq!("(0, 0)", stringified);

    // assert_eq!(
    //     Point { x: 4, y: 6 },
    //     Point { x: 1, y: 2 } + Point { x: 3, y: 4 }
    // );

    // let a = Atype {
    //     num: 100,
    //     a_vec: vec![10, 20, 30],
    // };
    // let mut b = a.clone();
    // b.num = 200;
    // b.a_vec[0] = 11;
    // b.a_vec[1] = 21;
    // b.a_vec[2] = 31;
    // println!("{a:?}");
    // println!("{b:?}");

    // let range = 0..10;
    // let get_range_count = || range.count();
    // assert_eq!(get_range_count(), 10);

    // let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    // let mut min = i32::MIN;
    // let ascending = nums
    //     .into_iter()
    //     .filter(|&n| {
    //         if n <= min {
    //             false
    //         } else {
    //             min = n;
    //             true
    //         }
    //     })
    //     .collect::<Vec<_>>();
    // assert_eq!(vec![0, 4, 8, 10, 15, 18], ascending);

    // let nums = vec![0, 4, 2, 8, 10, 7, 15, 18, 13];
    // let min = 9;
    // let greater_than_9 = nums.into_iter().filter(|&n| n > min).collect::<Vec<_>>();
    // assert_eq!(vec![10, 15, 18, 13], greater_than_9);

    // fn add_one(x: i32) -> i32 {
    //     x + 1
    // }
    // let mut fn_ptr: fn(i32) -> i32 = add_one;
    // assert_eq!(fn_ptr(1), 2);
    // fn_ptr = |x| x + 1;
    // assert_eq!(fn_ptr(1), 2);
}
