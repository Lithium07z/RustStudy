/*
Topic : Rust if let and while let
*/
fn main() {
    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(0); // 안전한 데이터 접근 방법 데이터가 있다면 Some에 담아서 반환 없다면 None 반환
    let get_two = my_vec.get(10);

    println!("{:?}, {:?}", get_one, get_two);

    for index in 0..10 {
        match my_vec.get(index) { // None의 경우 아무것도 하지 않는 경우 이렇게 사용할 수도 있지만
            Some(number) => println!("The number is: {}", number),
            None => ()
        }
    }

    for index in 0..10 { // if let을 사용해 None의 경우를 무시해 더 짧고 직관적으로 사용할 수 있다.
        if let Some(number) = my_vec.get(index) { // let binding : my_vec.get(index)가 Some(number)에 들어갈 수 있다면 
            // Scope 내부의 코드를 실행 
            println!("The number is: {}", number);
        }
    }

    let weather_vec = vec![
        vec!["Berlin", "cloudy", "5", "-7", "78"],
        vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
        println!("For the city of {}:", city.get(0).unwrap());
        while let Some(information) = city.pop() { 
            // city에서 꺼낸 값이 Some(information)에 들어갈 수 있다면 Scope로 진입, None인 경우(Vector가 빈 경우) 종료
            if let Ok(number) = information.parse::<i32>() {
                // city에서 꺼낸 값 information을 i32타입으로 변환할 수 있고 Ok(number)에 들어갈 수 있다면 진입, 아니라면 종료
                println!("The number is: {}", number); // 최종적으로 number를 출력 city.pop() > information > number 
            }
        }
    }
}
