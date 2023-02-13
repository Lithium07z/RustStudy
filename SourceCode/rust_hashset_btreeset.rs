/*
Topic : Rust HashSet and BTreeSet
*/
use std::collections::HashSet;
use std::collections::BTreeSet;

fn main() {
    let many_numbers = vec![
        94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
        51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
        35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
        96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82,
        58, 64, 80, 16, 61, 57, 14, 11]; // 100개의 랜덤 값, 중복 포함

    let mut number_hashset = HashSet::new(); // 새로운 HashSet 생성

    for number in many_numbers {
        number_hashset.insert(number); // Vector의 값들을 하나씩 HashSet에 삽입
    }

    let hashset_length = number_hashset.len(); // HashSet의 길이 저장
    println!("There are {} unique numbers, so we are missing {}.", hashset_length, 100 - hashset_length);
    // HashSet은 중복되는 값을 받지 않으므로 100에서 HashSet의 길이를 빼면 Vector에 삽입한 값들 중 중복된 값이 몇 개 였는지 알 수 있음

    let mut missing_vec = vec![]; // 중복된 값들을 따로 저장할 Vector
    for number in 0..100 {
        if number_hashset.get(&number).is_none() { 
            // get을 통해 값을 가져오는 경우 해당 값이 없다면 None을 반환함, 이때 is_none() 함수를 사용해 None인 경우
            missing_vec.push(number); // 해당 값을 missing_vec에 삽입함
        }
    }

    print!("It does not contain: ");
    for number in missing_vec {
        print!("{} ", number); // 중복된 값들 출력
    }
    
    let mut btree_set = BTreeSet::new(); // BTreeSet 생성
    let many_numbers = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];

    for i in many_numbers {
        btree_set.insert(i);
    }

    println!();
    for number in btree_set {
        print!("{} ", number); // 정렬된 순서로 출력됨
    }
}