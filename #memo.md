# ロガー作りかけ。
logs フォルダーが必要。

# 投了テスト
position startpos moves 3g3f 3c3d 3i3h 2b7g+ 7i6h 7g6h 5i4h 6h6i 4h3g 6i8g 9g9f 8g8h 8i7g 8h7g 9i9h 7g6g 4i4h B*6d 4g4f N*2e 3g4g G*5e 2h1h 5e4f

# まだ逃げれるのでは？
position startpos moves 5i5h 5a6b 7i6h 6b7b 1i1h 3c3d 3i4h 5c5d 6h7i 3a4b 3g3f 4b5c 6g6f 7a6b 7i6h 8c8d 3f3e 3d3e 4g4f 8d8e 5h5i 8e8f 8g8f 8b8f P*3c 8f8h+ 2h3h 8h8i 3h3f 3e3f 9g9f 2b3c 2i3g 3f3g+ 6i7i N*6g 5i5h 6g7i+ 7g7f B*1d 5h6g 8i7h

# 設定。
option name depth type spin default 1 min 1 max 3
setoption name depth value 2
setoption name depth value 3

# なぜ投了
position startpos moves 2g2f 6a7b 2f2e 3c3d 2e2d 1c1d 2d2c+ 2b4d P*2b 2a1c 2b2a+ 3a4b 2a1a 4a3a 1a1b 8c8d L*4f 4d5e 1b1c 5a6a 1c2b 3a4a 2b3b 4a5a 5g5f 5e6d 2c2b 5a5b 2h2c+ 4b5a 4f4c+ 5b6b 2c3d 5c5d 7g7f 5a5b 3b4b 5b4a 3d3a 7b8c 3a4a 6a7b 4b5b 8c9d 8h4d 6b6a 4a6a
go btime 494000 wtime 289000 binc 10000 winc 10000

- 玉を取られた時点で探索を打ち切りたい。

#

>1:usi
<1:id name Kifuwarabe 2018
<1:id author TAKAHASHI, Satoshi
<1:option name depth type spin default 1 min 1 max 999
<1:usiok
>1:setoption name USI_Ponder value true
>1:setoption name USI_Hash value 256
>1:setoption name depth value 11
>1:isready
<1:readyok
>1:usinewgame
>1:position startpos
>1:go btime 600000 wtime 600000 byoyomi 10000
