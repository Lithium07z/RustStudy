/*
Topic : Rust 구조체 (struct)
*/

/* 구조체는 일종의 복합적인 Custom 데이터 타입이라고 볼 수 있다. 
   Rust의 구조체는 struct 정의 안에 필드만을 가지며, 메서드를 정의하지 않는다.
   구조체와 연관된 함수는 밖에서 구현한다.
*/

// unit struct or unit-like struct, 이름만 있는 struct, 0Byte
struct FileDirectory;

fn takes_file_directory(input: FileDirectory) {
    println!("I got a file directory");
}

fn main() {
    let x = FileDirectory;
    takes_file_directory(x);
}

// tuple struct
#[derive(Debug)] // attribute
struct Colour(u8, u8, u8);

fn main() {
    let my_colour = Colour(20, 50, 100);
    // tuple struct는 필드명이 없으므로 인덱스를 통해 필드 값에 접근한다. Ex) ~~.0, ~~.1, ~~.2, ...
    println!("The second colour is {}", my_colour.1); // The second colour is 50
    println!("{:?}", my_colour); // Colour(20, 50, 100)
}

// named struct
struct Country {
    population: u32, // 필드명, 타입
    capital: String,
    leader_name: String,
}

fn main() {
    // 구조체 초기화
    let canada = Country {
        population: 35_000_000,
        capital: "Ottawa".to_owned(),
        leader_name: "Justin Trudeau".to_owned(),
    };
    println!("The population is: {}\nThe capital is: {}", canada.population, canada.capital);
    // .을 이용해 필드 값을 읽음
}

/* struct 필드 단축 초기화
   함수에 전달되는 파라미터명과 struct 필드명이 동일하다면, 
   struct 초기화시 "필드명:파라미터명" 대신, "필드명"과 같이 ":파라미터명"을 생략할 수 있다.
*/

struct Country {
    population: u32, 
    capital: String,
    leader_name: String,
}

fn main() {
    let population = 35_000_000;
    let capital = "Ottawa".to_owned();
    let leader_name = "Justin Trudeau".to_owned();

    let my_country = Country {
        population, // struct 필드명과 파라미터명이 같은 경우 사용 population: population, 보다 훨씬 간편함
        capital,
        leader_name
    };
}

// Struct Update 문법 
// Struct Update 문법(struct update syntax)은 "..{구조체변수}" 으로, 복제할 이전 구조체 변수 앞에 .. 을 써서 나머지를 이전 구조체의 데이터로 채우는 것이다.
struct Admin {
    name: String,
    group: String,
    active: bool
}
 
fn main() {
    let adm1 = Admin {
        name: String::from("Tom"),
        group: String::from("IT"),
        active: true
    };
 
    let adm2 = Admin {
        name: String::from("Kim"), // name 필드만을 변경함
        ..adm1     // struct update syntax, 나머지 필드는 adm1으로부터 받아옴
    };
 
    println!("{}", adm2.group); // OK
    println!("{}", adm1.name);  // OK
    //println!("{}", adm1.group); // 에러
    /* 한가지 주의할 점은, 이렇게 다른 구조체의 값을 가져올 때, 소유권을 가진 데이타 타입(Owned type)에 대해서는 소유권 이전(move)이 일어날 수 있다는 점이다. 
       예를 들어, 위의 경우 adm1의 group 필드는 그 소유권이 adm2.group 으로 이동하게 되어, 이후 adm1.group을 사용할 수 없게 된다.
       adm1.name의 경우는 adm2.name으로 소유권이 이전 되지 않았기 때문에 사용할 수 있다.
     */
}
