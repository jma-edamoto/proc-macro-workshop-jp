// いくつかのジェネリック型はその型パラメータがDebugを実装していなくてもそれ自体がDebug
// を実装している場合があります。一例をあげると、PhantomDataは以下の実装を持ちます：
//
//     impl<T: ?Sized> Debug for PhantomData<T> {...}
//
// こういう状況に対処する一つの方法は、それぞれのジェネリックパラメータに対して`#param: Debug`
// というトレイト境界を生成する代わりに、それぞれのフィールドに対して`#field_ty: Debug`を生成
// することです。下のテストにあるField<T>構造体でいうと、このようなコードを生成することになりま
// す：
//
//     impl<T> Debug for Field<T>
//     where
//         PhantomData<T>: Debug,
//     {...}
//
// 今後のテストでわかりますが、このアプローチには致命的な欠陥があります。
//
// その代わりに我々はPhantomDataを十分に広く利用されているものと考えてこれを特例として扱い、
// その他のアプリケーションに固有の「特例」のためにトレイト境界を上書きする「脱出口」を今後の
// 別な作業で提供することにします。
//
// 具体的には、入力に含まれるすべての型パラメータ #param に対してそれぞれがPahtomDataの中で 
// だけ言及されるものか否かを判定し、そうである場合にはそのパラメータに対するトレイト境界
// `#param: Debug`の生成を回避します。テストを通過するためにはPhantomData<#param>型の
// フィールドを探すだけで十分ですが、現実にはマクロの利用者が最終的にたとえばPhantomData
// <&'a #param>のようなコード片になる何かをコードに配置する可能性にまで気を配る必要があり
// ます。
//
// 我々が経験則の領域に足を踏み入れたことに留意してください。Rustのマクロシステムはderive
// マクロの中で正しいトレイト境界を推論することができません。正しい推論を行うためには名前解
// 決が必要で、それはつまり、マクロがあるフィールドの型にどんなトレイト境界が対応するかを名
// 前から判定する能力を持つことを意味します。Rustコンパイラは名前解決の前に全てのマクロを展
// 開することを選択しました（ただし、マクロそれ自体の名前だけは通常より制限された方法で解決
// されます）。
//
// このマクロ展開と名前解決の明確な分離には手続きマクロが型に関する情報を扱えないという欠点
// をはるかに上回る利点があるため、今後も変更される見込みはありません。代替措置として、マク
// ロは型についての不可欠な情報の代わりにドメインに固有の経験則や「脱出口」を利用するか、よ
// り一般的には名前解決をRustのトレイトシステムに委譲します。以下のマクロ宣言で入力に含まれ
// る "S" という単語がString型を表すことを知る手段が存在しないにも関わらず、展開されたコー
// ドではString型のDebug実装が正しく呼び出される点には特に注意を払ってください。

use derive_debug::CustomDebug;
use std::fmt::Debug;
use std::marker::PhantomData;

type S = String;

#[derive(CustomDebug)]
pub struct Field<T> {
    marker: PhantomData<T>,
    string: S,
    #[debug = "0b{:08b}"]
    bitmask: u8,
}

fn assert_debug<F: Debug>() {}

fn main() {
    // Does not implement Debug.
    struct NotDebug;

    assert_debug::<PhantomData<NotDebug>>();
    assert_debug::<Field<NotDebug>>();
}
