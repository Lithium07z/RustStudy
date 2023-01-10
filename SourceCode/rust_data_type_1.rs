/*
Topic : Rust 타입(Scalar)
*/
fn main() {
    //  정수형 
    /*  8 bit : i8, u8
        16 bit : i16, u16
        32 bit : i32, u32
        64 bit : i64, u64
        128 bit : i128, u128
        아키텍쳐 크기에 따른 isize, usize */

    let a: i32 = 1_000_000; // 10진수, 컴파일러는 _를 무시함
    let a = 0xff;           // 16진수
    let a = 0o15;           // 8진수
    let a = 0b1111_1111;    // 2진수
    let a: u8 = b'A';       // 바이트 (앞에 b를 붙임)

    // 부동소수점형
    // f32와 f64 두 가지 종류가 존재, 명시적으로 타입을 지정하지 않으면 f64로 지정됨

    let a: f32 = 3.14;
    let a: f64 = 3.14;
    let a = 9.; // f64타입의 변수 a에 9.0이 초기화됨 

    // 부울린형

    let a: bool = true;

    // 문자형
    // Rust에서 char형은 4bytes의 크기를 갖는다.

    let a: char = 'A'; 
    let first_letter = 'A';  
    let space = ' ';               // 공백도 char형이다.
    let other_language_char = 'Ꮔ'; // 다양한 언어를 표현할 수 있다.
    let cat_face = '😺';           // 이모지도 char형으로 표현 가능
    // Rust의 char형은 4bytes이기 때문에 유니코드상의 대부분의 문자를 표현할 수 있다.

    let a = 'A';
	print!("{}", a as u8); // 이 경우 65가 출력된다.

	let a = 65;
	print!("{}", a as char); // 하지만 역은 성립하지 않는다.

    // std::mem::size_of::<데이터타입>() 코드를 사용해 데이터 타입이 몇 bytes인지 알 수 있다.
    println!("Size of a char: {}", std::mem::size_of::<char>()); // 4 bytes
	println!("Size of a char: {}", std::mem::size_of::<u128>()); // 16 bytes

    //  서로 다른 크기의 변수간의 연산시 주의할 점 
    /*  서로 다른 크기로 지정된 변수끼리의 연산은 불가능하다.
        단, 두 변수 중 하나의 크기로 두 변수의 크기를 같도록 만들어준다면 연산이 가능하다.
        또는 두 변수 중 하나가 크기 지정이 되어있지 않다면
        컴파일러가 추론하여 두 변수의 크기를 같도록 만들어주어 연산이 가능하다. */ 

    let a: u8 = 10;
    let b: u32 = 20;
    let c = a + b; // 에러 발생
    let c = a + b as u8;  // 연산 가능
    let c = a as u32 + b; // 연산 가능

    let a: u8 = 10;
    let b = 20;
    let c = a + b; // 컴파일러가 자동으로 변수 b의 크기를 u8로 지정하여 연산 가능
    // as를 사용해 크기를 바꾸는 것은 as가 사용되는 시점에 한해 잠시 바꾸는 것일 뿐 본래 크기는 변하지 않음
}