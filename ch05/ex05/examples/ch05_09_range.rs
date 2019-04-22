fn main() {
    let a = ['a', 'b', 'c', 'd', 'e'];

    // 糖衣構文と実際の範囲の対応
    assert_eq!(a[ ..  ], ['a', 'b', 'c', 'd', 'e'] );
    assert_eq!(a[ .. 3], ['a', 'b', 'c',         ] );
    assert_eq!(a[ ..=3], ['a', 'b', 'c', 'd'     ] );
    assert_eq!(a[1..  ], [     'b', 'c', 'd', 'e'] );
    assert_eq!(a[1.. 3], [     'b', 'c'          ] );
    assert_eq!(a[1..=3], [     'b', 'c', 'd'     ] );

    // 糖衣構文とRange*型の対応
    assert_eq!(   ..   , std::ops::RangeFull                   );
    assert_eq!(   .. 3 , std::ops::RangeTo { end: 3 }          );
    assert_eq!(   ..=3 , std::ops::RangeToInclusive { end: 3 } );
    assert_eq!(  1..   , std::ops::RangeFrom { start: 1 }      );
    assert_eq!(  1.. 3 , std::ops::Range { start: 1, end: 3 }  );
    assert_eq!(  1..=3 , std::ops::RangeInclusive::new(1, 3)   );
}
