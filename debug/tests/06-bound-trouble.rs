// ここまでの実装によっては、このテストにコードの変更は必要ないかもしれません。ここでは、前
// のテストで言及した`#field_ty: Trait`というトレイト境界がなぜ不適切なのかを実演します。
//
//     #[derive(CustomDebug)]
//     pub struct One<T> {
//         value: T,
//         two: Option<Box<Two<T>>>,
//     }
//
//     #[derive(CustomDebug)]
//     struct Two<T> {
//         one: Box<One<T>>,
//     }
//
// 問題のあるコードは以下のように展開されます:
//
//     impl<T> Debug for One<T>
//     where
//         T: Debug,
//         Option<Box<Two<T>>>: Debug,
//     {...}
//
//     impl<T> Debug for Two<T>
//     where
//         Box<One<T>>: Debug,
//     {...}
//
// このコードには２つの問題があります。まず、このコードに関連する `impl<T> Debug for 
// Option<T> where T: Debug` と `impl<T> Debug for Box<T> where T: ?Sized +Debug`
// という標準ライブラリの実装を考えると、以下のような循環定義が存在することがわかります。
//
//   - One<T>がDebugを実装しているなら、Box<One<T>>もDebugを実装している;
//   - Box<One<T>>がDebugを実装しているなら、Two<T>もDebugを実装している;
//   - Two<T>がDebugを実装しているなら、Box<Two<T>>もDebugを実装している;
//   - Box<Two<T>>がDebugを実装しているなら、Option<Box<Two<T>>>もDebugを実装している;
//   - Option<Box<Two<T>>>がDebugを実装しているなら、One<T>もDebugを実装している;　循環！
//
// こういう「どこから現れたのかわからない」型の実装を禁止するために、Rustのコンパイラは循環を
// 検出して拒否し、以下のようなエラーを返します：
//
//     error[E0275]: overflow evaluating the requirement `One<u8>: std::fmt::Debug`
//      -->
//       |     assert_debug::<One<u8>>();
//       |     ^^^^^^^^^^^^^^^^^^^^^^^
//
// 将来的には双帰納推論（co-inductive reasoning）と呼ばれるテクニックを用いてRustコンパイラ
// のトレイトソルバーがこのような循環を処理できるようになるかもしれませんが、この双帰納性をRust
// のトレイト実装に適用した際に何らかの不具合が発生する可能性についてはいまだに未知数です。今の
// ところこの問題に動きはありませんが、Githubには"#[derive] sometimes uses incorrect bounds"
// というissueが存在しています：
// https://github.com/rust-lang/rust/issues/26925
//
// 二つ目の問題は、private-in-public違反です:
//
//     error[E0446]: private type `Two<T>` in public interface
//      -->
//       |   struct Two<T> {
//       |   - `Two<T>` declared as private
//     ...
//       | / impl<T> Debug for One<T>
//       | | where
//       | |     T: Debug,
//       | |     Option<Box<Two<T>>>: Debug,
//     ... |
//       | | }
//       | |_^ can't leak private type
//
// RustのpublicなAPIはprivateな型を用いて定義することができません。PublicなAPIはpublicな引数型
// を持ちpublicな関数呼び出しを返しますが、同時にpublicな型に対するpublicなトレイトの実装に含まれる
// トレイト境界もpublicなものとして返すのです。

use derive_debug::CustomDebug;
use std::fmt::Debug;

#[derive(CustomDebug)]
pub struct One<T> {
    value: T,
    two: Option<Box<Two<T>>>,
}

#[derive(CustomDebug)]
struct Two<T> {
    one: Box<One<T>>,
}

fn assert_debug<F: Debug>() {}

fn main() {
    assert_debug::<One<u8>>();
    assert_debug::<Two<u8>>();
}
