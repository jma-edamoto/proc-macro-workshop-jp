// あなたのマクロは、標準ライブラリのごく基本的な要素の名前が呼び出し先のコードで
// 別な意味に書き換えられていても正常に動作するでしょうか？
// Does your macro still work if some of the standard library prelude item names
// mean something different in the caller's code?
//
// 無意味に思われるかもしれませんが、こういう事態は実際に発生します。一番多いパターンは
// "Result"に関するもので、クレート内でResultをそのクレートに固有のエラーを返す単一
//　のパラメータの型に上書き（エイリアス）して利用することがしばしばあります。
//
// マクロによって生成されたコードはResultを2つのパラメータ変数をとる型と考えるため、
// このエイリアスはコードを破壊します。別な例を挙げると、Hyper 0.10ではhyper::Okを
// hyper::status::StatusCode::Okの再エクスポートして扱っていましたが、これはResult::Ok
// とは全く異なります。これによって'use hyper::*'と内部で'Ok'を生成するマクロを同時に
// 使用すると問題が発生します。
//

// 一般的に、第三者に利用される可能性がある全てのマクロは（宣言マクロであろうと手続きマクロであろうと）
// 展開後のコードの中で全てのものをstd::result::Resultのように絶対パスで扱う必要があります。

use derive_builder::Builder;

type Option = ();
type Some = ();
type None = ();
type Result = ();
type Box = ();

#[derive(Builder)]
pub struct Command {
    executable: String,
}

fn main() {}
