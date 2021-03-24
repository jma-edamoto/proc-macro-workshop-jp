// 標準ライブラリのstd::process::Commandビルダーは、argsメソッドによってベクターを
// を丸ごと引数に取るだけではなく、もう少し便利な方法で引数を受け取ることができます。
//
// マクロを修正して、それぞれのフィールドについて#[builder(each = "...")]という属性の
// 有無を判定し、その属性を持つフィールドに対しては型がVecであることを仮定して、属性の
// 中で与えられた単語をメソッド名としてVecに1つずつ要素を追加できるようにして下さい。
//
// あなたのマクロが"builder"という属性を利用することをコンパイラに示すために、deriveマクロ
// の最初でそのことを宣言する必要があります。さもないと、コンパイラは認識されない属性値
// ということでエラーを返します。
//
//     #[proc_macro_derive(Builder, attributes(builder))]
//
// これらの属性はinert attributes(不活性な属性)と呼ばれます. 「不活性」という言葉はこの属性が
// マクロ呼び出しとは対応せず、別なマクロ呼び出しの中で利用されることを示しています。
//　
// なお、もしこの「1つずつ要素を追加する」メソッドがフィールドと同じ名前で与えられた場合、
// 通常の「ベクターを丸ごと引数に取る」メソッドは生成しないようにしないと名前の衝突が発生します。
//
// 参考資料:
//
//   - 関連する構文木:
//     https://docs.rs/syn/1.0/syn/struct.Attribute.html
//     https://docs.rs/syn/1.0/syn/enum.Meta.html

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    #[builder(each = "env")]
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .arg("build".to_owned())
        .arg("--release".to_owned())
        .build()
        .unwrap();

    assert_eq!(command.executable, "cargo");
    assert_eq!(command.args, vec!["build", "--release"]);
}
