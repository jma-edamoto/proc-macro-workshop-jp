// このテストは正しい名前のderiveマクロが存在することを検証します。ですので、
// マクロによって具体的なコードを生成する必要は無く、空のTokenStreamを返す
// だけで十分です。
//
// 次に進む前に、deriveマクロに与えられた入力をsyn::DeriveInput型の構文木にパースして下さい。
//
//
// 参考資料
//
//   - deriveマクロの入力を表現するDeriveInput構文木:
//     https://docs.rs/syn/1.0/syn/struct.DeriveInput.html
//
//   - Synを使ったderiveマクロの実装例:
//     https://github.com/dtolnay/syn/tree/master/examples/heapsize

use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    bitmask: u16,
}

fn main() {}
