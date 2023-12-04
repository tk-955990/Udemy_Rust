pub fn run() {
    let mut v1 = vec!["Rust", "Python", "Java"];
    println!("before v1 : {:?}", v1);
    println!("{:p}", &v1);
    // ヒープメモリ、要素数、キャパシティの確認
    println!("v1.as_ptr is {:?}", v1.as_ptr());
    println!("v1.len is {:?}", v1.len());
    println!("v1.capacity is {:?}", v1.capacity());

    // 要素の追加（要素の最後に追加）
    v1.push("C++");
    println!("after v1 : {:?}", v1);

    // 要素の取り出し
    println!("v1[3] is : {}", v1[3]);

    // 要素の取り出し
    // 存在しない値を指定するとパニック発生
    println!("v1.get[3] is : {:?}", v1.get(3));
    println!("v1.get[4] is : {:?}", v1.get(4));
    // println!("v1.[4] is : {:?}", v1[4]);

    // 要素の削除（最後の要素を削除）
    let v2 = v1.pop();
    println!("let v2 = v1.pop() v2 is : {:?}", v2);
    println!("let v2 = v1.pop() v1 is : {:?}", v1);

    // インデックス指定の要素の追加、削除
    // 存在しない値を指定するとパニック発生
    // 要素の追加
    v1.insert(1, "PHP");
    println!("v1.insert(1, \"PHP\") v1 is : {:?}", v1);
    //　要素の削除
    v1.remove(0);
    println!("v1.remove(0) v1 is : {:?}", v1);
}
