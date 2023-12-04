use std::collections::HashMap;

pub fn run() {
    let mut map = HashMap::new();
    // キーと値を格納
    map.insert("Japan", 11);
    map.insert("USA", 3);
    map.insert("Chaina", 1);
    map.insert("India", 2);
    println!("before map is : {:#?}", map);

    // キーが重複した場合、値は上書きされる
    map.insert("Japan", 10);
    println!("after map is : {:#?}", map);

    // キーを指定した値の取り出し（オプション型で出力される）
    println!("map.get(\"USA\") is : {:?}", map.get("USA"));
    // 指定したキーが該当しない場合、Noneが返される
    println!("map.get(\"usa\") is : {:?}", map.get("usa"));

    //　キーを指定したエントリーの削除
    println!("map.remove(\"India\"){:?}", map.remove("India"));
    println!("after map is : {:#?}", map);

    // タプル型を使用したエントリーの出力
    for (k, v) in &map {
        println!("{:?}", k);
        println!("{:?}", v);
    }
}
