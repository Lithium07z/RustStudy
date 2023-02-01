/*
Topic : Rust 함수
*/

fn test_function(one: i32, two: i32) -> i32 { // 파라미터 이름: 데이터 타입으로 매개변수를 받음, -> 데이터 타입으로 반환타입 지정
    /*
    1. 함수는 "fn" 키워드로 정의한다.
    2. 함수명과 변수명은 소문자의 snake case를 사용한다. (다르게 사용할 수 있지만 warning 발생)
    3. 함수를 외부에서 사용할 수 있도록 pun fn와 같이 정의할 수 있다.
    4. 피호출 함수는 호출함수 뒤에 와도 상관 없다. 함수 위치 상관 X
    5. return문 생략시 empty tuple을 반환한다.
    */
    dbg!(one, two); // 디버그 매크로로 매개변수로 들어온 인자 상태를 출력
    return one + two; // return을 사용해 반환할 수 있고 (statement)
    one + two // ;없이 함수의 가장 마지막줄에 배치하는 것으로 반환할 수 있다. (expression)
    // 함수의 중간에 값을 리턴해야 하는 경우는 return문을 하고, 마지막에서 값을 리턴할 때는 보통 (return문 대신) Expression을 사용한다.
}

fn string_function_1(value1: String, value2: String) -> String{
    format!("{}{}", value1, value2)
}

fn string_function_2(value1: &String, value2: &String) -> String{
    format!("{}{}", value1, value2)
}

fn char_function(value: char) -> char {
    value
}

fn main() {
    let a: String = "Hello".to_owned();
    let b: String = "World".to_owned();
    println!("{}", test_function(1, 2));
 
    // 함수에 값을 전달할 때는 항상 &로 전달해야 함! (Scalar타입 제외), 소유권 (OwnerShip) 문제 때문
    let c = string_function_1(a, b);
    println!("{}", a); // 에러 발생, string_function에 값을 전달할 때 &로 주지 않았으므로 소유권 자체가 이전되어 사용 불가능
    println!("{}", b); // 에러 발생, string_function에 값을 전달할 때 &로 주지 않았으므로 소유권 자체가 이전되어 사용 불가능

    let a: String = "Hello".to_owned();
    let b: String = "World".to_owned();
    let c = string_function_2(&a, &b);

    println!("{}", a); // 정상 작동, string_function에 &로 값을 전달했기 때문에 소유권 이전이 발생하지 않음
    println!("{}", b); // 정상 작동, string_function에 &로 값을 전달했기 때문에 소유권 이전이 발생하지 않음
    println!("{}", c);

    let a = 'A';
    let c = char_function(a);
    println!("{}", c);
}
