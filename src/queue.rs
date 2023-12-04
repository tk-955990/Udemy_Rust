use std::collections::{BinaryHeap, VecDeque};
pub fn run() {
    let mut q = VecDeque::new();
    // let mut q = vec![1, 2, 3];
    q.push_back(1);
    q.push_back(2);
    q.push_back(3);
    println!("q is : {:?}", q);

    // VecDequeにおける要素の追加
    // 最前列に追加
    q.pop_front();
    println!("q.pop_front() q is : {:?}", q);
    // 最後列に追加
    q.pop_back();
    println!("q.pop_back() q is : {:?}", q);

    // BinaryHeapにおける要素の追加、削除
    let mut bh = BinaryHeap::new();
    bh.push(1);
    bh.push(10);
    bh.push(20);
    bh.push(2);
    // 要素の最大値が最前列に並ぶ
    println!("bh is : {:?}", bh);
    // 最大値の要素が削除される
    println!("bh.pop() bh is : {:?}", bh.pop());
    // 要素の最大値が最前列に並ぶ
    println!("bh is : {:?}", bh);
}
