/*
Topic : Rust 타입(Compound)
*/
/* 배열(array)은 동일한 데이터 타입을 갖는 요소들의 집합으로서, 고정된 길이를 갖는다. 
*  튜플은 각 요소마다 데이터 타입이 다를 수 있지만, 배열은 동일한 데이터 타입만을 갖는다. 
*  Rust에서 배열은 스택(stack)에 할당된다.
*/
fn main() {
    /****************
    *      배열    
    ****************/
    let arr1 = [0; 10]; // 타입 u32(default), 크기는 0-9, 0으로 초기화
    let arr2 = [1, 2, 3, 4]; // 순서대로 1, 2, 3, 4
    let arr3: [u32; 10]; // 초기화 하지 않은 상태              
    let arr4: [[u32; 5]; 2] = [ // 다차원 배열 선언, u32타입 2행 5열 
        [0, 1, 2, 3, 4], 
        [5, 6, 7, 8, 9]
    ];

    println!("{ }", arr2[0]); // 출력 1

    for index in 0..4 { // 0 ~ 3까지 index에 전달
        println!("{}", arr2[index]); // arr2 0 ~ 3까지 값 출력 1, 2, 3, 4
    }

    let array = ["One", "Two"]; // [&str; 2]
    let array2 = ["One", "Two", "Five"]; // [&str; 3]

    println!("Is array the same as array2? {}", array == array2); // 에러 발생
    /*  error[E0277]: can't compare `[&str; 2]` with `[&str; 3]`
        array와 array2는 엄연히 다른 타입이므로 비교가 불가능하다 
    */

    let array = ["One", "Two"]; // [&str; 2]
    let array2 = ["One", "Two",]; // [&str; 2]

    println!("Is array the same as array2? {}", array == array2); // 정상 동작, true

    let array = ["One", "Two"];
    
    array.abcdefg(); // 실존하지 않는 메소드를 사용하려 한다.
	/*  error[E0599]: no method named `abcdefg` found for array `[&str; 2]` in the current scope
		에러와 함께 배열의 타입을 알려준다. 사실상 귀찮으니 컴파일러한테 알려달라고 하는 셈	
	*/	

    let array = ["1월", "2월"];
    println!("{:?}", array[3]); // 에러 발생, 프로그램 비정상 종료, index out of bounds
    println!("{:?}", array.get(3)); // 일단 동작, 프로그램 정상 종료, None
	println!("{:?}", array.get(1)); // get의 인자가 정상 범위라면 Some에 담아서 반환, Some("2월")
	// Some, None을 Option enum이라고 함

    /****************
    *      튜플    
    ****************/

    let dat: (i32, char, bool) = (1, 'A', true); // 명시적인 타입 지정
    let usr = ("Tom", 'B'); // 컴파일러의 타입 추론 (&str, char)

    // 튜플에서 값을 읽어 내기 위해선 첫번째 값은 {변수명}.0, 두번째 값은 {변수명}.1 과 같은 방식을 사용한다.
    let a = dat.0; 
    let b: char = dat.1;
    let c: bool = dat.2;
    let d = usr.0;
    let e = usr.1;

    println!("{}", a); // 1
    println!("{}", b); // A
    println!("{}", c); // true
    println!("{}", d); // Tom
    println!("{}", e); // B

    let dat: (i32, char, bool) = (1, 'A', true); // 명시적인 타입 지정
    let usr = ("Tom", 'B'); // 컴파일러의 타입 추론 (&str, char)

    /*  튜플에 값을 읽는 또다른 방식으로 튜플 전체를 읽어 여러 변수들에 나누어 할당하는 방식이 있는데
        이를 Destructuring 이라 부른다. 예를 들어 위의 dat 튜플은 다음과 같이 Destructuring 할 수 있다.
    */
    let (a, b, c) = dat;
    println!("{a}"); // 1
    println!("{b}"); // A
    println!("{c}"); // true

    let seasons = ["봄", "여름", "가을", "겨울", "봄", "여름", "가을", "겨울"];

    println!("{:?}", seasons[0..2]); 
    // 에러 발생, [&str]은 dynamically sized type이므로 컴파일 시간에 크기를 알 수 없음
    // 배열, 문자열 등... 모두 동적 데이터 타입이므로, 컴파일 시간에 데이터 크기를 알 수 없다. 따라서 참조함으로써
    // 그 데이터의 사이즈를 본다.
    
    println!("{:?}", &seasons[0..2]); // ["봄", "여름"]
    println!("{:?}", &seasons[0..=2]); // ["봄", "여름", "가을"]
    println!("{:?}", &seasons[..]); // 전체 출력
    println!("{:?}", &seasons[3..]); // 겨울부터 전체 출력
    println!("{:?}", &seasons[..=4]); // 두번째 봄까지 출력
}
