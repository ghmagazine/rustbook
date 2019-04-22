pub mod utils;

// 第1段階：初歩的な実装。u32型の値のソートのみに対応
pub mod first;

// 第2段階：ジェネリクスでさまざまなデータ型に対応
pub mod second;

// 第3段階：クロージャでソート順をカスタマイズ
pub mod third;

// 最終形：並列ソート
pub mod fourth;

pub enum SortOrder {
    Ascending,   // 昇順
    Descending,  // 降順
}
