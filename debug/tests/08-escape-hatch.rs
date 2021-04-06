// マクロ展開の際に得られる情報からトレイト境界を推論するのに、経験則だけでは不十分な場
// 合があります。このような場合には属性を利用し、マクロの利用者に正しいトレイト境界を手
// 書きしてもらうことにします。
//
// 以下のコードにあるWrapper<T>の実装には `debug(bound = "...")` で与えられたトレ
// イト境界が必要です。このような属性が存在する場合には、同時にトレイト境界に関する全て
// の推論を中止して本来推論される `T: Debug` の生成を防ぎます。 
//
//     impl<T: Trait> Debug for Wrapper<T>
//     where
//         T::Value: Debug,
//     {...}
//
// このテストには含まれていませんが、追加として `debug(bound = "...")` を個々のフィ
// ールドの属性としても受け入れられるようにしてください。この属性はそれが付与されたフィ
// ールドの型に対応するトレイト境界だけを代替し、それ以外のフィールドについては型から推
// 論されるトレイト境界を生成する必要があります。
//
//     #[derive(CustomDebug)]
//     pub struct Wrapper<T: Trait, U> {
//         #[debug(bound = "T::Value: Debug")]
//         field: Field<T>,
//         normal: U,
//     }

use derive_debug::CustomDebug;
use std::fmt::Debug;

pub trait Trait {
    type Value;
}

#[derive(CustomDebug)]
#[debug(bound = "T::Value: Debug")]
pub struct Wrapper<T: Trait> {
    field: Field<T>,
}

#[derive(CustomDebug)]
struct Field<T: Trait> {
    values: Vec<T::Value>,
}

fn assert_debug<F: Debug>() {}

fn main() {
    struct Id;

    impl Trait for Id {
        type Value = u8;
    }

    assert_debug::<Wrapper<Id>>();
}
