/*
Topic : Rust 생명 주기 (Lifetime)
*/
fn main() {
    /* Rust에서 reference와 lifetime은 엄격하게 관리되며 따라서 컴파일시간에 많은 에러와 unsafe를 잡아낼 수 있다.
    *  1. mut ref는 오직 하나만 가능하다.
    *  2. 일반 ref는 여러개 존재해도 상관 없다.
    *  3. mut ref, ref는 공존하지 못한다.
    */
    let mut number = 10;
    let num_ref = &mut number;

    //num_ref = 20; // 에러 발생 num_ref는 &mut i32타입이므로 20을 넣을 수 없음
    *num_ref = 20; // *을 사용해 직접적으로 값을 넣어주어야 함, C에서 포인터에 값을 넣지 않는것과 같음
    println!("{number}");


    // Case 1
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

    // Case 2
    let mut number = 10;
    let number_change = &mut number;
    *number_change += 10;
    /*  mutable reference인 number_change는 line4이후 쓰이지 않으므로 line4까지만 lifetime이 살아있음 이후부턴 영향 X
        따라서 line8의 number_ref가 immutable reference이든 mutable reference이든 상관없이 정상 작동함
        하지만 line9 이후에 number_change가 쓰인다면 number_change의 lifetime이 끝나기전에 새로운 reference가 등장하므로 에러 발생
    */
    let number_ref = &number;
    println!("{}", number_ref);

    // Case 3
    let a = 100;
    let a_ptr = &a;
    let a = 200; // a 쉐도잉
    
    println!("a의 참조 : {a_ptr}"); 
    /* a의 원래 주소는 a_ptr이 참조하므로 라이프타임이 끝나지 않음 따라서 메모리 해제되지 않고 사용이 가능함
       a_ptr이 이후 더이상 사용되지 않으면 라이프타임이 끝나면서 GC에 의해 메모리가 해제될 것임
       a_ptr 이후의 쉐도잉은 a라는 별명을 다른 공간이 사용하는 것 뿐이다. 이전 a와는 전혀 상관없음
    */
}
