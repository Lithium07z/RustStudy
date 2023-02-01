/*
Topic : Rust destructuring
*/

struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

#[derive(Debug)]
struct Person2 {
    name: String,
    height: u8
}

impl Person2 {
    fn from_person(input: Person) -> Self {
        let Person {
            name,
            height,
            .. // 나머지는 필요 없음
        } = input;

        Self {
            name,
            height
        }
    }
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_owned(),
        real_name: "Clarence".to_owned(),
        height: 170,
        happiness: false,
    };

    println!(
        "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
        papa_doc.name, papa_doc.real_name, papa_doc.height, papa_doc.happiness
    );  // destructuring 없이 일일이 출력

    let Person { name, real_name, height, happiness } = papa_doc; // destructuring

    println!(
        "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
        name, real_name, height, happiness
    );  // destructuring을 사용해 간결해짐

    let Person { name: a, real_name: b, height: c, happiness: d } = papa_doc;

    println!(
        "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
        a, b, c, d
    );  // 이름을 지정할 수도 있음

    let person2 = Person2::from_person(papa_doc);

    println!("Person2 type is: {:?}", person2);
}