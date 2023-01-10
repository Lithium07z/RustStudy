/*
Topic : Rust 변수
*/
fn main() {
    // 변수의 데이터 타입이 정의되지 않으면 컴파일러가 추론하여 결정
    let a = 100; // i32타입
    let b = 100.0; // f64 타입

    // 데이터 타입 정의 방법 (i32타입의 변수 a에 10을 넣는 예제)
    let a: i32 = 10;
    let a = 10i32;
    let a = 10_i32;

    // Rust의 모든 변수는 불변성(Immutability)의 성질을 가지므로 let으로 선언한 변수는 수정이 불가능함
    // 변수 선언 후 사용하지 않으면 변수가 사용되지 않는다는 warning이 발생한다. 변수명 앞에 _를 붙이면 warning이 사라진다.
    let a = 100;
    a = a + 1; // 에러 발생
    println!("{}", a);

    // 다른 언어의 변수처럼 데이터가 변경되어야 하는 경우 변수 앞에 'mut'라는 키워드를 지정함
    // mut으로 선언한 변수가 실행 중에 값의 변경이 없는 경우 let으로 수정하라는 warning이 발생한다.
    let mut a = 100;
    a = a + 1; // 참고, Rust에는 증감연산자가 없음 ++, -- 사용불가
    println!("{}", a);

    // Rust의 상수는 다른 언어에서의 상수와 같이 불변의 값을 가지며 항상 데이터 타입을 지정해야 한다.
    const PI: f64 = 3.141592;
    let area = PI * 5.0 *5.0;
    println!("{}", area);

    // 위의 코드 중 같은 이름의 변수를 다시 선언하는 이상한(?)코드가 있다. 이는 Shadowing으로 Rust의 기능 중 하나이다.
    let a = 1; // 변수 a는 i32타입의 정수형
    println!("{}", a);

    let a = "hello"; // 변수 a는 &str타입의 문자열, 이 시점에서 이전의 a는 잊게된다.
    println!("{}", a);

    // shadowing과 scope를 사용해 변수의 값을 유지한채로 변경해서 사용할 수 있다.
    let a = 1;
    let a = 2;
    {
        let a = a + 1;
        println!("{}", a); // 출력 3
    }

    println!("{}", a); // 출력 2

    // a가 let mut으로 선언된 경우 변수의 값이 변경되어 유지된다.
    let mut a = 1;
    a = 2;
    {
        a = a + 1;
        println!("{}", a); // 출력 3
    }
    println!("{}", a); // 출력 3
}