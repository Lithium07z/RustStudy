/*
Topic : Rust 열거형 enum
*/

/*  다른 언어의 enum과 다르게 상수가 아닌 구조체도 선언 가능함
    보통 enum은 0, 1, 2... 처럼 상수로 취급하지만 C++의 enum class처럼 상수 취급을 하지 않는다.
*/

enum ThingsInTheSky { // 자동으로 인덱스가 붙음
    Sun = 0, // 0 
    Stars,   // 1, enum에서 숫자를 지정하지 않으면 앞선 숫자에 이어서 순번이 부여됨
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun"),
        ThingsInTheSky::Stars => println!("I can see the stars")
    }
}

fn main() {
    check_skystate(&create_skystate(20));
	// I can see the stars
}

enum Mood {
    Happy,
    Sleepy,
    NotBad,
    Angry
}

fn match_mood(mood: &Mood) -> i32 {
    use Mood::*; // Mood:: 없이 필드 사용
    match mood {
        Happy => 10,
        Sleepy => 6,
        NotBad => 7,
        Angry => 2
    }   // enum 필드의 모든 경우의 수를 match에 작성했으므로 _ 사용안해도 됨
}

fn main() {
    let my_mood = Mood::NotBad;
    let happiness_level = match_mood(&my_mood);
    println!("Out of 1 to 10, my happiness is {}", happiness_level);
    // Out of 1 to 10, my happiness is 7
}

// enum을 vector로 만들고 출력하기
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter
}

fn main() {
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
        println!("The number is: {}", season as u32); // as u32 사용해야 출력됨
    }
	// The number is: 0
	// The number is: 1
	// The number is: 2
	// The number is: 3
}

// enum 필드에 타입을 직접 지정하고 데이터를 저장할 수 있다.
enum Number {
    U32(u32),
    I32(i32)
}

fn get_number(input: i32) -> Number {
    match input.is_positive() {
        true => Number::U32(input as u32),
        false => Number::I32(input)
    }
}

fn main() {
    let my_vec = vec![get_number(-800), get_number(8)];

    for item in my_vec {
        match item {
            Number::U32(number) => println!("It's a u32 with the value"),
            Number::I32(number) => println!("It's an i32 with the value")
        }
    }
	// It's an i32 with the value
	// It's a u32 with the value
}

// enum 타입에 데이터 파라미터 직접 사용
#[derive(Debug)]
enum LogType {
    Info(String),      //(infoMsg)
    Warning(String),   //(warningMsg)
    Error(i32, String) //(errorCode, errorMsg)
}
 
fn main() {
    let log = LogType::Error(1201, String::from("Not found"));
    println!("{:?}", log);
}

// enum 파라미터를 구조체로 정의해 사용
#[derive(Debug)]
enum LogType {
    None,
    Info(String),     
    Warning(String), 
    Error {code: i32, msg: String, caller: String } //구조체로 정의
}
 
fn main() {
    // 구조체값 설정
    let log = LogType::Error {
        code: 1201, 
        msg: String::from("Not found"), 
        caller: String::from("main")
    };
     
    println!("{:#?}", log);
}