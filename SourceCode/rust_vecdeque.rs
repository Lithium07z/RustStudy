/*
Topic : Rust VecDeque
*/
/*  벡터를 스택, 큐로서 사용한다면 O(1)로 연산을 할 수 있다.
    벡터를 벡터로서 사용한다면 O(N)만큼 걸린다.
*/
use::std::collections::VecDeque;

fn main() {
    let mut my_vec = vec![0; 600_000];
    for i in 0..600000 {
        my_vec.remove(0);
        // Vector는 .remove()시 삭제한 뒤의 원소를 모두 앞으로 하나씩 옮김
        // 따라서 Vector에서 위와 같은 로직은 굉장히 많은 시간을 필요로 함, time-out
    }

    let mut my_vec = VecDeque::from(vec![0; 600_000]); // Vector를 VecDeque으로 변환
    for i in 0..600000 {
        my_vec.pop_front(); // Vector와 같은 연산이지만 훨씬 빠르게 동작함
    }   
    // 상황에 따라 Vector와 VecDeque을 맞추어 사용하면됨 
}
