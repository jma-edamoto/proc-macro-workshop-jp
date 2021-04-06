// それぞれのフィールドについて#[debug = "..."]というフィールド属性を探し、存在する場合には
// 属性に与えられたフォーマット文字列に従ってフィールドをフォーマットしてください。
//
// この属性がderiveマクロに関連付けられていることをコンパイラに知らせるために、deriveマクロ
// のエントリーポイントで宣言する必要があります。
//
//     #[proc_macro_derive(CustomDebug, attributes(debug))]
//
// この属性は「内部属性」と呼ばれます. 「内部」という言葉はこの属性がそれ自体ではマクロの呼び
// 出しに対応せず、他のマクロ呼び出しの中で利用されることを示しています。
//
//
// 参考資料
//
//   - 関連する構文木:
//     https://docs.rs/syn/1.0/syn/struct.Attribute.html
//     https://docs.rs/syn/1.0/syn/enum.Meta.html
//
//   - ランタイムな値に対してフォーマット文字列を適用するマクロ:
//     https://doc.rust-lang.org/std/macro.format_args.html

use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

fn main() {
    let f = Field {
        name: "F",
        bitmask: 0b00011100,
    };

    let debug = format!("{:?}", f);
    let expected = r#"Field { name: "F", bitmask: 0b00011100 }"#;

    assert_eq!(debug, expected);
}
