fn main() {
    // 变量与可变性
    // let x = 5;
    // println!("The value of x is {x}");
    // x = 6; // cannot assign twice to immutable variable
    // println!("The value of x is {x}");

    // let x = 5;
    // println!("The value of x is {x}");
    // let x = 6;
    // println!("The value of x is {x}");

    // let a = 10;
    // let a = 'a';
    // println!("{}", a);

    // let mut x = 10;
    // println!("The value of x is: {x}");
    // x = 100;
    // println!("The value of x is: {x}");

    // 变量的类型
    // let a: u8 = 323232; // the literal `323232` does not fit into the type `u8` whose range is `0..=255`
    // println!("{a}");

    // let a = 10;
    // let b = a;
    // println!("{a}");
    // println!("{b}");

    // let s1 = String::from("I am a superman.");
    // let s2 = s1;
    // println!("{s1}"); // value borrowed here after move
    // println!("{s2}");

    // let s1 = String::from("I am a superman.");
    // let s2 = s1.clone();
    // println!("{s1}");
    // println!("{s2}");

    // 所有权
    // let s1 = String::from("I am a superman.");
    // let s2 = s1;
    // // println!("{s1}");
    // println!("{s2}");

    // let a = 10;
    // let b = a;
    // println!("{a}");
    // println!("{b}");

    // fn foo(s: String) {
    //     println!("{s}");
    // }
    // let s1 = String::from("I am a superman.");
    // foo(s1);

    // fn foo(s: String) {
    //     println!("{s}");
    // }
    // let s1 = String::from("I am a superman.");
    // foo(s1);
    // println!("{s1}"); // this parameter takes ownership of the value

    // fn foo(s: String) {
    //     println!("{s}");
    // }
    // let s1 = String::from("I am a superman.");
    // foo(s1);
    // foo(s1); // this parameter takes ownership of the value

    // fn foo(s: String) -> String {
    //     println!("{s}");
    //     s
    // }
    // let s1 = String::from("I am a superman.");
    // let s1= foo(s1);
    // println!("{s1}");

    // 思考题
    // let  s = "I am a superman.".to_string();

    // for i in 1..10 {
    //     let tmp_s = s.clone();
    //     println!("s{i} is {tmp_s}");
    // }
}
