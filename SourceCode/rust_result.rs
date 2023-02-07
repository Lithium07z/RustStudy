/*
Topic : Rust Result
*/
/*  Option과 Result의 차이
    Option - Maybe there, Maybe not, 값의 유무
    Result - May not work, 동작 여부
*/
enum Option<T> {
    None,    // .is_none()
    Some(T), // .is_some()
}

/*  Result 타입은 정상적으로 수행되었을 T 타입의 값을 리턴하고, 에러가 났을 때 E 타입의 값을 리턴한다.
		Result 타입은 자주 사용되기 때문에 별도로 std::io::Result을 include 하지 않아도 (즉, use를 사용하지 않아도) 
		직접 사용할 수 있다. 아래는 Result 열거형 타입의 정의인데, 기본적으로 정상 실행된 경우 Ok(T)를, 에러인 경우 Err(E)를 가진다.
*/

enum Result<T, E> {
    Ok(T),  // .is_ok()
    Err(E), // .is_err()
}

// Case 1, 정상 동작, Result와 if 사용
fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    if check_error(6).is_ok() {
        println!("It's okay, guys!");
    } else {
        println!("It's an error, guys!");
    }
}   // It's okay, guys!

// Case 2, 정상 동작, Result와 match 사용
fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    match check_error(5) {
        Ok(_) => println!("Okay guys"),
        Err(_) => println!("It's an error")
    }
}

// Case 3, Option과 마찬가지로 Err타입은 unwrap 불가능
fn check_error(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    check_error(5).unwrap()
    // thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: ()'
}

// Case 4, match와 같이 사용하기
fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err("Sorry, the number wasn't five.".to_owned())
    }
}

fn main() {
    let mut result_vec = Vec::new(); // Vec<Result<i32, String>>

    for number in 2..=7 {
        result_vec.push(check_if_five(number))
    }

    println!("{:#?}", result_vec);
}

// 번외, parse()를 사용해 문자열 > 숫자 변환, parse()로 변환 불가능한 경우 이미 존재하는 PaeseIntError를 반환하게 함 
fn parse_number(number: &str) -> Result<i32, std::num::ParseIntError> {
    number.parse()
}

fn main() {
    let mut result_vec = vec![];
    result_vec.push(parse_number("8"));
    result_vec.push(parse_number("abcdefg"));
    result_vec.push(parse_number("8"));

    for number in result_vec {
        println!("{:?}", number);
    }
}
