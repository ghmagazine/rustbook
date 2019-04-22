# -*- coding:utf-8-unix -*-
u"Bitonic Merge Sortモジュール。"

# Pythonによるsort関数
def sort(x, up):
    u"""
    リストxの要素を、upで指定された向きにソートする。upがTrueなら昇順、
    Falseなら降順になる。xの要素数は2のべき乗でなければならない
    （さもなければソート結果がおかしくなる）
    """
    if len(x) <= 1:
        # 要素数が1以下になったら終わり
        return x
    else:
        # ステップ1a
        # リストの前半（first）は昇順、後半（second）は降順でソートする
        mid_point = len(x) // 2        # `//`は整数除算
        first = sort(x[:mid_point], True)
        second = sort(x[mid_point:], False)

        # ステップ1b
        # 2分割したリストを1つに結合する
        x1 = first + second

        # ステップ2：サブソートに進む
        return _sub_sort(x1, up)

def _sub_sort(x, up):
    u"""
    バイトニックにソートされたリストxの前半と後半を、upで指定された向きに
    比較、交換し、前半と後半それぞれについて再帰的にサブソートを適用する
    """
    if len(x) == 1:
        # 要素数が1以下になったら終わり
        return x
    else:
        # ステップ2a
        # 要素数nのバイトニック列の要素をn/2要素おきに比較して
        # upで指定された順序（昇順または降順）になるように交換する
        _compare_and_swap(x, up)

        # ステップ2b
        # データ列を半分に分割し、それぞれに対して_sub_sortを繰り返す
        mid_point = len(x) // 2
        first = _sub_sort(x[:mid_point], up)
        second = _sub_sort(x[mid_point:], up)

        # ステップ2c
        # 2分割したデータ列を1つに結合する
        return first + second

def _compare_and_swap(x, up):
    u"""
    要素数nのバイトニック列の要素をn/2要素おきに比較して、upで指定された
    順序（昇順または降順）になるよう交換する（ステップ2a）
    """
    mid_point = len(x) // 2
    for i in range(mid_point):
        if (x[i] > x[mid_point + i]) == up:
            # 要素を交換する
            x[i], x[mid_point + i] = x[mid_point + i], x[i]
