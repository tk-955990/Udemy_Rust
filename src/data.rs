pub fn run() {
    // タプル型(型は不問、固定長型)
    let t1 = (3, 5.6, "Hello", true);
    println!("{:?}", t1);
    // 要素取り出し
    let (a, _x, _y, _z) = t1;
    println!("a is: {}", a);
    let mut a = "hello";
    println!("The first a Stack memory address is: {:p}", &a);
    println!("The first a Heap memory is: {:?}", a.as_ptr());
    a = "world";
    println!("The first a Stack memory address is: {:p}", &a);
    println!("The first a Heap memory is: {:?}", a.as_ptr());

    // 配列型(型は同一、不変型)
    let l1 = [2, 3, 4, 5];
    println!("The l1 is: {:?}", l1);
    let l2 = [8; 9];
    println!("The l2 is: {:?}", l2);
    // 要素取り出し
    let l3 = &l1[0..];
    println!("The l3 is:{:?},", l3);

    // ベクター型(型は同一、可変長型)
    for value in l1.iter() {
        println!("{}", value);
    }

    let mut v1 = vec![1, 2, 3, 4, 5];
    println!("The v1 is: {:?}", v1);
    // 要素追加
    v1.push(9);
    v1.push(8);
    v1.push(7);
    println!("The v1 is: {:?}", v1);
    let x = v1.pop();
    println!("The x is: {:?}", x);
    println!("The v1 is: {:?}", v1);
    let x = v1.pop();
    println!("The x is: {:?}", x);
    println!("The v1 is: {:?}", v1);

    // 要素取り出し
    let y = v1[4];
    println!("The v1[4] is:{:?}", y);
    // let y = v1[6];
    // println!("The v1[6] is:{:?}", y);
    // 要素取り出し(getメソッド使用)
    // 要素が無い場合は"None"を返す
    let z = v1.get(6);
    println!("The v1.get(6) is:{:?}", z);
    let z = &v1[..];
    println!("The &v1[..] is:{:?}", z);

    // 文字型
    let _c1 = 'a';
    let _c2 = '&';
    let c3 = '😀';
    println!("The c3 is:{:?}", c3);

    // 文字列型
    let s1 = "Rust";
    // String型への変換
    let _s2 = String::from("Python");
    let _s3 = "Java".to_string();

    let mut s4 = String::from("Hello");
    // 文字列の連結
    s4.push_str(" World!");
    println!("The before s4 is: {}", s4);
    // 文字列の削除
    s4.truncate(s4.len() - " World!".len());
    println!("The after s4 is: {}", s4);
    // formatマクロ　新しい文字列の作成(作成のみ、出力はしない)
    let s5 = format!("{} {}!", s4, s1);
    println!("The s5 is: {}", s5);
}
