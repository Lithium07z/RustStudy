/*
Topic : Rust 생명 주기와 소유권, Call by Value & Call by Reference 차이, Shadowing 추가
*/
fn main() {
    /*******************************************
    *            생명 주기(LifeTime)            
    *******************************************/

    fn main() {
        let mut number = 10;
        let num_ref = &mut number;
    
        //num_ref = 20; // 에러 발생 num_ref는 &mut i32타입이므로 20을 넣을 수 없음
        *num_ref = 20; // *을 사용해 직접적으로 값을 넣어주어야 함, C에서 포인터에 값을 넣지 않는것과 같음
        println!("{number}");
    }

    let mut number = 10;
    let number_ref = &number; // immutable reference 사용으로 number 값 변경 불가능, 무결성 유지됨
    let number_change = &mut number; // 에러 발생
    /*  immutable reference인 number_ref가 마지막 출력에 호출되기 때문에 아직 lifetime이 남아있음 
        따라서 number에 대한 immutable reference가 남아있는데 number를 mutable reference로 number_change에 초기화 했기 때문에
        충돌 발생, 무결성 손상으로 에러가 발생됨 
        만약 마지막 출력이 number였다면 line4에서 immutable reference인 number_ref의 lifetime이 끝나기 때문에 에러 발생 X  
    */
    *number_change += 10;
    println!("{}", number_ref); 

    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    /*  mutable reference인 number_change는 line4이후 쓰이지 않으므로 line4까지만 lifetime이 살아있음 이후부턴 영향 X
        따라서 line8의 number_ref가 immutable reference이든 mutable reference이든 상관없이 정상 작동함
        하지만 line9 이후에 number_change가 쓰인다면 number_change의 lifetime이 끝나기전에 새로운 reference가 등장하므로 에러 발생
    */
    let number_ref = &number;
    println!("{}", number_ref);

    /*******************************************
    *             소유권 (OwnerShip)            
    *******************************************/

    fn print_country(country_name: String) {
        println!("My country is {}", country_name);
    }
    fn main() {
        let country = "대한민국".to_owned();
        print_country(country); // 정상 동작
        print_country(country); // 에러 발생
        /*  line6번에서 country를 call by value로 넘겨줌으로써 소유권의 이전이 발생함
            이 때문에 line6의 호출 이후에 main의 country는 소유권이 없으므로 사용이 불가능
        */
    }

    fn print_country(country_name: String) -> String {
        println!("My country is {}", country_name);
        country_name
    }
    fn main() {
        let mut country = "대한민국".to_owned();
        country = print_country(country); 
        country = print_country(country); 
        country = print_country(country); 
    }   // 이렇게 사용할 수도 있겠지만 Rust스럽게 사용하는 것은 아님

    fn print_country(country_name: &String) {
        println!("My country is {}", country_name);
    }
    fn main() {
        let country = "대한민국".to_owned();
        print_country(&country); // call by reference로 소유권의 이전이 없음
        print_country(&country); 
        print_country(&country); 
    }   // 소유권의 이전이 없으므로 모두 정상적으로 동작함, 가장 바람직한 방법

    fn add_is_great(mut country_name: String) { // 소유권이 이전되면서 mutable로 선언됨
        country_name.push_str(" is great"); // 값의 변경이 가능해짐
        println!("Now it says: {}", country_name); // 정상 동작
    }
    
    fn main() {
        let country = "대한민국".to_owned(); // immutable let
        add_is_great(country); // call by value로 인해 소유권이 이전됨
    }

    fn add_is_great(country_name: &mut String) {
        country_name.push_str(" is great");
        println!("Now it says: {}", country_name);
    }
    
    fn main() {
        let mut country = "대한민국".to_owned(); // mutable String
        add_is_great(&mut country); // mutable reference로 전달, Now it says: 대한민국 is great
        add_is_great(&mut country); // Now it says: 대한민국 is great is great
    }   // mutable reference이므로 변경된 값이 누적됨, 소유권 이전X

    /***********************************************
    *        Call by Value, Call by Reference        
    ***********************************************/

    fn print_number(number: i32) {
        println!("{}", number);
    }
    fn main() {
        let my_number = 8;
        print_number(my_number);
        print_number(my_number);
    }   // Scalar타입 데이터는 소유권 이전없이 그냥 사용가능


    fn print_number(number: i32) {
        println!("{}", number);
    }
    
    fn print_string(input: String) {
        println!("{}", input);
    }
    
    fn main() {
        let my_number = 8;
        print_number(my_number);
        print_number(my_number);
        // Scalar타입 데이터는 소유권 이전없이 그냥 사용가능
    
        let my_country = "Korea".to_owned();
        print_string(my_country.clone()); // clone을 사용해 my_country의 사본을 call by value의 인자로 사용함
        print_string(my_country); // clone이 사용됬으므로 my_country의 소유권 이전이 발생하지 않음, 정상 동작하지만 좋은 방법은 아님
    }

    /*******************************************
    *   Shadowing은 Destory가 아니라 Block이다   
    *******************************************/

    let country = "대한민국";
    let country_ref = &country; // country_ref는 country메모리를 참조함
    let country = 8;
    /*  shadowing으로 country는 새로 8의 값을 얻게되었지만, 그전의 값이 메모리 해제되는 것은 아님.
        country_ref는 여전히 메모리의 "대한민국"을 참조하고 있고 정상적으로 사용이 가능함
				다만 country라는 이름으로 쓸 수 없을 뿐임 
     */
    println!("{}, {}", country_ref, country); // 정상 동작, 대한민국 8

    /*******************************
    *      초기화되지 않은 변수      
    *******************************/

    fn main() {
        let my_number; // 에러 발생, 선언과 동시에 초기화 해야함
        let my_number: u8; // 일단 에러가 발생하진 않음 
    
        println!("{}", my_number); // 에러 발생, 초기화되지 않았으므로 사용 불가능
    }
}