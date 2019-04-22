use std::fmt;

pub struct ToyVec<T> {
    elements: Box<[T]>,  // T型の要素を格納する領域。各要素はヒープ領域に置かれる
    len: usize,          // ベクタの長さ（現在の要素数）
}

// implブロック内に関連関数やメソッドを定義していく。トレイト境界としてDefaultを設定する
impl<T: Default> ToyVec<T> {

    // newはキャパシティ（容量）が0のToyVecを作る
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    // with_capacityは指定されたキャパシティを持つToyVecを作る
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            elements: Self::allocate_in_heap(capacity),
            len: 0,
        }
    }

    // T型の値がsize個格納できるBox<[T]>を返す
    fn allocate_in_heap(size: usize) -> Box<[T]> {
        std::iter::repeat_with(Default::default)
            .take(size)            // T型のデフォルト値をsize個作り
            .collect::<Vec<_>>()   // Vec<T>に収集してから
            .into_boxed_slice()    // Box<[T]>に変換する
    }

    // ベクタの長さを返す
    pub fn len(&self) -> usize {
        self.len
    }

    // ベクタの現在のキャパシティを返す
    pub fn capacity(&self) -> usize {
        self.elements.len()   // elementsの要素数（len）がToyVecのキャパシティになる
    }

    pub fn push(&mut self, element: T) {
        if self.len == self.capacity() {  // 要素を追加するスペースがないなら
            self.grow();  // もっと大きいelementsを確保して既存の要素を引っ越す
        }
        self.elements[self.len] = element; // 要素を格納する（所有権がムーブする）
        self.len += 1;
    }

    // elementsを拡張する（より大きなサイズで作り直す）
    fn grow(&mut self) {
        if self.capacity() == 0 {  // 現在のelementsが空なら
            // 1要素分の領域を確保する
            self.elements = Self::allocate_in_heap(1);
        } else {
            // 現在の2倍の領域を確保する
            let new_elements = Self::allocate_in_heap(self.capacity() * 2);
            // self.elementsを置き換える
            let old_elements = std::mem::replace(&mut self.elements, new_elements);
            // 既存の全要素を新しい領域へムーブする
            // Vec<T>のinto_iter(self)なら要素の所有権が得られる
            for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
                self.elements[i] = elem;
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {            // インデックスが範囲内なら
            Some(&self.elements[index])  // Some(不変の参照)を返す
        } else {
            None                         // 範囲外ならNoneを返す
        }
    }

    // インデックスが範囲内なら要素への参照を返し、さもなければdefaultで与えた別の値への参照を返す
    pub fn get_or<'a>(&'a self, index: usize, default: &'a T) -> &'a T {
        self.get(index).unwrap_or(default)
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;

            // 最後の要素の所有権を得たいが、借用（&mut self）経由では所有権を奪えない
            // let elem = self.elements[self.len];
            //   → error[E0507]: cannot move out of borrowed content

            // 代わりの値と交換するならできる（ここではデフォルト値を使用）
            let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
            Some(elem)
        }
    }

    // 要素へのイミュータブルな参照（Option<&T>）を返すイテレータを作る
    // 説明のためにライフタイムを明示しているが、実際には省略できる
    pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
        Iter {
            elements: &self.elements,  // Iter構造体の定義より、ライフタイムは'vecになる
            len: self.len,
            pos: 0,
        }
    }

    // 要素へのイミュータブルな参照（Option<&mut T>）を返すイテレータを作る
    pub fn iter_mut<'vec>(&'vec mut self) -> IterMut<'vec, T> {
        IterMut {
            elements: &mut self.elements,
            len: self.len,
            pos: 0,
        }
    }

    // 要素の所有権をとる（Option<T>）イテレータを作る
    pub fn into_iter<'vec>(self) -> IntoIter<T> {
        IntoIter {
            elements: self.elements,
            len: self.len,
            pos: 0,
        }
    }

}

impl<T: Default> Default for ToyVec<T> {
    fn default() -> Self {
        // newはキャパシティ（容量）が0のToyVecを作る
        Self::new()
    }
}

impl<T: Clone + Default> Clone for ToyVec<T> {
    fn clone(&self) -> Self {
        let mut cloned = Self::with_capacity(self.len());
        // 各要素のcloneを呼ぶことでdeepコピーを実現する
        for elem in self.iter() {
            cloned.push(elem.clone());
        }
        cloned
    }
}

impl<T: PartialEq> PartialEq for ToyVec<T> {
    fn eq(&self, other: &Self) -> bool {
        // スライス[T]同士を比較。各要素（T）がPartialEqを実装しているので可能になる
        self.elements[..self.len] == other.elements[..other.len]
    }
}

impl<T: fmt::Debug> fmt::Debug for ToyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.elements[..self.len].fmt(f)
    }
}

// IntoIteratorトレイトを実装するとfor式での繰り返しができるようになる
impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
    type Item = &'vec T;            // イテレータがイテレートする値の型
    type IntoIter = Iter<'vec, T>;  // into_iterメソッドの戻り値の型

    // &ToyVec<T>に対するトレイト実装なので、selfの型はToyVec<T>ではなく&ToyVec<T>
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'vec, T: Default> IntoIterator for &'vec mut ToyVec<T> {
    type Item = &'vec mut T;
    type IntoIter = IterMut<'vec, T>;

    // selfの型はToyVec<T>ではなく&mut ToyVec<T>
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: Default> IntoIterator for ToyVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    // selfの型はToyVec<T>
    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

//
// 要素へのイミュータブルな参照（Option<&T>）を返すイテレータ
//

// ライフタイムの指定により、このイテレータ自身またはnext()で得た&'vec T型の値が
// 生存してる間は、ToyVec<T>は変更できない
pub struct Iter<'vec, T> {
    elements: &'vec Box<[T]>,
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for Iter<'vec, T> {
    // 関連型（トレイトに関連付いた型）で、このイテレータがイテレートする要素の型を指定する
    // 関連型は8章で説明
    type Item = &'vec T;

    // nextメソッドは次の要素を返す
    // 要素があるなら不変の参照（&T）をSomeで包んで返し、ないときはNoneを返す
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            let res = Some(&self.elements[self.pos]);
            self.pos += 1;
            res
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

//
// 要素へのミュータブルな参照（Option<&mut T>）を返すイテレータ
//

pub struct IterMut<'vec, T> {
    elements: &'vec mut Box<[T]>,  // ミュータブルな参照
    len: usize,
    pos: usize,
}

impl<'vec, T> Iterator for IterMut<'vec, T> {
    type Item = &'vec mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            // 要素を&'vec mut Tとして返したいが、&'a mut selfから要素を取り出すと
            // 要素が&'a mut Tになってしまい、ライフタイム要件が満たせない
            // そこで以下のように対応した
            //   1. &'a mut Tを生ポインタ*mut Tに変換してライフタイムをなくす
            //   2. *mut Tの参照外しをして要素Tにアクセス
            //   3. 要素Tから&'vec mut Tを得る
            let elem = unsafe { &mut *(&mut self.elements[self.pos] as *mut T) };
            self.pos += 1;
            Some(elem)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

//
// 要素の所有権をとるイテレータ。Option<T>を返す
//

pub struct IntoIter<T> {
    elements: Box<[T]>,  // ミュータブルな参照
    len: usize,
    pos: usize,
}

impl<T: Default> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.len {
            None
        } else {
            // &mut selfから要素Tをムーブアウトできないのでreplaceでデフォルト値と交換している
            let elem = std::mem::replace(&mut self.elements[self.pos], Default::default());
            self.pos += 1;
            Some(elem)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

#[cfg(test)]
mod tests {

    use super::ToyVec;

    #[test]
    fn test_char_vec() {
        let mut v = ToyVec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);

        v.push('a');
        assert_eq!(v.capacity(), 1);
        v.push('b');
        assert_eq!(v.capacity(), 2);
        v.push('c');
        assert_eq!(v.capacity(), 4);
        assert_eq!(v.len(), 3);

        assert_eq!(v.get(2), Some(&'c'));
        assert_eq!(v.get(3), None);

        let mut iter = v.iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(&'a'));
        assert_eq!(iter.next(), Some(&'b'));
        assert_eq!(iter.next(), Some(&'c'));
        assert_eq!(iter.next(), None);

        assert_eq!(v.pop(), Some('c'));
        assert_eq!(v.pop(), Some('b'));
        assert_eq!(v.pop(), Some('a'));
        assert_eq!(v.pop(), None);

        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 4);
    }

    #[test]
    fn test_string_vec() {
        let mut v = ToyVec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);

        v.push("alfalfa");
        v.push("broccoli");
        v.push("carrot");
        assert_eq!(v.capacity(), 4);
        assert_eq!(v.len(), 3);

        assert_eq!(v.get(2), Some(&"carrot"));
        assert_eq!(v.get(3), None);

        let mut iter = v.iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(&"alfalfa"));
        assert_eq!(iter.next(), Some(&"broccoli"));
        assert_eq!(iter.next(), Some(&"carrot"));
        assert_eq!(iter.next(), None);

        assert_eq!(v.pop(), Some("carrot"));
        assert_eq!(v.pop(), Some("broccoli"));
        assert_eq!(v.pop(), Some("alfalfa"));
        assert_eq!(v.pop(), None);

        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 4);
    }

    #[test]
    fn test_nested_vec() {
        let mut v = ToyVec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);

        let mut v1 = ToyVec::new();
        v1.push("alfalfa");
        let mut v2 = ToyVec::new();
        v2.push("broccoli");
        let mut v3 = ToyVec::new();
        v3.push("carrot");

        v.push(v1);
        v.push(v2);
        v.push(v3);

        assert_eq!(v.capacity(), 4);
        assert_eq!(v.len(), 3);

        assert!(v.get(2).is_some());
        assert!(v.get(3).is_none());

        let mut iter = v.iter();
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());

        let mut v1 = ToyVec::new();
        v1.push("alfalfa");
        let mut v2 = ToyVec::new();
        v2.push("broccoli");
        let mut v3 = ToyVec::new();
        v3.push("carrot");

        assert_eq!(v.pop(), Some(v3));
        assert_eq!(v.pop(), Some(v2));
        assert_eq!(v.pop(), Some(v1));
        assert_eq!(v.pop(), None);

        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 4);
    }

    #[test]
    fn test_iter() {
        let mut v = ToyVec::new();
        v.push(1);
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(5);

        // このイテレータはイミュータブルな要素（Option<&i32>）を返す
        let mut iter = v.iter();

        // イテレータが有効な間でもvからの直接のget（読み出し）はできる
        assert_eq!(v.get(4), Some(&5));

        // イテレータが有効な間はvへの直接のpush（変更）はできない
        // v.push(8);
        // → error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable

        let mut sum = 0;

        // &ToyVec<T>にIntoIteratorを実装し、Iter<T>を返すようにしたので以下のように使える
        for i in &v {
            sum += *i;
        }

        assert_eq!(sum, [1, 1, 2, 3, 5].iter().sum());
        assert_eq!(iter.next(), Some(&1));

        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_iter_mut() {
        let mut v = ToyVec::new();  // ToyVec<i32>
        v.push(1);
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(5);

        // このイテレータはミュータブルな要素（Option<&mut i32>）を返す
        let mut iter = v.iter_mut();

        // イテレータが有効な間はvからの直接のget（読み出し）やpush（変更）はできない
        // v.get(0);
        // → error[E0502]: cannot borrow `v` as immutable because it is also borrowed as mutable

        iter.next().map(|i| *i *= 8);  // 最初の要素を8倍する

        // &mut ToyVec<T>にIntoIteratorを実装し、IterMut<T>を返すようにしたので以下のように使える
        for i in &mut v {
            *i += 10;
        }

        assert_eq!(v.get(0), Some(&18));
        assert_eq!(v.get(1), Some(&11));
        assert_eq!(v.get(2), Some(&12));
        assert_eq!(v.get(3), Some(&13));
        assert_eq!(v.get(4), Some(&15));
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_into_iter() {
        let mut v = ToyVec::new();
        v.push(1);
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(5);

        let mut sum = 0;

        // ToyVec<T>にIntoIteratorを実装し、IntoIter<T>を返すようにしたので以下のように使える
        for i in v {
            sum += i;

            // IntoIter<T>はToyVec<T>消費するので、作成後はvにアクセスできなくなる
            // v.get(0);
            // → error[E0382]: borrow of moved value: `v`
        }

        assert_eq!(sum, [1, 1, 2, 3, 5].iter().sum());
    }

}
