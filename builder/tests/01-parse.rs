// このテストは正しい名前のderiveマクロが存在することを検証します。ですので、
// マクロによって具体的なコードを生成する必要は無く、空のTokenStreamを返す
// だけで十分です。
//
// まず最初に、deriveマクロに与えられた入力をsyn::DeriveInput型の構文木にパースして下さい。
//
// 少し時間を使って、doc.rsでsyn::DeriveInput構造体の中を探検してみて下さい。
// 各フィールドのリンクをクリックし、マクロを動かすのに必要な情報がどこから得られるかを
// 確認して下さい。
//
//
// 参考資料
//
//   - 手続きマクロの入力をパースするSynクレート:
//     https://github.com/dtolnay/syn
//
//   - deriveマクロの入力を表現するDeriveInput構文木:
//     https://docs.rs/syn/1.0/syn/struct.DeriveInput.html
//
//   - Synを使ったderiveマクロの実装例:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {}
