/*
Topic : Rust 반복문
*/
// loop 반복문
fn loop_then_return(mut counter: i32) -> i32{
    loop {
        counter += 1;
        if counter % 50 == 0 {
            break;
        }
    }
    counter
}   // Rust적이진 않지만 다른 언어들에서 사용하는 방식으로 반복문 구현이 가능함

fn main() {
    let my_number; 
    {
        let x = loop_then_return(43);
        my_number = x
    }

    println!("{}", my_number);
}

// while 반복문
fn main() {
    let mut sum = 0;
    let mut i = 1;
 
    while i <= 100 {
        sum += i;        
        i = i + 1;
    }
 
    println!("Sum of 1~100: {}", sum);
}

// for 반복문
fn main() {
    let arr = [1,2,3,4,5];
 
    for i in arr {
        println!("{}", i);
    }
}

fn main() {
    let mut sum = 0;
    for i in 1..101 {
        sum += i;
    }
    println!("Sum of 1~100: {}", sum);
}

// break와 continue
fn main() {
    let mut i = 0;
    let mut sum = 0;
 
    let result = loop {
        i += 1;
 
        if i % 2 == 1 {
            continue;
        }
 
        sum += i;
 
        if i == 10 {
            break sum / 2 // Rust는 break 뒤에 이렇게 Expression을 넣어 값을 리턴할 수 있다
        }
    };
 
    println!("{}", result);
}