// cargo test -- --test-threads=1 テストを直列処理
// cargo test test_calc_add テスト関数を指定（関数名は部分一致）

// 単体テスト
#[test]
fn test_sample() {
    let a = 1 + 2;
    let b = 3;
    assert_eq!(a, b);
}

fn maybe_panic(flag: bool) {
    if flag == false {
        println!("sefe!");
        // do something
        panic!("unexpected!")
    } else {
        // 想定したテストパニック
        panic!("flag is true!!!");
    }
}
// テストモジュール
// cargo test時のみコンパイルされる
#[cfg(test)]
mod test_module {
    // パニックテスト（flag == true でパニック）
    #[test]
    // 想定したパニックのみをテストするための措置（文字列の部分一致）
    #[should_panic(expected = "flag is true")]
    fn test_maybe_panic() {
        // スコープ外の関数呼び出し
        super::maybe_panic(true);
    }
    #[test]
    // テストを無視
    #[ignore]
    fn test_calc_add() {
        assert_eq!(1 + 1, 2);
    }
    #[test]
    fn test_calc_diff() {
        assert_eq!(1 - 1, 0)
    }
}
pub fn run() {}
