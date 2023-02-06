/*
Topic : Rust Generics
*/
// Case 1, 정상 동작, 일반적인 제너릭 사용 상황
fn give_thing<T>(input: T) -> T {
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    println!("{}", x);    
    println!("{}", y);
}   // 정상 동작, 다른 언어에서의 제너릭과 비슷함

// Case 2, 에러 발생, Generic이 특정 trait을 가지지 않는 경우
fn give_thing<T>(input: T) -> T {
    println!("{}", input); 
    // 에러 발생, input이 Display trait을 가지고 있어야 가능한데 input은 Generic이라 Display trait이 있는 값이 올지 알 수 없음
    // error[E0277]: `T` doesn't implement `std::fmt::Display`
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    println!("{}", x);    
    println!("{}", y);
}   

// Case 3 - 1, 정상 동작, Generic이 특정 trait을 가지지 않는 경우
fn give_thing<T: std::fmt::Display>(input: T) -> T { // Generic T는 Display trait을 가진 값만을 받음
    println!("{}", input); 
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    println!("{}", x);    
    println!("{}", y);
}   

// Case 3 - 2, 정상 동작, Generic이 특정 trait을 가지지 않는 경우
use std::fmt::Display;

fn give_thing<T: Display>(input: T) -> T { // Generic T는 Display trait을 가진 값만을 받음
    println!("{}", input); 
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    println!("{}", x);    
    println!("{}", y);
}   

// Case 4, 에러 발생, Generic이 특정 trait을 가지지 않는 경우
struct Book;
// 구조체 Book은 Display trait이 없다.

use std::fmt::Display;

fn give_thing<T: Display>(input: T) -> T { // Generic T는 Display trait을 가진 값만을 받음
    println!("{}", input); 
    input
}

fn main() {
    let x = give_thing(String::from("Take this thing"));
    let y = give_thing(9);
    let z = give_thing(Book); // 에러 발생, error[E0277]: `Book` doesn't implement `std::fmt::Display`
    println!("{}", x);    
    println!("{}", y);
}   

// Case 5, 에러 발생, 하나의 Generic이 여러 trait을 필요로 할 때 
use std::fmt::Display;

fn compare_and_print<T: Display>(statement: T, num_1: T, num_2: T) {
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2 // 에러 발생, Gereric T는 Display trait만 가지고 있지 비교에 필요한 trait을 가지고 있을지는 모름
    );
}

fn main() {
    compare_and_print("Listen up!", 9, 8);
}

// Case 6, 정상 동작, 하나의 Generic이 여러 trait을 필요로 할 때 
use std::fmt::Display;
use std::cmp::PartialOrd;

fn compare_and_print<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    // T는 Display trait을 가지고 있는 값만 가능하고 U는 Display trait과 PartialOrd trait을 모두 가지고 있는 값만 가능하다.
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2 // num_1과 num_2는 Display, PartialOrd trait을 모두 가지고 있으므로 출력 및 비교가 가능함
    );
}

fn main() {
    compare_and_print("Listen up!", 9, 8);
}

// Case 7, 정상 동작, where문 사용
use std::fmt::Display;
use std::cmp::PartialOrd;

fn compare_and_print<T, U>(statement: T, num_1: U, num_2: U) 
    where 
        T: Display,
        U: Display + PartialOrd,
    {
    println!(
        "{}! Is {} greater than {}? {}",
        statement,
        num_1,
        num_2,
        num_1 > num_2
    );
}

fn main() {
    compare_and_print("Listen up!", 9, 8);
}