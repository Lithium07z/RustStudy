/*
Topic : Rust 문자열 타입(String, &str)
*/
fn main() {
    // 문자열 타입 종류
    let a = "Hello".to_string(); // String
    let b = &a; // &String
    let a = "World"; // &str

    /************************************/
    /*            문자열 출력            */
    /************************************/

    println!(r#"c:\this_drive\new_drive"#); // raw text, r# ~ # 사이의 문자열을 있는 그대로 출력함 

    println!("Hello
    World
    !"); // 입력한 문자열의 \n를 적용함

    /* 출력
    Hello
        World
    !     
    */

    let variable = &10; // 포인터 variable
    println!("{:p}", variable); // :p를 사용해 variable 데이터의 메모리 주소를 출력

	let variable = 20;
    println!("{:x}", variable); // :x를 사용해 16진수로 변수의 값을 출력, 16진수의 영어가 소문자로 출력
	println!("{:X}", variable); // :x를 사용해 16진수로 변수의 값을 출력, 16진수의 영어가 대문자로 출력

	let variable = 100;
    println!("{:b}", variable); // :b를 사용해 이진수로 변수의 값을 출력

    let title = "TODAY'S NEWS";
    println!("{:-^30}", title); // 전체 길이가 30이고 title을 중간에 놓은 상태에서 양 옆을 -로 채운 형태 
    let bar = "|";
    println!("{: <15}{: >15}", bar, bar); // 왼쪽 끝에 bar 추가 후 15칸 공백, 15칸 공백 후 오른쪽 끝에 bar 
    let a = "SEOUL";
    let b = "TOKYO";
    println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b);

    /* 출력
    ---------TODAY'S NEWS---------
    |                            |
    SEOUL--------------------TOKYO */

    /************************************/
    /*            문자열 조작            */
    /************************************/

    let s = "hello";    // let s: &'static str = "hello"; 와 동일
    println!("{}", s);  // s는 'static lifetime을 가지는 &str타입의 변수

    let s = "hello"; // &str
    let sub: &str = &s[1..4]; 
    println!("{}", sub);    // "ell" 출력

    let my_name = "Junho Kim"; // &str
	let my_name = format!("Junho Kim");       // String - 포매팅 매크로는 문자열 접합에 사용
	// 문자열 접합시 소유권 이전이 있다. 이 매크로로 쉽게 문자열 조작이 가능하다.
    let my_name = "Junho Kim".to_string();    // String
    /*  to_string()의 API문서는 아래와 같음 결국 to_owned()를 사용하므로 string에선 to_string보단 to_owned()를 사용하는 것이 좋음 
        fn to_string(&self) -> String {
            self.to_owned()
        }
    */
    let my_name = "Junho Kim".to_owned();     // String
    let my_name = String::from("Junho Kim");  // String
	let my_name: String = "Junho Kim".into(); // String
    // to_string()은 임의의 타입을 String 타입으로 변환한다.
    // 하지만 더 많은 메모리를 사용하므로 &str 타입에서는 to_owned() 함수를 사용하는 것이 좋다.
	// into()를 사용할 때는 변수를 미리 String타입으로 지정해주어야 한다.	

	let mut my_other_name = "Junho Kim".to_owned();
    my_other_name.push('!');
    println!("{}", my_other_name); // Junho Kim!, String타입이므로 수정 가능

	let mut my_other_name = "Junho Kim";
    my_other_name.push('!');  // 에러 발생
    println!("{}", my_other_name); // Junho Kim!, &str타입이므로 수정 불가능

	let a = "Hello".to_owned();
	let b = "World".to_string();
	let c = a + b; // 에러 발생, 두 문자열을 더하려면 b가 &str이어야 함 (&a + b는 불가능)
	let c = a + &b; // 성공, 하지만 이 경우 a의 소유권이 c로 넘어가 a는 더이상 사용이 불가능함 (비대칭)
	/* string 객체의 add함수
	    fn add(mut self, other: &str) -> String { // add를 호출한 자신과 더할 문자열을 &str로 받음
            self.push_str(other); // 이후 자신에게 더할 문자열을 복사하여 추가
            self // 자기 자신 반환 
        }
	*/
	// 따라서 두 문자열을 합칠 때는 format! 매크로를 사용하는 것이 좋다.
	let c = format!("{}{}", a, b); // 소유권 이전 없이 복사하여 접합하기 때문에 a, b, c 모두 사용가능

    let mut my_name = "Junho Kim".to_owned();
    my_name.push('!'); // 문자열에 문자 추가
    my_name.push_str(" and I live in Korea"); // 문자열에 문자열 추가
    println!("{}", my_name);

    let mut my_name = "hi".to_owned();
    println!("{}", my_name.capacity()); // 2
    my_name.push_str(" my name");
    println!("{}", my_name.capacity()); // 10
    my_name.push_str(" is Junho Kim");
    println!("{}", my_name.capacity()); // 23
	// .capacity() : 해당 문자열이 차지하고 있는 공간의 크기를 반환함, .len()보다 같거나 크다.

    let mut my_name = String::with_capacity(26); // 문자열의 공간을 26으로 지정
    println!("{}", my_name.capacity()); // 26
    my_name.push_str(" my name");
    println!("{}", my_name.capacity()); // 26
    my_name.push_str(" is Junho Kim");
    println!("{}", my_name.capacity()); // 26
	// 26보다 더 큰 사이즈가 필요하면 그때 reallocation
}