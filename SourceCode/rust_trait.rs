/*
Topic : Rust Trait
*/
/*  struct User << things (객체, 대상)
    enum Months << choices (선택지, 분기문)
    trait << verbs, adjectives (실제 하려는 기능)
*/

// Case 1, trait 설명
use std::fmt::Debug;

#[derive(Debug)] // derive(Debug) 덕분에 Debug trait을 impl했으므로 Debug를 사용할 수 있음
// 모든 Trait을 이렇게 사용할 수 있는 것은 아님
struct Mystruct {
    number: usize,
}

#[derive(Add)] // 에러 발생, 어떤 방식으로 더할지 직접 구현해줘야 함
struct ThingsToAdd {
    first_thing: u32,
    second_thing: f32
}

fn print_as_debug<T>(input: T) where T: Debug, {
    println!("{input:?}");
}

fn main() {
    let my_thing = ThingsToAdd {
        first_thing: 32,
        second_thing: 8.8
    };

    let second_thing = ThingsToAdd {
        first_thing: 32,
        second_thing: 8.8
    };

    let sum = my_thing + second_thing;
}

// Case 2, trait 사용 및 디폴트 구현
struct Animal {
    name: String
}

trait Canine {
    fn bark(&self) { // default impl 구현
        println!("Woof woof");
        println!("{}", self.name); // 에러 발생, Canine을 impl하는 대상이 name필드가 없을 수도 있음
    }
    fn run(&self) { // default impl 구현
        println!("I am running");
    }
}

impl Canine for Animal { // impl TraitName for TypeName으로 struct에 trait을 붙여줌
    /*  fn bark() -> i8 { << 에러발생, trait 형식에 맞춰 구현해야함
            8
        }
    */
    fn bark(&self) {
        println!("멍멍! 나는 {}라고 한다", self.name); // Animal은 name필드가 있으므로 정상 동작
    }
}

fn main() {
    let my_animal = Animal {
        name: "Mr. Mantle".to_owned()
    };

    my_animal.bark();
    my_animal.run();
}

// Case 3, 기존에 있는 trait 수정해서 사용
use std::fmt;

#[derive(Debug)]
struct Cat {
    name: String,
    age: u8
}

impl fmt::Display for Cat { // 기본적으로 구현된 fmt의 Display trait를 Cat이 사용할 수 있도록 함
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { // 기본 fmt함수 양식
        let name = &self.name; // self는 Cat 구조체이므로 name을 가짐, name은 owned타입이므로 &로 전달해 소유권 이전을 방지
        let age = self.age;
        write!(f, "My cat's name is {name} and it is {age} years old") // 버퍼에 올림
    }
}

fn main() { 
    let mr_mantle = Cat {
        name: "Reggie Mantle".to_owned(), 
        age: 4
    };

    println!("Mr. Mantle is a {mr_mantle:?}"); // trait을 받기 전 출력 방식
    println!("{mr_mantle}"); // 기존엔 디버그 출력해야 했으나 이젠 직접 수정한 Display trait을 가지므로 출력이 가능함
    // My cat's name is Reggie Mantle and it is 4 years old
}

// Case 4, 임의의 struct와 trait을 구현 후 사용
struct Monster {
    health: i32
}

struct Wizard {}
struct Ranger{}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!("You strike with your sword! Your opponent's health is now {}", opponent.health);
    }
    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!("You strike with your fist! Your opponent's health is now {}", opponent.health);
    }
}

impl FightClose for Wizard {} // Wizard는 FightClose의 함수(기능)을 사용할 수 있음 
impl FightClose for Ranger {} // Ranger는 FightClose의 함수(기능)을 사용할 수 있음 

trait FightFromDistance {
    fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
        if distance < 10 {
            opponent.health -= 10;
            println!("You strike with your bow! Your opponent's health is now {}", opponent.health);
        }
    }
    fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
        if distance < 3 {
            opponent.health -= 4;
            println!("You strike with a rock! Your opponent's health is now {}", opponent.health);
        }
    }
}

impl FightFromDistance for Ranger {} // Ranger는 FightFromDistance의 함수(기능)을 사용할 수 있음, 2개 trait의 4개 함수를 모두 사용 가능

fn main() { 
    let radagast = Wizard {};
    let aragorn = Ranger{};

    let mut uruk_hai = Monster { health: 40 };

    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 7);
}

// Case 5, trait bounds 형식으로 사용
use std::fmt::Debug;

struct Monster {
    health: i32,
}

#[derive(Debug)]
struct Wizard {
    health: i32,
}

#[derive(Debug)]
struct Ranger {
    health: i32,
}

/*  trait bounds형식으로 사용
    사용할 기능을 trait에 선언하지 않고 함수로 구현한 뒤 함수에서 받는 인자를 해당 trait을 impl한 대상으로 제한하는 방법
 */
trait Magic {}
trait FightClose {}
trait FightFromDistance {}

impl Magic for Wizard {}
impl FightClose for Ranger {}
impl FightClose for Wizard {}
impl FightFromDistance for Ranger {}

fn attack_with_bow<T>(character: &T, opponent: &mut Monster, distance: u32)
where
    T: FightFromDistance + Debug, 
    /*  제너릭 T는 FightFromDistance와 Debug trait을 impl한 대상이어야함 
        이는 Ranger만 현재 함수를 사용할 수 있도록 영역을 제한(bounds)하는 것과 같음 
    */
{
    if distance < 10 {
        opponent.health -= 10;
        println!(
            "You attack with your bow. Your opponent now has {} health left. You are now at: {character:?}",
            opponent.health
        );
    }
}

fn attack_with_sword<T>(character: &T, opponent: &mut Monster)
where
    T: FightClose + Debug,
{
    opponent.health -= 10;
    println!(
        "You attack with your sword. Your opponent now has {} health left. You are now at: {character:?}",
        opponent.health
    );
}

fn fireball<T>(character: &T, opponent: &mut Monster, distance: u32)
where
    T: Magic + Debug,
{
    if distance < 15 {
        opponent.health -= 20;
        println!(
            "You raise your hands and cast a fireball! Yout opponent now has {} health left. You are now at: {character:?}", 
            opponent.health
        );
    }
}

fn main() {
    let radagast = Wizard { health: 60 };

    let aragon = Ranger { health: 80 };

    let mut uruk_hai = Monster { health: 40 };
    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragon, &mut uruk_hai, 7);
    fireball(&radagast, &mut uruk_hai, 12);
}