/*
Topic : Rust Option 열거형 타입
*/
fn main() {
    /*  Rust의 표준 라이브러리에는 다음과 같은 Option 열거형 타입이 정의되어 있다. 
        Rust는 다른 프로그래밍 언어와 달리 null 타입을 지원하지 않고 있는데, 
        대신 아래와 같은 Option 타입을 활용하여 value가 있는지 없는지를 컴파일 타임에 체크하고 있다. 
        Rust에서 어떤 변수가 Option 타입이 아니라면 그 변수는 항상 값을 갖는다고 볼 수 있고, 
        만약 (null과 같이) 값을 갖지 않을 수 있다면 Option을 사용하여 이를 핸들링해 주어야 한다.
    */
    enum Option<T> {
        None,
        Some(T),
    }

    /*  Option 열거형은 데이타가 없다는 것을 의미하는 None과 어떤 데이타가 있다는 것을 의미하는 Some으로 나뉘어 지고, 
        Some에 들어갈 데이타 타입을 제네릭(generci)타입 T로 지정하고 있다.
        Option 열거형을 사용하기 위해서는 다른 열거형과 마찬가지로 Option::None 혹은 Option::Some(값)를 사용하면 된다. 
        다만, Option 열거형이 매우 널리 사용되기 때문에 Option:: 을 생략하고 None 혹은 Some(값) 등으로 간략히 표현할 수 있다.
    */

    // Option::None 지정
    let no_index: Option<i32> = Option::None;
    // Option::Some 지정 
    let index: Option<i32> = Option::Some(1);
     
    // Option:: 생략 표현
    let no_index: Option<i32> = None;
    let index = Some(1);
 
    println!("{:?}, {:?}", no_index, index);
    // enum 열거형의 각 variant별로 어떤 처리를 수행하기 위해 일반적으로 match expression을 자주 사용한다.
}