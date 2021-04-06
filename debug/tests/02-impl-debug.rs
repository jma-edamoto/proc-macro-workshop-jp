// 名前付きフィールドを持ちジェネリックな型パラメータを持たない基本的な構造体に対して
// std::fmt::Debugの実装を生成してください。

// deriveマクロの名前は、そのマクロによって実装されるトレイトと無関係であることに留意
// してください。ここではマクロの名前をCustomDebugとしていますが、それによって生成さ
// れるのはDebugに対する実装です。慣例として、通常deriveマクロはマクロと同じ名前の
// トレイトを実装します。
//
//
// 参考資料:
//
//   - Debugトレイト:
//     https://doc.rust-lang.org/std/fmt/trait.Debug.html
//
//   - 構造体を正しくフォーマットするDebugStructヘルパー:
//     https://doc.rust-lang.org/std/fmt/struct.DebugStruct.html

use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    bitmask: u8,
}

fn main() {
    let f = Field {
        name: "F",
        bitmask: 0b00011100,
    };

    let debug = format!("{:?}", f);

    assert!(debug.starts_with(r#"Field { name: "F","#));
}
