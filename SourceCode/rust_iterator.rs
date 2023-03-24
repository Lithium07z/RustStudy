/*
Topic : Rust Iterator
*/
/*  Iterator = a collection of things that you can call .next() on
    iterator 종류
        .iter() - iterator of references &T
        .iter_mut() - iterator of mutable references &mut T
        .into_iter() - consuming iterator
*/

fn main() {
    let vector1  = vec![1, 2, 3];
    let vector1_a = vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vector1_b: Vec<i32> = vector1.into_iter().map(|x| x * 10).collect();

    let mut vector2 = vec![10, 20, 30];
    vector2.iter_mut().for_each(|num| *num += 100);

    println!("{vector1_a:?}"); // 1, 2, 3을 1씩 증가시킨 뒤 다시 vec에 담았으므로 [2, 3, 4] 출력
    println!("{vector2:?}"); // 10, 20, 30을 10씩 증가시킨 뒤 다시 vec에 담았으므로 [110, 120, 130] 출력
    println!("{:?}", vector1_b); // 1, 2, 3을 10씩 곱한 뒤 다시 vec에 담았으므로 [10, 20, 30] 출력
    println!("{vector1:?}"); // 에러 발생, .into_iter()는 소유권이 이전되므로 더이상 사용 불가능함 

    /*  map(), for_each(), collect()
        map(f), for_each(f)는 iterator를 받아서 클로저 f를 수행한다.
        
        Ex) fn for_each<F>(self, f: F);
            fn map<B, F>(self, f: F) -> Map<Self, F>;
    
        1. map()은 반환 타입이 있다.
        2. for_each()는 반환 타입이 없다.
        3. collect()는 iterator 객체들을 풀어서 값을 얻는다. 
        - skip, take, map등의 연산 뒤에 값을 얻는다.
     */

    let my_vec = vec!['a', 'b', '거', '柳'];
    let mut my_vec_iter = my_vec.iter();

    assert_eq!(my_vec_iter.next(), Some(&'a')); // 두 인자가 같으면 통과
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'柳'));
    assert_eq!(my_vec_iter.next(), None);

    /*  .next()는 다음 아이템을 반환함 타입은 Option<T> 다음 아이템이 있다면 Some에 담아 반환하고
        다음 아이템이 없다면 None을 반환한다.
     */
}