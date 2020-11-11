// use std::io::Write; // write, writelnマクロを使うため
use std::thread;

fn main() {
    println!("Hello, world!");
    // 文字列 convert str and String
    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;              // String --> &str
    let s3: String = s2.to_string(); // &str --> String
    println!("{}", type_of(&s1));
    println!("{}", type_of(&s2));
    println!("{}", type_of(&s3));
    
    // タプル
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{:?}",t);   

    // 配列
    let mut a: [i32; 3] = [0,1,2];
    let b: [i32; 4] =[0; 4];
    a[1] = b[1];     
    a[2] = b[2];
    println!("{:?}",&a[0..3]);

    // ユーザ定義型
    #[derive(Debug)] //構造体をPrintlnする https://qiita.com/Kashiwara/items/9cec0d4940a8c92e85a5
    struct Person{
        name: String,
        age: u32,
    };
    let p = Person{
        name: String::from("John"),
        age: 8,
    };
    println!("{:?}", p);

    // 列挙型
    #[derive(Debug)]
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown {x: i32, y:i32},
    }
    let e1 = Event::Quit;
    let e2 = Event::KeyDown(1);
    let e3 = Event::MouseDown{x: 10,y: 10};

    println!("{:?}", e1);
    println!("{:?}", e2);
    println!("{:?}", e3);

    // 標準ライブラリの型
    // pub enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let result: Result<i32, String> = Ok(200);
    match result {
        Ok(code) => println!("code {}", code),
        Err(err) => println!("Err {}", err),
    };
    
    let result: Result<i32, String> = Ok(200);
    if let Ok(code) = result {
        println!("code {}", code);
    }
    println!("code {}", result.unwrap_or(-1));
    
    let result: Result<i32, String> = Err("error".to_string());
    println!("code {}", result.unwrap_or(-1));

    fn func(code: i32) -> Result<i32, String>{
        println!("code: {}", code);
        Ok(100)
    }

    let result: Result<i32, String> = Ok(200);
    let _next_result = result.and_then(func); //実行される
    
    let result: Result<i32, String> = Err("error".to_string());
    let _next_result = result.and_then(func); //実行されない

    // fn error_handle(result: Result<i32,String>) -> Result<i32,String>{
    //     let code = result?;
    //     println!("code: {}",code);
    //     Ok(100)
    // }

    // Vec
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![0; 5];
    println!("{}", v1[0]);
    println!("{}", v2[0]);

    let v = vec![1,2,3,4,5];
    for element in &v {
        println!("{}", element);
    }
    
    // Box
    let byte_array = [b'h',b'h',b'h',b'h',b'h'];
    print(Box::new(byte_array));

    // letとmut
    let immut_val = 10;
    let mut mut_val = 20;

    mut_val += immut_val;
    println!("{}", mut_val);

    let v1: u64 = 10;
    let v2 = 10u64;
    println!("{}", v1);
    println!("{}", v2);

    //if
    let number = 1;
    if 0 < number {
        println!("0 < number");
    }else if number < 0{
        println!("number < 0");
    }else{
        println!("0==number");
    }

    let number = 1;
    let result = if 0 <= number{
        number
    }else{
        -number
    };
    println!("{}",result);

    // ループ
    let mut count = 0;
    let result = loop {
        println!("conut: {}",count);
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("{}",result);

    let mut count = 0;
    while count < 10{
        println!("conut: {}",count);
        count += 1;
    }

    for count in 0..10{
        println!("conut: {}",count);
    }

    let array = [0,1,2,3,4,5,6,7,8,9];

    for element in &array{
        println!("element {}",element);
    }

    'main: loop{
        println!("main loop start");
        // 'sub: loop{ //使ってないラベルなので、これはコメントアウト
            println!("sub loop start");

            break 'main;
            // println!("sub loop end");
        // }
        // println!("main loop end");
    }

    // match
    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("_"),
    }

    enum Color {
        Red,
        Bule,
        Green,
    }

    let c1 = Color::Red;
    let c2 = Color::Bule;
    let c3 = Color::Green;
    match c1 {
        Color::Red => println!("Red"),
        Color::Bule => println!("Bule"),
        Color::Green => println!("Green"),
    }
    match c2 {
        Color::Red => println!("Red"),
        Color::Bule => println!("Bule"),
        Color::Green => println!("Green"),
    }
    match c3 {
        Color::Red => println!("Red"),
        Color::Bule => println!("Bule"),
        Color::Green => println!("Green"),
    }

    let result : Result<i32, String> = Ok(100);
    let result_number = match result{
        Ok(number) => number,
        Err(message) => {
            println!("Error {}",message);
            -1
        },
    };
    println!("{}",result_number);

    // range
    for number in 1..5{
        println!("{}", number);
    }

    // iterator
    let it = Iter{
        current: 0,
        max: 10,
    };

    // Iterトレイト
    for num in it{
        println!("{}",num);
    }

    // 関数
    let x = add(1,2);
    println!("x = {}",x);

    println!("abs {}",abs(-110));

    //impl
    let p = Person2{
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name();
    p.say_age();

    let p = Person3{
        name: String::from("Taro"),
        age: 20,
    };
    p.say_name().say_age();

    let p = Person3::new("Taro", 20);
    p.say_name().say_age();

    //マクロ
    let s = concat!("A","B","C");
    println!("{}",s);
    let s = format!("{}--{:?}",s,("D",5));
    println!("{}",s);
    let s = format!("{}-{}","abc","cdf");
    println!("{}",s);

    print!("hello");
    println!("hello {}","world");
    print!("hello {}","error");
    eprint!("hello");

    // let mut w = Vec::new();
    // write!(&mut w, "{}", "ABC");
    // writeln!(&mut w, " is 123");
    // dbg!(w);

    // panic!("it will panic");

    // let v = vec![1,2,3];

    println!("defined in file: {}",file!());
    println!("defined on line: {}",line!());
    println!("is test: {}",cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    // アサーション用のマクロ
    // assert!(true);
    // assert_eq!(1,1);
    // assert_ne!(1,0);
    // debug_assert!(false);
    // debug_assert_eq!(1,1);
    // debug_assert_ne!(1,0);

    // 実装補助用のマクロ
    // let mut p = HappyPerson{
    //     name: "Takashi".to_string(),
    //     state: Emotion::Happy,
    // };

    // println!("{}", p.get_happy());

    //トレイトの導出
    println!("{:?}", A(0)==A(1));

    println!("{:?}", B(1.0) > B(0.0));

    let c0 = C;
    let _c1 = c0;
    let _c2 = c0;

    let d0 = D;
    let _d1 = d0.clone();

    println!("{:?}",E);

    let _f = F::default();
    
    // traitとdyn
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice();
    dove.shout();

    let duck = Duck {};

    let bird_vec: Vec<Box<dyn Tweet>> = vec![Box::new(dove),Box::new(duck)];
    for bird in bird_vec{
        bird.tweet();
    }

    // ジェネリクス
    let t1 = make_tuple(1, 2);
    let t2 = make_tuple("Hello", "World");
    let t3 = make_tuple(vec![1,2,3], vec![4,5]);
    let t4 = make_tuple(3, "years old");
    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t3);
    println!("{:?}", t4);

    // ムーブセマンティック
    let a = Color2{r:255, g:255, b:255};
    let b = a;
    println!("{} {} {}",b.r, b.g, b.b);

    // 借用
    // let mut important_data = "Hello, World!".to_string();
    // important_data = calc_data(important_data);
    // println!("{}", important_data);

    let important_data = "Hello, World!".to_string();

    calc_data(&important_data);

    println!("{}",&important_data);

    // 不変な参照
    let x = 5;
    let y = &x;
    let z = &x;

    dbg!(x);
    dbg!(y);
    dbg!(z);

    let mut x = 5;
    let y = &mut x;
    // let z = &mut x;

    dbg!(y);
    // dbg!(z);

    let y = &x;
    // let z = &mut x;

    dbg!(y);
    // dbg!(z);

    // 借用の範囲の話めっちゃおもろいな
    let mut x = 5;

    // let y = &x;

    let z = &mut x;

    dbg!(z);

    dbg!(x);

    // {
    //     let d = Droppable;
    // }
    println!("The Droppable should be released at the end of block.");

    //スレッドを作る 不確定性がある
    let mut handles = Vec::new();
    let mut data = vec![1;10];

    for x in 0..10{
        handles.push(thread::spawn(move ||{
            println!("Hello, world! {}",x);
        }))
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);

}
// ======================================================================================================================================================
// RALL
// struct Droppable;

// impl Drop for Droppable{
//     fn drop(&mut self) {
//         println!("Resource will be released!");
//     }
// }

// 借用
fn calc_data(data: &String){
    println!("{}", data);
}

struct Color2 {
    r: i32,
    g: i32,
    b: i32,
}

// ジェネリクス
fn make_tuple<T,S>(t: T, s: S) -> (T,S){
    (t,s)
}

// traitとdyn
trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self){
        self.tweet();
        self.tweet();
    }

    fn shout(&self){
        println!("Uoooooohhhh!!");
    }
}

struct Dove;
struct Duck;


impl Tweet for Dove {
    fn tweet(&self){
        println!("Coo!");
    }
}

impl Tweet for Duck{
    fn tweet(&self){
        println!("Quack");
    }
}
// トレイトの導出
#[derive(Eq, PartialEq)]
struct A(i32);

#[derive(PartialEq, PartialOrd)]
struct B(f32);

#[derive(Copy, Clone)]
struct C;

#[derive(Clone)]
struct D;

#[derive(Debug)]
struct E;

#[derive(Default)]
struct F;


// unreachableの関数の書き方
// fn f(x: usize) -> &'static str{
//     match x {
//         n if n * n % 3 == 0 => "3n",
//         n if n * n % 3 == 1 => "3n+1 or 3n+2",
//         _ => unreachable!(),
//     }
// }

// 実装補助用のマクロ
// enum Emotion{
//     // Anger,
//     Happy,
// }

// trait Emotional{
//     fn get_happy(&mut self) -> String;
//     fn get_anger(&mut self) -> String;
//     fn tell_state(&self) -> String;
// }

// struct HappyPerson{
//     name: String,
//     state: Emotion,
// }

// impl Emotional for HappyPerson{
//     fn get_anger(&mut self) -> String {
//         unimplemented!()
//     }
//     fn get_happy(&mut self) -> String {
//         format!("{} is always happy.",self.name)
//     }
//     fn tell_state(&self) -> String {
//         todo!()
//     }
// }
//impl
struct Person3{
    name: String,
    age: u32,
}

impl Person3 {
    fn new(name: &str, age: u32) -> Person3 {
        Person3 {
            name: String::from(name),
            age: age,
        }
    }
    fn say_name(&self) -> &Self{
        println!("I am {}.",self.name);
        self
    }
    fn say_age(&self)-> &Self {
        print!("I am {} years old.",self.age);
        self
    }
}


struct Person2{
    name: String,
    age: u32,
}

impl Person2{
    fn say_name(&self){
        println!("I am {}.",self.name);
    }

    fn say_age(&self){
        print!("I am {} years old.",self.age);
    }
}

// 型を表示する関数 http://pineplanter.moo.jp/non-it-salaryman/2020/01/16/rust-typeof/
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}

fn print(s: Box<[u8]>){
    println!("{:?}",s);
}
// Iterトレイト
struct Iter {
    current: usize,
    max: usize,
}

impl Iterator for Iter{
    type Item = usize;

    fn next(&mut self) -> Option<usize>{
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        }else{
            None
        }
    }
}

// 関数
fn add(a: i32, b: i32) -> i32{
    a + b
}

fn abs(number: i32) -> i32{
    if number < 0{
        return -number;
    }
    number
}
