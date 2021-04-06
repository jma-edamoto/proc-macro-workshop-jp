// Field<T>にDebugをインプリメントするにはどんな実装を生成しなければならないか考えましょう。
// この実装には型パラメータTのトレイト境界が含まれるはずです。 
//
// マクロの利用者はDebugを実装していないT型を用いて自由にField<T>をインスタンス化することが
// できるべきです。ただし、そのようなField<T>は生成されたDebug実装のトレイト境界を満たさないため、
// Debugで出力することはできないでしょう。
//
//
// 参考資料:
//
//   - Syn構文木におけるジェネリックの表現:
//     https://docs.rs/syn/1.0/syn/struct.Generics.html
//
//   - impl文にジェネリックを配置するためのヘルパー:
//     https://docs.rs/syn/1.0/syn/struct.Generics.html#method.split_for_impl
//
//   - Synで型パラメータを扱うサンプルコード:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field<T> {
    value: T,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

fn main() {
    let f = Field {
        value: "F",
        bitmask: 0b00011100,
    };

    let debug = format!("{:?}", f);
    let expected = r#"Field { value: "F", bitmask: 0b00011100 }"#;

    assert_eq!(debug, expected);
}
