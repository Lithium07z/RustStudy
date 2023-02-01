/*
Topic : Rust 소유권 (ownership)
*/
/*  기본적으로 함수 호출시 인자들은 소유권 전달이다.
    따라서 다음 호출시 소유권이 이전되어 사용하지 못한다.
    이러한 문제는 call by reference, 복사, 재리턴 등의 방법들이 있다.
*/

// Case 1 (Call by value), 에러 발생
fn print_country(country_name: String) {
    println!("My country is {}", country_name);
}

fn main() {
    let country = "대한민국".to_owned();
    print_country(country); // 정상 동작
    print_country(country); // 에러 발생
    /*  line6번에서 country를 call by value로 넘겨줌으로써 소유권의 이전이 발생함
        이 때문에 line6의 호출 이후에 main의 country는 소유권이 없으므로 사용이 불가능
    */
}

// Case 2 (return value), 정상 동작
fn print_country(country_name: String) -> String {
    println!("My country is {}", country_name);
    country_name
}

fn main() {
    let mut country = "대한민국".to_owned();
    country = print_country(country); 
    country = print_country(country); 
    country = print_country(country); 
}   // 이렇게 사용할 수도 있겠지만 Rust스럽게 사용하는 것은 아님

// Case 3 (clone), 정상 동작
fn print_country(country_name: String) {
    println!("My country is {}", country_name);
}

fn main() {
    let country = "대한민국".to_owned();
    print_country(country.clone()); 
    print_country(country.clone()); 
    print_country(country.clone()); 
}   // 이렇게 사용할 수도 있겠지만 Rust스럽게 사용하는 것은 아님

// Case 4 (Call by reference), 정상 동작
fn print_country(country_name: &String) {
    println!("My country is {}", country_name);
}

fn main() {
    let country = "대한민국".to_owned();
    print_country(&country); // call by reference로 소유권의 이전이 없음
    print_country(&country); 
    print_country(&country); 
}   // 소유권의 이전이 없으므로 모두 정상적으로 동작함, 가장 바람직한 방법

// 함수 매개변수의 소유권 이전 1
fn add_is_great(mut country_name: String) { // country의 소유권이 이전되면서 mutable로 선언됨
    country_name.push_str(" is great"); // 값의 변경이 가능해짐
    println!("Now it says: {}", country_name); // 정상 동작
}

fn main() {
    let country = "대한민국".to_owned(); // immutable let
    add_is_great(country); // call by value로 인해 소유권이 이전됨
}

// 함수 매개변수의 소유권 이전 2
fn add_is_great(country_name: &mut String) {
    country_name.push_str(" is great");
    println!("Now it says: {}", country_name);
}

fn main() {
    let mut country = "대한민국".to_owned(); // mutable String
    add_is_great(&mut country); // mutable reference로 전달, Now it says: 대한민국 is great
    add_is_great(&mut country); // Now it says: 대한민국 is great is great
}   // mutable reference이므로 변경된 값이 누적됨, 소유권 이전X