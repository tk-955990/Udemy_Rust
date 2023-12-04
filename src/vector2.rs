pub fn run() {
    let v1 = vec!["Rust", "Python", "Java"];
    let v2 = vec!["PHP", "Go"];

    // ベクター型の連結
    let v3 = [v1, v2].concat();
    println!("v3 is : {:?}", v3);

    // ベクター型の分割（インデックス２から分割）
    let (v4, v5) = v3.split_at(2);
    println!("v4 is : {:?}", v4);
    println!("v5 is : {:?}", v5);

    let mut v6 = vec![3, 6, 1, 7, 2];
    // ベクター型の並べ替え（昇順）
    v6.sort();
    println!("v6.sort is : {:?}", v6);
    // ベクター型の並べ替え（降順）
    v6.reverse();
    println!("v5.reverse is : {:?}", v6);

    #[derive(Debug)]
    struct S {
        val1: i32,
        val2: i32,
    }
    let mut v7 = vec![
        S { val1: 3, val2: 4 },
        S { val1: 5, val2: 2 },
        S { val1: 1, val2: 5 },
    ];
    println!("v7 is : {:?}", v7);
    // 構造体のソート
    // 引数ｓのクロージャでキーを設定

    // 昇順
    v7.sort_by_key(|s| s.val1);
    println!("v7.sort_by_key(|s| s.val1);");
    println!("{:?}", v7);
    v7.sort_by_key(|s| s.val2);
    println!("v7.sort_by_key(|s| s.val2);");
    println!("{:?}", v7);
    // 降順
    v7.sort_by_key(|x| std::cmp::Reverse(x.val1));
    println!("v7.sort_by(|x|std::cmp::Reverse(x.val1));");
    println!("{:?}", v7);
    v7.sort_by_key(|x| std::cmp::Reverse(x.val2));
    println!("v7.sort_by(|x|std::cmp::Reverse(x.val2));");
    println!("{:?}", v7);

    // ベクター型の要素の検索（真偽値で結果出力）
    // 指定するインデックスは参照で
    let v8 = vec![3, 6, 1, 7, 2];
    println!("v8.containts(&5) is : {:?}", v8.contains(&5));

    // ベクター型の要素の検索（インデックスの確認）
    // 検索する要素の値は参照外しで
    let x = v8.iter().position(|x| *x == 3);
    println!("x == 3 is : {:?}", x);
    // 検索した要素がなければNone
    let x = v8.iter().position(|x| *x == 5);
    println!("x == 5 is : {:?}", x);
}
