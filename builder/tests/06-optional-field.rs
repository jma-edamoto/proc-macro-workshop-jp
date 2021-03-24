// フィールドの中には、値がセットされている必要がないものがあります。
// 元の構造体のフィールドがOption<T>で表現されている場合がそれです。
//
// 与えられた構造体のフィールドがOption型を持っているかを識別し、対応するbuilder
// メソッドがoptionalになるようマクロを修正して下さい。今回のテストではcurrent_dir
// がOption型を持っているためこのままではテストを通過出来ません。
//
// ラストのコンパイラはマクロが完全に展開された後で名前解決を行うことに注意して下さい。
// 言い換えると、手続きマクロを処理している途中では"型"は存在せず、構文木のトークンだ
// けが存在しています。一般的に、多くの異なるトークン表現が最終的には同じ型を表します。
// 例えば、"Option<T>"と"std::option::Option<T>"と"<Vec<Option<T>> as IntoIterator>::Item"
// はいずれも異なる名前の同じ型です。逆に、同じトークンであっても出現する場所によっては
// 異なる型を表す場合があります。例えば、"Error"の意味はそれを取り囲むスコープでインポートされて
// いるのがstd::error::Errorかstd::io::Errorかに依存します。ですので、一般論として
// 二つのトークン表現が同じ型を意味しているか否かをマクロ自身が判定することは不可能です。
//
// 今回のテストで言うと、この事実は「あるフィールドが名前解決の後で最終的にOptional型に
// なるかをマクロが判定出来るようなコンパイル表現は存在しない」ということを意味します。
// 我々が得られるのはユーザーがコードの中で型を記述するのに用いたトークンだけです。
// 必然的に、マクロは型が"Option<...>"という文字列で書かれたフィールドを探すことになり、
// Option型が違う方法で記述されている場合にはその存在に気付かない可能性があります。
//
// Rustが多様な文法を備えているために、トークンからパースされた構文木はやや複雑な構造を
// 持っています。マクロが判定する必要があるデータは以下のような入れ子構造のものになります。
// 
//     Type::Path(
//         TypePath {
//             qself: None,
//             path: Path {
//                 segments: [
//                     PathSegment {
//                         ident: "Option",
//                         arguments: PathArguments::AngleBracketed(
//                             AngleBracketedGenericArguments {
//                                 args: [
//                                     GenericArgument::Type(
//                                         ...
//                                     ),
//                                 ],
//                             },
//                         ),
//                     },
//                 ],
//             },
//         },
//     )

use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .build()
        .unwrap();
    assert!(command.current_dir.is_none());

    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .current_dir("..".to_owned())
        .build()
        .unwrap();
    assert!(command.current_dir.is_some());
}
