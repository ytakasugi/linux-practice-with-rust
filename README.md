# linux-practice-with-rust

## Github

[linux-in-practice-2nd](https://github.com/satoru-takeuchi/linux-in-practice-2nd)

## Reference

* 試して理解 Linuxのしくみ
  * [1章〜4章](https://zenn.dev/elvis/scraps/9ddd9a012c1621)
  * [5章〜8章](https://zenn.dev/elvis/scraps/3a022eafa4cb8b)
* [Linuxのmanコマンドで表示される()内の数字（セクション番号）について](https://qiita.com/yasushi-jp/items/ca6a32fa51f9c7a5c1e9)
* [SIer だけど技術やりたいブログ](https://www.kimullaa.com/categories/linux/)
    * [Linux メモリ管理 徹底入門(プロセス編)](https://www.kimullaa.com/posts/201912010532/)
    * [Linux システムコール 徹底入門](https://www.kimullaa.com/posts/202001051012/)
    * [Linux メモリ管理 徹底入門(カーネル編)](https://www.kimullaa.com/posts/202002160657/)
    * [Linux プロセス管理を理解したい](https://www.kimullaa.com/posts/202004070247/)

## Execute

```test
cargo build --example ${file_name}
cargo run --example ${file_name}
```

## ch04-02-mmapの実行結果例

```text
新規メモリ領域獲得前のメモリマップ
...（省略）
558bea893000-558bea8b4000 rw-p 00000000 00:00 0                          [heap]
7fdcc4e24000-7fdd04e28000 rw-p 00000000 00:00 0 
...（省略）
新規メモリ領域： アドレス = 0x7fd3c282b000, サイズ = 0x40000000
新規メモリ領域獲得後のメモリマップ
...（省略）
55e051ce3000-55e051d04000 rw-p 00000000 00:00 0                          [heap]
7fd3c282b000-7fd40282f000 rw-p 00000000 00:00 0 
```

上記を計算すると・・・

```text
7fdcc4e24000 - 7fd3c282b000 = 1073741824 byte ≒ 1Gib
```

### Note

16進数から10進数に変換する
