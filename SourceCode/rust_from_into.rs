/*
Topic : Rust from(), into()
*/
fn main() {
    let my_name = "junho Kim".to_owned();
    let my_city: String = "Seoul".into();
    let my_city: String = String::from("Seoul"); 
	// from의 인자 값을 from을 사용하는 타입에 맞추어 바꾸어줌 (아직 확실하지 않음)
    // from > into는 항상 가능하지만 into > from은 항상 가능하지 않다
}