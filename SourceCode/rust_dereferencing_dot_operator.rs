/*
Topic : Rust Dereferencing and the dot operator
*/

struct Item {
    number: u8
}

impl Item {
    fn compare_number (&self, other_number: u8) {
        println!("Are they equal? {}", self.number == other_number) // *self.number로 사용하지 않음
    }
}

fn main() {
    let my_number = 10;
    let reference = &my_number;

    println!("Are they same? {}", my_number == reference);
    // 에러 발생, my_number는 i32지만 reference는 &i32이므로 비교 불가능

    let item = Item {
        number: 10
    };

    let reference_item = &item;
    let other_reference_item = reference_item;

    item.compare_number(10);
    reference_item.compare_number(10);
    // 정상 동작, reference_item은 &item 타입이지만 에러가 발생하지 않음 
    other_reference_item.compare_number(10);
    // &&Item도 마찬가지로 정상 동작함
    // .(Dot Operator)를 사용하면 Rust가 자동으로 Deref(*)를 사용해 실제 값을 가져옴
}