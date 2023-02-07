/*
Topic : Rust HashMap and BTreeMap
*/
// Case 1, HashMap 사용
use std::collections::HashMap;

struct City {
    name: String,
    population: HashMap<u32, u32>, // u32타입의 Key와 Value를 가지는 HashMap
}

fn main() {

    let mut tallinn = City {
        name: "Tallinn".to_owned(),
        population: HashMap::new(), // 빈 HashMap 생성
    };

    tallinn.population.insert(1372, 3_250); // Key와 Value 삽입
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);


    for (year, population) in tallinn.population { // population HashMap에서 Key와 Value를 모두 추출함
        println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
    }   // HashMap이므로 출력 순서는 일정하지 않음
}

// Case 2, BTreeMap 사용
use std::collections::BTreeMap;

struct City {
    name: String,
    population: BTreeMap<u32, u32>, // u32타입의 Key와 Value를 가지는 BTreeMap
}

fn main() {

    let mut tallinn = City {
        name: "Tallinn".to_owned(),
        population: BTreeMap::new(), // 빈 BTreeMap 생성
    };

    tallinn.population.insert(1372, 3_250); // Key와 Value 삽입
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);


    for (year, population) in tallinn.population { // population BTreeMap에서 Key와 Value를 모두 추출함
        println!("In the year {} the city of {} had a population of {}.", year, tallinn.name, population);
    }   // BTreeMap이므로 출력순서는 Key값에 따라 정렬되어 출력됨, 자바의 TreeMap과 같음
}

// Case 3 - 1, map의 Value에 접근하는 방법
use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);        // 안전하지 못한 사용법
    println!("{:?}", city_hashmap.get("Bielefeld"));    // 안전한 사용법, Some에 값이 담겨 나옴
    println!("{:?}", city_hashmap.get("Bielefeldd"));   // 안전한 사용법, None이 반환됨 
}

// Case 3 - 2, map의 Value에 접근할 때 Key는 &로 줄것
use std::collections::HashMap;

fn main() {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");

    println!("{:?}", book_hashmap.get(1));  // 에러 발생, &로 주지 않으면 소유권 이전 발생
    println!("{:?}", book_hashmap.get(&1)); // 정상 동작, 값을 빌려오므로 소유권 이전 발생 X
}

// Case 3 - 3, Map의 소유권 이전 2
use std::collections::HashMap;
 
fn main() {
    let mut map: HashMap<String, i32> = HashMap::new();
 
    map.insert(String::from("Korea"), 5);
     
    let germ = String::from("Germany");
    map.insert(germ, 4);
    /*  만약 Key 혹은 Value에 String과 같은 owned value가 전달된다면, 
        그 소유권(ownership)은 HaspMap으로 이동(move)한다. 
        따라서, 위 예제의 변수 germ은 map.insert(germ, 4); 이후에 사용될 수 없다.
    */
    println!("{:?}", map);
    println!("{}", germ); // 에러 발생, germ 사용 불가능
}

// Case 4 - 1, HashMap함수 사용
use std::collections::HashMap;

fn main() {
    let book_collection = vec!["L'Allemagne Moderne", "Le Petit Prince", "Eye of the World", "Eye of the World"];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
        let return_value = book_hashmap.entry(book).or_insert(0); 
        // book을 Key로 가지는 Entry가 없다면 Key가 book이고 Value가 0인 Entry를 만들어 reference를 반환
        // book을 Key로 가지는 Entry가 있다면 그 Entry의 reference를 반환
        *return_value +=1; // return_value는 reference이므로 *를 붙여야 값에 접근 가능함
    }

    for (book, number) in book_hashmap {
        println!("{}, {}", book, number);
    }
}

// Case 4 - 2, HashMap함수 사용
use std::collections::HashMap;

fn main() {
    let data = vec![
        ("male", 9),
        ("female", 5),
        ("male", 0),
        ("female", 6),
        ("female", 5),
        ("male", 10),
    ];

    let mut survey_hash = HashMap::new();

    for item in data { // item: tuple(&str, i32)
        survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
        // item.0(male or female)을 Key로 가지는 Entry가 없다면 Value로 Vector를 만들어 넣고 그 Vector에 item tuple의 두번째 값을 추가함
        // item.0을 Key로 가지는 Entry가 있다면 해당 Entry의 Vector에 item tuple의 두번째 값을 추가함
    }

    for (male_or_female, numbers) in survey_hash {
        println!("{:?}: {:?}", male_or_female, numbers);
    }
}
