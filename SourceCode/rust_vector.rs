/*
Topic : Rust vector
*/
fn main() {
    //let mut my_vec = Vec::new(); // 에러 발생, 타입이 지정되지 않은 Vector
    let mut my_vec: Vec<String> = Vec::new(); // 정상 동작, String타입의 Vector임을 명시함 

    let name1 = "Junho".to_owned();
    let name2 = "Jiho".to_owned();

    let mut my_vec = Vec::new(); // 타입 지정이 되지 않았지만
    my_vec.push(name1); // 데이터가 추가됨으로써 타입 추론이 가능해짐
    my_vec.push(name2);

    println!("Hello {:?}", my_vec); // 정상 동작

    let name1 = "Junho".to_owned();
    let name2 = "Jiho".to_owned();

    let mut my_vec = vec![name1, name2]; // 매크로를 사용해 더 쉽게 Vector를 만들 수 있다

    println!("Hello {:?}", my_vec);

    let my_vec = vec![("Hey", 9), ("Hello there", 8972983)];
    // &str, i32타입의 원소를 가지는 튜플을 원소로 갖는 벡터가 됨 Vec<&str, i32>

    let arr:[i32; 10] = [1; 10];  // i32타입 10크기의 배열 1로 초기화
    
    let vec = Vec::from([8, 9, 10]);   // from 함수를 사용한 배열 > 벡터 변환
    let vec:Vec<i32> = Vec::from(arr); // from 함수를 사용한 배열 > 벡터 변환
    println!("{:?}", vec); // [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]

    let vec:Vec<i32> = vec![1, 2, 3]; // 벡터 매크로를 사용한 벡터 생성
    println!("{:?}", vec); // [1, 2, 3]

    let vec:Vec<u8> = Vec::from(String::from("HELLO")); 
    // from 함수를 사용한 Stirng > 벡터 변환
    // Vec::from을 사용해 벡터변환시 생성되는 벡터는 u8타입임
    println!("{:?}", vec); // [72, 69, 76, 76, 79], ASCII로 출력됨

    // 튜플을 포함한 배열의 2차원 배열
    let t:[[(&str,i32); 10]; 5] = [[("rust",2023); 10]; 5]; 
    // &str과 i32 타입을 가지는 튜플 10개를 가진 배열, 그리고 그 배열을 5개 가진 2차원 배열
    // 10 by 5크기의 배열에 각각의 원소는 ("rust, 2023")

    for i in t.iter() { // tuple iterator 사용
        println!("{i:?}");
    }
}