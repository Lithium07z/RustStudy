/*
Topic : Rust AsRef Trait
*/
use std::fmt::Display;

fn print_it<T: Display + AsRef<str>>(input: T) {
    /*  제너릭 T는 Display trait을 가지고 있어야 하며 AsRef<str>에 의해 공통적인 str레퍼런스 타입 하나로 일반화할 수 있어야 한다.
        Ex) String > using AsRef<str> > &str (O)
        &str > using AsRef<str> > &str  (O)
        10 > using AsRef<str> > 10 can't be the reference of str (X)
    */
    println!("{input}");
}

fn main() {
    print_it("Hello");
    // print_it(10); // 에러 발생, AsRef<str> trait이 없으므로 print_it() 사용 불가
}