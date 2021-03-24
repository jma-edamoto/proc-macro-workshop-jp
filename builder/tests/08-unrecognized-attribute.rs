// ユーザーが属性の綴りを間違えた場合に、マクロが適切なエラーを返すことを確認してください。
// これは、コンパイル失敗のテストです。 
//
// 手続きマクロでエラーを返すのに推奨される方法は、マクロが生成するコードの中で標準ライブラリ
// のcompile_errorマクロを呼び出すことです。
//
// 参考資料:
//
//   - 簡単なカスタムエラーを出力するcompile_errorマクロ:
//     https://doc.rust-lang.org/std/macro.compile_error.html
//
//   - syn::Errorをcompile_errorの呼び出しに変換する方法:
//     https://docs.rs/syn/1.0/syn/struct.Error.html#method.to_compile_error

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(eac = "arg")]
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {}
