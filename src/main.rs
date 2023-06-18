#![allow(non_snake_case)]

use std::fmt::format;

fn main() {
    // test1();
    // test2();
    // test3();
    // test4();
    // test5();
    // test6();
    test7();
}

fn test1() {
    // Rust의 특징인 Shadowing
    // #### 1 ####
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
    // #### 2 ####
    // mut를 사용하여 변수를 mutable하게 선언
    let mut k = 1;
    k = k + 10;
    // compile이 정상적으로 수행되어 아래 문자열 출력
    assert_eq!(k, 11);
    println!("Officer K!");
    // #### 3, 4 ####
    // main 함수에 test 함수 불러와 사용
}

fn test2() {
    // #### 3 ####
    let i = 10;
    let t = 5;
    {
        // scope 안에서 선언된 변수 t를 out of scope하여 block of code 밖에서도 t를 호출 할 수 있음
        // let t: i32 = 5;
        println!(
            "The value of I in the SCOPE is {} and value of T is {}",
            i, t
        );
    }
    println!(
        "The value of I in the out of SCOPE is {} and value of T is {}",
        i, t
    );

    // #### 4 ####
    let j = "hello!";
    println!("{}, 4th world!", j);
}

fn test3() {
    // Shadowing
    let x = 5;
    {
        // x 는 block of code에서 처음 선언되어 값이 12로 init됨
        let x = 12;
        // assert_eq!(x, 5);
        assert_eq!(x, 12);
    }
    // assert_eq!(x, 12);
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x);
}

fn test4() {
    let mut x = 1;
    x = 7;
    // mutatable한 x를 재선언(let)하려면 mut를 다시 써야함
    // let mut x: i32 = x;
    x = x + 3;

    let y = 4;
    // immutatable한 변수는 재선언(Shadowing)하여 y 값을 재할당
    let y = "I can also be bound to text!";

    println!(
        "Successfully compiled! mutate X value is {}, and shadowing Y value is {}",
        x, y
    )
}

fn test5() {
    // 패턴화된 Tuple의 변수 값을 변경하기
    // RUST는 모두 immutable이기 때문에 x 값을 변경하기 위해서 mut 선언
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Compile success!")
}

fn test6() {
    let (x, y);
    // x만 첫번째 위치에 선언되었기 때문에 x = 3
    (x, ..) = (3, 4);
    // y만 두번째 위치에 선언되었기 때문에 y = 2
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);

    println!("Compile success!")
}

fn test7() {
    // #### Num type test 1 ####
    // u8 Type의 38이라는 숫자를 u16 타입의 v에 할당하기 위해서는 "as u16" 으로 Type 변환을 수행
    let _v: u16 = 38_u8 as u16;

    // #### Num type test 2 ####
    let x: u32 = 5;
    // Function "type_of()" calls
    assert_eq!("u32".to_string(), type_of(&x));
    println!("{}", type_of(&x));

    // #### Num type test 3 ####
    // v1의 type을 지정하지 않으면, type을 유추하여 변수 type 지정
    // 251과 8의 type이 u8이기때문에 v1의 type은 u8
    // u8의 MAX value는 255 이기때문에 259를 담을 수 없음, u16, i16으로 type 변경
    let v1: u16 = 251_u16 + 8_u16;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}", v1, v2);

    // #### Num type test 4 ####
    // 다양한 숫자 byte type에 대한 계산 가능
    let g = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(g == 1597)
}

// Placeholder <T> 사용
// #### Num type test 2 ####
fn type_of<T>(_: &T) -> String {
    // test7() 에서 type_of(x), x를 argument로 지정했으므로 u32 예상
    format!("{}", std::any::type_name::<T>())
}
