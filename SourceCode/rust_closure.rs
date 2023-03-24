/*
Topic : Rust closure
*/
// closure = anonymous functions that capture their environment

fn main() {
    let my_number = 10;
    let my_closure = |x: i32| println!("{}", x + my_number);

    my_closure(5);

    let my_closure = || { // 내용이 길어질 땐 {}를 사용할 수 있음
        let my_number = 7;
        let other_number = 10;
        println!("The two numbers are {my_number} and {other_number}");
        my_number + other_number
    };

    /*  .iter().map(|item| {
            let my_number = 7;
            item + my_number
        })
        .collect()
        같은 형태로도 사용이 가능하다.
     */

    let my_var = my_closure();
    println!("{my_var}")
}

fn main() {
    let my_number = 9;
    let anonymous_function = || println!("I am a function");
    let closure = || println!("{my_number}");
    // 서로 다르지만 보통 같게본다.

    // .iter().map().fillter().inspect().collect()
    // zero cost abstractions
    // chaining을 하더라도 속도에는 전혀 영향이 없다. 컴파일 시간은 증가할 수 있음

    let my_vec = vec![8, 9, 10];
    let fourth = my_vec.get(3).unwrap_or_else(|| {
        if my_vec.get(0).is_some() {
            &my_vec[0]
        } else {
            &0
        }
    });

    println!("{fourth}");
}

fn main() {
    // .iter()를 map으로 받고 다시 vec으로 바꾸는 경우
    let num_vec = vec![2, 4, 6];

    let double_vec = num_vec
        .iter()
        .map(|number| number * 2)
        .collect::<Vec<i32>>();

    println!("{double_vec:?}"); // [4, 8, 12]

    let num_vec = vec![2, 4, 6];


    // .iter를 map으로 받고 또 map으로 받는 경우
    let double_vec = num_vec
        .iter()
        .map(|number| number * 2)
        .map(|number| number * 3);

    println!("{double_vec:?}"); 
    // Map { iter: Map { iter: Iter([2, 4, 6]) } }
    // zero cost abstractions


    // enumerate()를 for_each()에서 enumerate값으로 받는 경우
    let num_vec = vec![2, 4, 6];

    num_vec
        .iter() // 2, 4, 6
        .enumerate() // (0, 2), (1, 4), (2, 6)
        .for_each(|(index, number)| {
            println!("The number at index {index} is {number}");
        });
    /*  The number at index 0 is 2
        The number at index 1 is 4
        The number at index 2 is 6  */

    
    // enumerate()를 for_each()에서 tuple로 받는 경우
    let num_vec = vec![2, 4, 6];

    num_vec
        .iter() // 2, 4, 6
        .enumerate() // (0, 2), (1, 4), (2, 6)
        .for_each(|tuple| {
            println!("The number at index {} is {}", tuple.0, tuple.1);
        });

    /*  The number at index 0 is 2
        The number at index 1 is 4
        The number at index 2 is 6  */


    let num_vec = vec![2, 4, 6];

    num_vec
        .iter() // 2, 4, 6
        .enumerate() // (0, 2), (1, 4), (2, 6)
        .map(|(index, number)| {
            println!("The number at index {index} is {number}");
       });
    // 아무것도 동작하지 않음 구조만 만든 상태이기 때문, Map<Enumerate<std::slice::Iter<'_, {integer}>> 타입
    // warning: unused `Map` that must be used, note: iterators are lazy and do nothing unless consumed 


    let num_vec = vec![2, 4, 6];

    let new_thing = num_vec
        .iter() // 2, 4, 6
        .enumerate() // (0, 2), (1, 4), (2, 6)
        .map(|(index, number)| {
            println!("The number at index {index} is {number}");
            0
       })
       .collect::<Vec<_>>();
       /*   .collect::<Vec<_>>(); 때문에 iterator가 움직이고 따라서 출력됨
            The number at index 0 is 2
            The number at index 1 is 4
            The number at index 2 is 6
            [0, 0, 0] */
    println!("{new_thing:?}");
}