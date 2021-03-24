// マクロにビルダーの状態を保持する構造体と、ビルダーの空のインスタンスを返すbuilder関数
// を生成させて下さい。
//
// 一番簡単な方法は、次のコードを生成することです（ただし、この型名は呼び出し元から
// 与えられた入力に一致する必要があります）
//
//     impl Command {
//         pub fn builder() {}
//     }
//
// この時点ではビルダーに対して何の操作も行っていないため、ビルダーの型として'()'を返す
// この関数はテストをクリアします。
//
// 次のテストに進む前に、マクロに以下の構造体を生成させて下さい:
//
//     pub struct CommandBuilder {
//         executable: Option<String>,
//         args: Option<Vec<String>>,
//         env: Option<Vec<String>>,
//         current_dir: Option<String>,
//     }
//
// また、builder関数として以下のコードを生成するように修正して下さい:
//
//     impl Command {
//         pub fn builder() -> CommandBuilder {
//             CommandBuilder {
//                 executable: None,
//                 args: None,
//                 env: None,
//                 current_dir: None,
//             }
//         }
//     }
//
//
// 参考資料:
//
//   - マクロの出力をまとめるQuoteクレート:
//     https://github.com/dtolnay/quote
//
//   - 入力の型名と"Builder"を結合してビルダーの型名を作る方法:
//     https://docs.rs/syn/1.0/syn/struct.Ident.html

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let builder = Command::builder();

    let _ = builder;
}
