# Rust Latam: 手続きマクロワークショップ

*Rustの手続きマクロ（コードを生成するコード）の学習用にデザインされたプロジェクトです。*

*このリポジトリは[dtolnay/proc-macro-workshop]の抄訳版です。全ての権利はオリジナルの
作者に帰属します。*

[dtolnay/proc-macro-workshop]: https://github.com/dtolnay/proc-macro-workshop

*ここにあるプロジェクトはどれも、実際の用例に則した実用的なものです。5つのプロジェクトの
うち3つはオリジナルの作者でdtolnay氏が業務のために作成した物で、後の2つは別な方が作成した
crate.ioにも登録されているライブラリです。*

<br>

## コンテンツ

- [**推奨される前提知識**](#推奨される前提知識)
- [**プロジェクト**](#プロジェクト) — それぞれのプロジェクトの紹介
  - [**Derive macro:** `derive(Builder)`](#derive-macro-derivebuilder)
  - [**Derive macro:** `derive(CustomDebug)`](#derive-macro-derivecustomdebug)
  - [**Function-like macro:** `seq!`](#function-like-macro-seq)
  - [**Attribute macro:** `#[sorted]`](#attribute-macro-sorted)
  - [**Attribute macro:** `#[bitfield]`](#attribute-macro-bitfield)
  - [**Project recommendations**](#project-recommendations) — What to work on
    depending on your interests
- [**Test harness**](#test-harness) — Explanation of how testing is set up
- [**作業の手順**](#作業の手順) — Recommended way to work through the workshop
- [**デバッグに関するtips**](#デバッグに関するtips)

<br>

## 推奨される前提知識

このワークショップでは属性マクロ、deriveマクロ、関数風マクロを扱います。

プロジェクト内のコンテンツや解説は全て、Rustの構造体、列挙体、トレイト、トレイト実装、ジェネリックなパラメータ、およびトレイト境界に関する知識を前提としています。
それらの知識が無い方でもワークショップに取り組んでいただくことは可能ですが、マクロそのものよりも上に挙げた知識の方が遙かに容易に習得出来ることはすぐにお分かりいただ
けるでしょう。

<br>

## プロジェクト

ここではそれぞれのプロジェクトについて紹介した後、最後にプロジェクトに取り組む順番
について皆さんの興味に基づいたお勧めのやり方を述べます。各プロジェクトはここで紹介
する以上の深い内容を含んでいることに留意して下さい。

### Derive macro: `derive(Builder)`

This macro generates the boilerplate code involved in implementing the [builder
pattern] in Rust. Builders are a mechanism for instantiating structs, especially
structs with many fields, and especially if many of those fields are optional or
the set of fields may need to grow backward compatibly over time.

[builder pattern]: https://en.wikipedia.org/wiki/Builder_pattern

There are a few different possibilities for expressing builders in Rust. Unless
you have a strong pre-existing preference, to keep things simple for this
project I would recommend following the example of the standard library's
[`std::process::Command`] builder in which the setter methods each receive and
return `&mut self` to allow chained method calls.

[`std::process::Command`]: https://doc.rust-lang.org/std/process/struct.Command.html

Callers will invoke the macro as follows.

```rust
use derive_builder::Builder;

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
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
}
```

This project covers:

- traversing syntax trees;
- constructing output source code;
- processing helper attributes to customize the generated code.

*Project skeleton is located under the <kbd>builder</kbd> directory.*

### Derive macro: `derive(CustomDebug)`

This macro implements a derive for the standard library [`std::fmt::Debug`]
trait that is more customizable than the similar `Debug` derive macro exposed by
the standard library.

[`std::fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html

In particular, we'd like to be able to select the formatting used for individual
struct fields by providing a format string in the style expected by Rust string
formatting macros like `format!` and `println!`.

```rust
use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: String,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}
```

Here, one possible instance of the struct above might be printed by its
generated `Debug` impl like this:

```console
Field { name: "st0", bitmask: 0b00011100 }
```

This project covers:

- traversing syntax trees;
- constructing output source code;
- processing helper attributes;
- dealing with lifetime parameters and type parameters;
- inferring trait bounds on generic parameters of trait impls;
- limitations of derive's ability to emit universally correct trait bounds.

*Project skeleton is located under the <kbd>debug</kbd> directory.*

### Function-like macro: `seq!`

This macro provides a syntax for stamping out sequentially indexed copies of an
arbitrary chunk of code.

For example our application may require an enum with sequentially numbered
variants like `Cpu0` `Cpu1` `Cpu2` ... `Cpu511`. But note that the same `seq!`
macro should work for any sort of compile-time loop; there is nothing specific
to emitting enum variants. A different caller might use it for generating an
expression like `tuple.0 + tuple.1 + ... + tuple.511`.

```rust
use seq::seq;

seq!(N in 0..512 {
    #[derive(Copy, Clone, PartialEq, Debug)]
    pub enum Processor {
        #(
            Cpu#N,
        )*
    }
});

fn main() {
    let cpu = Processor::Cpu8;

    assert_eq!(cpu as u8, 8);
    assert_eq!(cpu, Processor::Cpu8);
}
```

This project covers:

- parsing custom syntax;
- low-level representation of token streams;
- constructing output source code.

*Project skeleton is located under the <kbd>seq</kbd> directory.*

### Attribute macro: `#[sorted]`

A macro for when your coworkers (or you yourself) cannot seem to keep enum
variants in sorted order when adding variants or refactoring. The macro will
detect unsorted variants at compile time and emit an error pointing out which
variants are out of order.

```rust
#[sorted]
#[derive(Debug)]
pub enum Error {
    BlockSignal(signal::Error),
    CreateCrasClient(libcras::Error),
    CreateEventFd(sys_util::Error),
    CreateSignalFd(sys_util::SignalFdError),
    CreateSocket(io::Error),
    DetectImageType(qcow::Error),
    DeviceJail(io_jail::Error),
    NetDeviceNew(virtio::NetError),
    SpawnVcpu(io::Error),
}
```

This project covers:

- compile-time error reporting;
- application of visitor pattern to traverse a syntax tree;
- limitations of the currently stable macro API and some ways to work around
  them.

*Project skeleton is located under the <kbd>sorted</kbd> directory.*

### Attribute macro: `#[bitfield]`

This macro provides a mechanism for defining structs in a packed binary
representation with access to ranges of bits, similar to the language-level
support for [bit fields in C].

[bit fields in C]: https://en.cppreference.com/w/cpp/language/bit_field

The macro will conceptualize one of these structs as a sequence of bits 0..N.
The bits are grouped into fields in the order specified by a struct written by
the caller. The `#[bitfield]` attribute rewrites the caller's struct into a
private byte array representation with public getter and setter methods for each
field.

The total number of bits N is required to be a multiple of 8 (this will be
checked at compile time).

For example, the following invocation builds a struct with a total size of 32
bits or 4 bytes. It places field `a` in the least significant bit of the first
byte, field `b` in the next three least significant bits, field `c` in the
remaining four most significant bits of the first byte, and field `d` spanning
the next three bytes.

```rust
use bitfield::*;

#[bitfield]
pub struct MyFourBytes {
    a: B1,
    b: B3,
    c: B4,
    d: B24,
}
```

```text
                               least significant bit of third byte
                                 ┊           most significant
                                 ┊             ┊
                                 ┊             ┊
║  first byte   ║  second byte  ║  third byte   ║  fourth byte  ║
╟───────────────╫───────────────╫───────────────╫───────────────╢
║▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒ ▒║
╟─╫─────╫───────╫───────────────────────────────────────────────╢
║a║  b  ║   c   ║                       d                       ║
                 ┊                                             ┊
                 ┊                                             ┊
               least significant bit of d         most significant
```

The code emitted by the `#[bitfield]` macro for this struct would be as follows.
Note that the field getters and setters use whichever of `u8`, `u16`, `u32`,
`u64` is the smallest while being at least as large as the number of bits in
the field.

```rust
impl MyFourBytes {
    // Initializes all fields to 0.
    pub fn new() -> Self;

    // Field getters and setters:
    pub fn get_a(&self) -> u8;
    pub fn set_a(&mut self, val: u8);
    pub fn get_b(&self) -> u8;
    pub fn set_b(&mut self, val: u8);
    pub fn get_c(&self) -> u8;
    pub fn set_c(&mut self, val: u8);
    pub fn get_d(&self) -> u32;
    pub fn set_d(&mut self, val: u32);
}
```

This project covers:

- traversing syntax trees;
- processing helper attributes;
- constructing output source code;
- interacting with traits and structs other than from the standard library;
- techniques for compile-time assertions that require type information, by
  leveraging the trait system in interesting ways from generated code;
- tricky code.

*Project skeleton is located under the <kbd>bitfield</kbd> directory.*

### Project recommendations

If this is your first time working with procedural macros, I would recommend
starting with the `derive(Builder)` project. This will get you comfortable with
traversing syntax trees and constructing output source code. These are the two
fundamental components of a procedural macro.

After that, it would be equally reasonable to jump to any of
`derive(CustomDebug)`, `seq!`, or `#[sorted]`.

- Go for `derive(CustomDebug)` if you are interested in exploring how macros
  manipulate trait bounds, which is one of the most complicated aspects of
  code generation in Rust involving generic code like [Serde]. This project
  provides an approachable introduction to trait bounds and digs into many of
  the challenging aspects.

- Go for `seq!` if you are interested in parsing a custom input syntax yourself.
  The other projects will all mostly rely on parsers that have already been
  written and distributed as a library, since their input is ordinary Rust
  syntax.

- Go for `#[sorted]` if you are interested in generating diagnostics (custom
  errors) via a macro. Part of this project also covers a different way of
  processing input syntax trees; the other projects will do most things through
  `if let`. The visitor approach is better suited to certain types of macros
  involving statements or expressions as we'll see here when checking that
  `match` arms are sorted.

[Serde]: https://serde.rs/

I would recommend starting on `#[bitfield]` only after you feel you have a
strong grasp on at least two of the other projects. Note that completing the
full intended design will involve writing at least one of all three types of
procedural macros and substantially more code than the other projects.

<br>

## Test harness

Testing macros thoroughly tends to be tricky. Rust and Cargo have a built-in
testing framework via `cargo test` which can work for testing the success cases,
but we also really care that our macros produce good error message when they
detect a problem at compile time; Cargo isn't able to say that failing to
compile is considered a success, and isn't able to compare that the error
message produced by the compiler is exactly what we expect.

The project skeletons in this repository use an alternative test harness called
[trybuild].

[trybuild]: https://github.com/dtolnay/trybuild

<p align="center">
<a href="#test-harness">
<img src="https://user-images.githubusercontent.com/1940490/55197640-eb390080-5191-11e9-8c1f-1183935c0c26.png" width="600">
</a>
</p>

The test harness is geared toward iterating on the implementation of a
procedural macro, observing the errors emitted by failed executions of the
macro, and testing that those errors are as expected.

<br>

## 作業の手順

５つのプロジェクトにはそれぞれ<kbd>tests</kbd>ディレクトリ内にテストスートが一式用意されています（テストの追加や削除、あなたの実装に合わせたテストの変更等は自由に行って下さい）。各プロジェクトのトップのディレクトリ内で`cargo test`を実行し、テストを走らせて下さい。

全てのプロジェクトは、テストを全て無効にした状態からスタートします。*tests/progress.rs*を開いてテスト項目を１つずつ有効にし、それを通過するように実装を行うことを繰り返して下さい。**それぞれのテストファイル（例えば*tests/01-parse.rs*）には
今からテストしようとしている機能と、それを実装するためのTipsがコメントで記載されています。**テストの番号順に、１つずつ作業を進めることを推奨します。

テストには正しくコンパイル出来て動作することを確認するためのものと、コンパイルに失敗し適切なエラーメッセージを返すことを確認するためのものの２種類があります。もし正しくコンパイル出来て動作すべきテストが失敗すれば、テストランナーはコンパイルエラーかランタイムエラーを返します。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197637-eb390080-5191-11e9-9197-5832071639ea.png" width="600">
</a>
</p>

コンパイルの失敗を確認するテストの場合には、期待されるエラーメッセージを記したファイルをコンパイラの出力結果と比較します。両者が一致すればテストは通過したと判断されます。一致しない場合、テストランナーは期待されるエラーメッセージと実際の出力を返します。
期待されるエラーが書かれたファイルは、対応するテストのファイル名の拡張子を _*.rs_. から _*.stderr_　に変えた物になります。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197639-eb390080-5191-11e9-9c8f-a47cab89652d.png" width="600">
</a>
</p>

コンパイルに失敗することを期待するテストに _*.stderr_ ファイルが用意されていない場合、テストランナーはコンパイラの出力を<kbd>tests</kbd>ディレクトリと同じ階層の<kbd>wip</kbd>というディレクトリに保存します。ですので _*.stderr_ ファイルを更新する場合は一度ファイルを削除したあとでテストを実行し、*wip*ディレクトリに出力された新しい _*.stderr_ ファイルを*tests*に移動してください。

<p align="center">
<a href="#workflow">
<img src="https://user-images.githubusercontent.com/1940490/55197642-ebd19700-5191-11e9-8f00-2d7c5f4be1a9.png" width="600">
</a>
</p>

<br>

## デバッグに関するtips

マクロが生成したコードを確認するには[cargo expand]をインストールし、このリポジトリのルート（各プロジェクトのディレクトリの一つ上）で`cargo expand`を実行してmain.rsを展開します。このmain.rsにテストの内容をコピーして調整を行って下さい。

[cargo expand]: https://github.com/dtolnay/cargo-expand

マクロが文法的に誤ったコード（変数型の問題とは限りません）を生成する場合には、cargo expandでコードを確認することが出来ません。代わりに、マクロがトークンストリームを返す前にその内容を標準エラーに出力させて下さい。

```rust
eprintln!("TOKENS: {}", tokens);
```

`cargo test`（main.rsを利用している場合には`cargo check`）を実行すると、コンパイラはマクロを展開する間にこの内容を表示してくれます。Stderr はマクロが入力からパースした構文木の構造を確認するのに便利な方法です。

```rust
eprintln!("INPUT: {:#?}", syntax_tree);
```

Synの構文木をデバッグ出力するためには、Synの依存関係として`features = ["extra-traits"]`をセットする必要があります。これは数百のデバッグ用実装をSynに追加するには相応のコンパイル時間が必要で、それはマクロの開発時にだけ必要だからです。


<br>

### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this codebase by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
