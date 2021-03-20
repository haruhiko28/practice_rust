fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // 手数はconstと全て大文字でアンダースコアで単語区切り
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINT is {}", MAX_POINTS);
}
