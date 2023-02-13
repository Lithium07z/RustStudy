/*
Topic : Rust BinaryHeap
*/
use std::collections::BinaryHeap;
use std::cmp::Reverse; // 최소힙을 만들기 위해 사용

fn show_remainder(input: &BinaryHeap<i32>) -> Vec<i32> { // BinaryHeap의 남은 값들을 출력하는 함수, Iterator 사용이 더 빠르지만 편의상 구현
    let mut remainder_vec = vec![];
    for number in input {
        remainder_vec.push(*number) // BinaryHeap을 reference로 받았으므로 *붙여서 사용함
    }
    remainder_vec
}

fn main() {
    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers {
        my_heap.push(number);
    }

    while let Some(number) = my_heap.pop() { // .pop()은 값이 있다면 Some에 담아 반환하고 없다면 None을 반환함, 큐의 맨 앞에서부터 꺼내옴
        println!("Popped off {}. Remaining numbers are: {:?}", number, show_remainder(&my_heap));
    }   // BinaryHeap은 기본적으로 MaxHeap이므로 가장 큰 값을 꺼내옴

    let mut minheap = BinaryHeap::new();

    for num in (1..=10).rev() { // .rev()사용으로 10부터 1까지 감소하며 반복
        minheap.push(Reverse(num)); // 최소힙으로 만들기 위해 Reverse() 사용
    }

    while let Some(Reverse(i)) = minheap.pop() {
        print!("{} ", i); // 최소힙이므로 1, 2, 3...순으로 출력
    }
}