// ビルダーから元の構造体を返すbuildメソッドを生成して下さい.
//
// このメソッドは構造体の全てのフィールドに値がセットされていることを要求します。
// 値がセットされていないフィールドがある場合にはエラーを返す必要があります。
// エラーの型は重要ではありませんので、Box<dyn Error>のFrom<String>インプリメ
// ンテーションを利用して下さい。
//
//     impl CommandBuilder {
//         pub fn build(&mut self) -> Result<Command, Box<dyn Error>> {
//             ...
//         }
//     }

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

fn main() {
    let mut builder = Command::builder();
    builder.executable("cargo".to_owned());
    builder.args(vec!["build".to_owned(), "--release".to_owned()]);
    builder.env(vec![]);
    builder.current_dir("..".to_owned());

    let command = builder.build().unwrap();
    assert_eq!(command.executable, "cargo");
}
