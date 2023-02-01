/*
Topic : Rust match pattern
*/
// Option 열거형 match 표현
fn choice(opt: Option<i32>) {
    match opt {
        None => println!("No choice"), // Option이 None이면 No choice 출력
        Some(val) => println!("#{} was chosen", val) // Option이 Some이면 Some에 담긴 값을 출력
    }
}
 
fn main() {
    let opt: Option<i32> = Option::Some(1); // i32 타입의 Option enum의 Some에 1을 담음 Some은 일종의 Generic
    choice(opt);
}

// Catch-all 패턴
/* match에서 일부 패턴만 처리하고 나머지 패턴들에 대해서는 하나의 실행코드를 실행할 수 있다 (즉, C/C#에서의 default 블럭처럼).
   아래 예제는 변수 n의 값을 체크해서 0이면 "Zero", 1이면 "One" 을 출력하고 나머지 모두에 대해서는 
   숫자를 문자열로 변환해서 리턴하는 match 식을 표현하고 있다. 
   아래에서 Catch-all 패턴은 변수 x를 사용하고 있는데, 이는 임의의 변수명을 사용할 수 있다. 
   Catch-all 패턴은 항상 match 의 마지막에 있어야 한다.
*/
fn main() {
    let n: u8 = 2;
 
    let res = match n {
        0 => String::from("Zero"),
        1 => String::from("One"),
        x => x.to_string()
    };
 
    println!("{}", res);
}

// Catch-all 패턴에서 만약 나머지의 경우 어떤 특별한 일을 할 필요가 없다면, 
// 아래와 같이 패턴 변수명에 밑줄(_) 을 사용하고 실행 코드에 unit 타입 () 을 적으면 된다.
fn main() {
    let n: u8 = 2;
 
    match n {
        0 => println!("{n}"),
        1 => println!("{n}"),
        _ => () // {}도 사용할 수 있음
    };
}

// 단, match의 반환값이 있다면 사용 불가능 
fn main() {
    let n: u8 = 2;
 
    let res = match n {
        0 => String::from("Zero"),
        1 => String::from("One"),
        _ => () // 에러 발생, 위의 arm들과 반환값을 일치시켜야함 String::new() 등을 사용해 String 반환해야함 
    };
 
    println!("{}", res);
}

// if let 표현
/* match에서 하나의 패턴에 대해 특별한 처리를 하고, 나머지를 무시하거나 혹은 나머지 전체에 대해 다른 코드 블럭을 실행하는 경우, 
   (match 대신) "if let" 이라는 간략한 표현을 사용할 수 있다. 예를 들어, 아래와 같이 Option 매칭에서 Some인 경우만 처리하고 싶으면
*/
fn process(opt: Option<i32>) {
    match opt {
        Some(val) => println!("#{} was chosen", val),
        _ => ()
    }
}

// 아래와 같이 if let을 써서 보다 간략히 표현할 수 있다.
fn process(opt: Option<i32>) {
    if let Some(val) = opt {
        println!("#{} was chosen", val);
    }
}

/* "if let" 뒤에서는 match에서의 "Some(val)"와 같은 패턴을 먼저 적고, 다음 match하고자 하는 변수명(여기서는 opt)를 적는다. 
   if let의 패턴이 맞는 경우 { } 블럭 안의 코드가 실행되고, 나머지 조건의 경우(match에서의 _ 조건)는 무시된다.
   만약 나머지 조건에 어떤 코드를 실행하기 위해서는 if let 블럭 뒤에 else 블럭을 사용할 수 있다.
*/

fn process(opt: Option<i32>) {
    if let Some(val) = opt {
        println!("#{} was chosen", val);
    } else {
        println!("No option");
    }
}