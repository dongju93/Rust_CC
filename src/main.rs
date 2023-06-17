#![allow(non_snake_case)]

fn main() {
    // Rust의 특징인 Shadowing

    // x를 재선언하며 1을 더해 x 는 6이 됌
    let x = 5;
    let x = x + 1;

    // [warn(unused_variables)]를 회피하려면 "_"을 변수명 앞에 붙힘
    let _y = true;

    // spaces는 공백인 string 이였는데, spaces의 길이를 담은 정수로 재할당
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}");
    println!("{x}");

    // mut를 사용하여 변수를 mutable하게 선언
    let mut k = 1;
    k = k + 10;

    assert_eq!(k, 11);
    println!("Officer K!")
}
