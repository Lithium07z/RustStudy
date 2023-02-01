/*
Topic : Rust match
*/
fn main() {
    /*  rust는 OOP의 if도 지원하지만 이보다 더 강력한 match가 있다.
        match 입력되는 값의 모든 경우 조건 분기를 검사한다.
        리턴이 있다면 반드시 모든 분기의 리턴은 같은 타입이어야 한다.
    */
    // 일반적인 조건문
    let number = 5;
    if number == 7 { // if (number == 7)처럼 괄호를 붙여도 되지만 warning발생
        println!("It's seven");
    }

    let score = 'B';
    let ok = if score <= 'C' { 
        println!("{}", score);  // Statement
        "Pass"         // Expression
    } 
    else { 
        "Fail"         // Expression
    };
     
    println!("{}", ok); // 출력 B, Pass

    // Match 
    // 에러 발생
    let number = 5;
    
    match number {
        0 => println!("It's a zero"), // 각 패턴별 실행코드를 지정하는 것을 갈래(arm)이라 부름 arm1
        1 => println!("It's a one"),  // arm2
        // 에러 발생, match에 들어오는 모든 경우의 수에 대해 처리가 되있어야 함
    }

    // 정상 동작
    let number = 5;
    
    match number {
        0 => println!("It's a zero"),
        1 => println!("It's a one"),
        _ => println!("It's a different number") // 정상 동작, 0과 1을 제외한 모든 경우를 _에서 처리함
    }

    // _(언더바) 활용
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
        ("cloudy", _) => println!("Cloudy and something else"), // cloudy만 맞으면 두번째 인자는 상관 X, 출력됨
        _ => println!("Not sure what the weather is."),
    }

    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
        _ => println!("Not sure what the weather is."), // match에 들어오자마자 나가게 됨
        ("cloudy", "cold") => println!("It's not very nice today"),
        ("clear", "warm") => println!("It's a nice day"),
    }

    // 인자 이름 지정
    let children = 5;
    let married = true;

    match (children, married) {
        (children, married) if married == false => println!("Not married with {} children", children),
        (c, m) if c == 0 && m => println!("Married but with no children"),
		// 인자의 이름을 지정해서 사용할 수 있음 
        _ => println!("Some other type of marriage and children")
    }

    // Match Statement 
    let number = 10;
    let some_variable = match number {
        10 => 8, // match의 경우마다 다른 타입을 반환할 수 없음
        _ => "Not ten"
    };

    let number = 10;
    let some_variable = if number == 10 { 8 } else { "Something else" }; 
    // if문도 동일함, 에러 발생, 반환타입 일치X
}

fn match_number(input: i32) {
    match (input) {
        number => println!("It's the number {}", number), // number 의 값에 상관없이 무조건 실행됨
        number @ 0..=10 => println!("It's between 0 and 10. It's the number {}", number), 
        // input으로 들어온 값을 @를 이용해 범위연산함
        // number if number >= 0 && number <= 10 보다 훨씬 짧아짐
        _ => println!("It's greater than ten")
    }
}

fn main() {
    match_number(10);
}
