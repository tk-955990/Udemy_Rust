use std::collections::HashSet;

pub fn run() {
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(1);
    set1.insert(1);
    // 重複する値は一つのみ追加される
    println!("before set1 is : {:?}", set1);

    set1.insert(2);
    set1.insert(3);
    set1.insert(4);
    // 値の順番に保証が無い
    println!("afterf set1 is : {:?}", set1);
    // 指定した値の検索、削除（指定した値があるかどうかは真偽値で出力される）
    println!("set1.contains(&2) set1 is : {}", set1.contains(&2));
    println!("set1.remove(&2) set1 is : {:?}", set1.remove(&2));
    println!("afterf set1 is : {:?}", set1);

    let mut set2 = HashSet::new();
    set2.insert(1);
    set2.insert(2);
    set2.insert(3);
    set2.insert(5);
    println!("set2 is : {:?}", set2);

    // 和集合
    let set3 = &set1 | &set2;
    println!("&set1 | &set2 set3 is : {:?}", set3);
    // 積集合
    let set4 = &set1 & &set2;
    println!("&set1 & &set2 set4 is : {:?}", set4);
    // 差集合
    let set5 = &set1 - &set2;
    println!("&set1 - &set2 set5 is : {:?}", set5);
    // 排他的論理和
    let set6 = &set1 ^ &set2;
    println!("&set1 ^ &set2 set6 is : {:?}", set6);
}
