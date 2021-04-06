// このテストではトレイト境界を推論するderiveマクロでしばしば有用な、もう一つの経験則を
// 扱います。
//
// 生成される実装は、このような形になる必要があります:
//
//     impl<T: Trait> Debug for Field<T>
//     where
//         T::Value: Debug,
//     {...}
//
// あなたはsyn::TypePathがpath segmentを複数持ち、最初のsegmentが型変数であるような
// ものを探すことで関連型（associated types）を判別することができます。
//
//
// 参考資料:
//
//   - 関連する型は入力の中では以下の構文木で表される：
//     node: https://docs.rs/syn/1.0/syn/struct.TypePath.html

use derive_debug::CustomDebug;
use std::fmt::Debug;

pub trait Trait {
    type Value;
}

#[derive(CustomDebug)]
pub struct Field<T: Trait> {
    values: Vec<T::Value>,
}

fn assert_debug<F: Debug>() {}

fn main() {
    // Does not implement Debug, but its associated type does.
    struct Id;

    impl Trait for Id {
        type Value = u8;
    }

    assert_debug::<Field<Id>>();
}
