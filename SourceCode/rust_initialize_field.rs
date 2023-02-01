/*
Topic : Rust 변수 초기화 
*/
fn main() {
    /* rust는 NULL이 없기에 초기화 되지않은 변수를 선언할 수 없다.
       하지만 러스트는 컴파일 타임에 알 수 있기에 조금 다른 방식으로 사용할 수 있다.
       1. 변수 선언시 타입 지정.
       2. 그리고 이 변수는 반드시 초기화 한다.
    */
    let my_number; // 에러 발생, 선언과 동시에 초기화 해야함
    let my_number: u8; // 임시 선언(타입 지정), 일단 에러가 발생하진 않음 
    
    println!("{}", my_number); // 에러 발생, 초기화되지 않았으므로 사용 불가능

    my_number = 10;
    println!("{}", my_number); // 정상 동작

    let tt:String; // Scope를 사용한 선언 및 초기화
    {
        tt = String::from("Hello rust!");
    }
    println!("{tt}");
}