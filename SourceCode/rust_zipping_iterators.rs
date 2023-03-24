/*
Topic : Rust zipping iterators
*/
use std::collections::HashMap;

fn main() {
    let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
    let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // Vec<&str>

    let number_word_hashmap: HashMap<i32, &str> = some_numbers
        .into_iter() 
        .zip(some_words.into_iter())
        .collect();

    let result_str = number_word_hashmap.get(&10).unwrap_or_else(|| {
        println!("Help");
        &"no number"
    });

    println!("{result_str}");


    let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
    let some_words = vec!["zero", "one", "two", "three", "four"]; // Vec<&str>

    let number_word_hashmap: HashMap<i32, &str> = some_numbers
        .into_iter() 
        .zip(some_words.into_iter())
        .collect();

    number_word_hashmap.iter().for_each(|stuff| {
        println!("{stuff:?}");
    });
    /*  (3, "three")
        (4, "four") 
        (0, "zero") 
        (1, "one")  
        (2, "two")  
        some_numbers와 some_words의 개수가 맞지 않지만 .zip()을 사용하면 더 짧은 쪽을 기준으로 매칭한다. */
}