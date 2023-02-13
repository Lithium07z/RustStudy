/*
Topic : Rust Question mark operator (?)
*/
use std::num::ParseIntError;

fn parse_str(input: &str) -> Result<i32, ParseIntError> { // Ok에 값을 담아 반환하거나 ParseIntError 반환
    // let parsed_number = input.parse::<u16>()?.to_string().parse::<u32>()?.to_string().parse::<i32>()?;
    // 모든 ?을 통과하면 parsed_number에 값이 저장됨 하나라도 ?를 통과하지 못하면 Error 반환
    let parsed_number = input.parse::<i32>()?; // 항상 Result와 같이 사용함
    // input을 i32타입으로 변환할 수 있으면 변환해서 담고 아니면 return Err
    Ok(parsed_number)
}

fn main() {
    for item in vec!["Seven", "8", "9.0", "nice", "6060"] {
        let parsed = parse_str(item);
        println!("{:?}", parsed);
    }
}