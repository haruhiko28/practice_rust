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
}

// 型を表示する関数 http://pineplanter.moo.jp/non-it-salaryman/2020/01/16/rust-typeof/
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
}
