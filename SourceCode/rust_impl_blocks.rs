/*
Topic : Rust impl block
*/

/*  struct, enum에 대한 실질적인 행동(method)를 구현한다.
    연관 함수 - static method
    메소드 - member method
*/

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog
}

impl AnimalType { // enum의 impl block
    fn check_type(&self) { // 메서드
        use AnimalType::*; // AnimalType:: 필요없이 AnimalType 필드 값 사용, namespace와 유사

        match self { // AnimalType enum의 impl block이므로 self == AnimalType
            Cat => println!("Animal type is cat"),
            Dog => println!("Animal type is dog")
        }
    }
}

impl Animal {
    // impl 블럭은 하나이상 정의할 수 있는데, 일반적으로 여러 개의 impl 블럭을 정의하기 보다는 하나의 impl 블럭을 정의하여 사용한다.
    fn new_old_cat() -> Self {
        Self {
            age: 15,
            animal_type: AnimalType::Cat
        }
    }
}

impl Animal {
    fn new(age: u8, animal_type: AnimalType) -> Self {
        Self {
            age,
            animal_type,
        }
    }

    fn new_cat(age: u8) -> Self { // function signature
        Self {
            age,
            animal_type: AnimalType::Cat
        }
    }

    fn new_dog(age: u8) -> Self { // function signature
        Self {
            age,
            animal_type: AnimalType::Dog
        }
    }

    fn print(&self) {
        println!("I am a: {:?}", self);
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed to dog! Now I am: {:?}", self);
    }

    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed to cat! Now I am: {:?}", self);
    }
}

fn main() {
    let mut my_animal = Animal::new_dog(10); // associated function(연관 함수)
    /*  impl block의 연관 함수를 사용해 실질적으로 impl block(Animal)을 생성한 뒤 사용할 수 있도록함
        생성자와 비슷하다고 볼 수 있음
    */
    
    my_animal.print(); 
    // print()는 메서드 이므로 ::가 아닌 .으로 사용함
    // 컴파일러는 Animal::print(&my_animal);로

    my_animal.change_to_cat();
    my_animal.change_to_dog();
    
    let my_old_cat = Animal::new_old_cat(); // 같은 이름의 다른 impl block도 사용가능

    println!("I made a: {:?}", my_animal);

    use AnimalType::*;

    let my_cat = Animal::new(10, Cat);
    let my_dog = Animal::new(10, Dog);

    my_cat.animal_type.check_type();
    // check_type()은 AnimalType enum의 메서드이고 AnimalType enum은 Animal 구조체의 멤버 이므로 
    // Animal 구조체인 my_cat에서 구조체 멤버인 animal_type의 check_type()을 사용해야 한다.
}
