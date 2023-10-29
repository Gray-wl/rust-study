fn main() {
    // 借用与引用
    // fn foo(s: String) -> String {
    //     println!("{s}");
    //     s
    // }
    // let s1 = String::from("I am a superman.");
    // let s1 = foo(s1);
    // println!("{s1}");

    // let a: u32 = 10;
    // let b = &a;
    // let c = &&&&a;
    // let d = &b;
    // let e = b;

    // println!("{a}");
    // println!("{b}");
    // println!("{c}");
    // println!("{d}");
    // println!("{e}");

    // let s1 = String::from("I am a superman.");
    // let s2 = &s1;
    // let s3 = &&&&s1;
    // let s4 = &s2;
    // let s5 = s2;

    // println!("{s1}");
    // println!("{s2}");
    // println!("{s3}");
    // println!("{s4}");
    // println!("{s5}");

    // 不可变引用、可变引用
    // let mut a = 10;
    // let b = &mut a;
    // *b = 20;
    // println!("{b}");
    // println!("{a}");

    // let mut a = 10;
    // let b = &mut a;
    // *b = 20;
    // let c = &a;
    // println!("{c}");

    // let mut a = 10;
    // let c = &a; // immutable borrow occurs here
    // let b = &mut a;
    // *b = 20;
    // println!("{c}");

    // let mut a = 10;
    // let b = &mut a;
    // *b = 20;
    // let d = &mut a;
    // println!("{d}");

    // let mut a = 10;
    // let r1 = &a;
    // a = 20; // `a` is assigned to here but it was already borrowed
    // println!("{r1}");

    // let mut a = 10;
    // let r1 = &mut a;
    // a = 20; // `a` is assigned to here but it was already borrowed
    // println!("{r1}");

    // let mut a = 10;
    // let r1 = &mut a;
    // let r2 = r1;
    // println!("{r2}");

    // 多级引用
    // let mut a1 = 10;
    // let mut b = &mut a1;
    // *b = 20;
    // let c = &mut b;
    // **c = 30;
    // println!("{c}");

    // let mut a1 = 10;
    // let b = &mut a1;
    // let mut c = b;
    // let d = &mut c;
    // **d = 30;
    // println!("{d}");

    // 用引用改进函数的定义
    // fn foo(s: &String) {
    //     println!("in fn foo: {s}");
    // }
    // let s1 = String::from("I am a superman.");
    // foo(&s1);
    // println!("{s1}");

    // fn foo(s: &mut String) {
    //     s.push_str(" You are batman.");
    // }
    // let mut s1 = String::from("I am a superman.");
    // println!("{s1}");
    // foo(&mut s1);
    // println!("{s1}");

    
}
