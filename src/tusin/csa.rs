#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unreachable_code)]
use std::io::prelude::*;
use std::net::TcpStream;
use std;
///
/// CSA通信
///
/// 参考Webサイト
/// 「ＣＳＡプロトコルによる通信」Ike's page
/// http://usapyon.game.coocan.jp/ComShogi/0c.html
///
/// 「Struct std::net::TcpStream」
/// https://doc.rust-lang.org/std/net/struct.TcpStream.html
use jotai::uchu::*;

/// 使い方説明の表示
pub fn show_usage()
{
    g_writeln("shogi CPU|HUMAN|LAN CPU|HUMAN|LAN [User Password [Server [Port]]]");
    g_writeln("User and Password must set if match with LAN.");
    g_writeln("Server default is usapyon.dip.jp, Port default is 4081");
}

// 手番
enum Teban {
  CPU,
  HUMAN,
  LAN
}
// 手番の文字列の配列
const CSA_TEBAN_LN : usize = 3;
const TEBANSTR : [&str;CSA_TEBAN_LN] = [
  "CPU",
  "HUMAN",
  "LAN",
];

/*
Teban teban[2];

Te TeHistory[1000];
 */
const CSAKOMASTR : [&str;48] = [
//  0:空 空 空 空 空 空 空 空 空 空 空 空 空 空 空 空 16:空
  "","","","","","","","","","","","","","","","","",
//   17:歩   香   桂   銀   金   角   飛   王   と   杏   圭   全  金  馬   龍  32:空
  "FU","KY","KE","GI","KI","KA","HI","OU","TO","NY","NK","NG","","UM","RY","",
//   33:歩   香   桂   銀   金   角   飛   王   と   杏   圭   全  金  馬   47:龍
  "FU","KY","KE","GI","KI","KA","HI","OU","TO","NY","NK","NG","","UM","RY"
];
/*
int ByouHistory[1000];

#ifdef _WINDOWS
SOCKET s;
#else 
int s;
#endif
*/


pub fn csa_send(buf:&String)
{
  let mut stream = TcpStream::connect("127.0.0.1:34254").expect("CSASend:SOCKET ERROR:SOCK NOT OPEN?");
  // ignore the Result
  let _ = stream.write(format!("send:{}", buf).as_bytes());
  let _ = stream.read(&mut [0; 128]); // ignore here too

  g_writeln(&format!("send:{}", buf));
}

///
/// 2017-01-23「[Multi-Threading & TCP] Writing/Reading on TcpStream」
/// https://users.rust-lang.org/t/multi-threading-tcp-writing-reading-on-tcpstream/10558/1
pub fn csa_recv()//buf:&mut String
{
  /*
  let mut stream = TcpStream::connect("127.0.0.1:34254").expect("CSASend:SOCKET ERROR:SOCK NOT OPEN?");
  let sum = 0;
  let nRecv;

  loop {
    // 1byteずつ読み取る。'\n'が来るまで
    let mut buf2 = String::new(); 
    println!("Received {} bytes", stream.read_to_end(&mut buf2).unwrap());
    buf.push_str(buf2.to_string());
    if buf2[0]=='\n' {
      break;
    }
  }

  g_writeln(&format!("recv:{}", buf));
  */
}


// 見本のメインです。
pub fn csa_main()//int argc,char *argv[]
{
/*
  long start=clock();
  // 局面ハッシュの初期化をここに書く
*/
  let mut sikou_jikan_total : [i64;2] = [0, 0];

  let mut i = 0;
  let mut j = 0;
  /*
  for(i=1;i<=2;i++) {
    for(j=CPU;j<=LAN;j++) {
      if (strcmp(argv[i],tebanStr[j])==0) {
        break;
      }
    }
    teban[i-1]=(Teban)j;
    if (j==3) {
      show_usage();
      std::process::exit(1);
    }
  }
*/
let mut user_name = "Kifuwarabe";
let mut password = "KifuwarabePass";
let mut server_domain = "usapyon.dip.jp";
let mut port = 4081;
/*
  if (teban[0]==LAN && teban[1]==LAN) {
 */
    g_writeln("Can't match LAN against LAN.");
    show_usage();
    std::process::exit(1);
/*
  }

  if (teban[0]==LAN || teban[1]==LAN) {
    // LAN同士
    if (user_name[0]=='\0') {
      show_usage();
      std::process::exit(1);
    }

    // Socketを開く
    // ソケット構築
    s=socket(PF_INET,SOCK_STREAM,0);
    if (s<0) {
      */
      g_writeln("Can't Create Socket.");
      std::process::exit(1);
      /*
    }

    // 接続先サーバ名解決
    struct hostent *host=gethostbyname(server_domain);
    if (host==NULL) {
      */
      g_writeln( &format!("Can't Find Server '{}'.", server_domain) );
      std::process::exit(1);
      /*
    }

    // 接続（connect)
    struct sockaddr_in Addr;
    memset(&Addr, 0,sizeof(Addr));
    Addr.sin_family = AF_INET;
    Addr.sin_port = htons(port);
    memcpy((char *)&Addr.sin_addr, (char *)host->h_addr,host->h_length);
    int rtn = connect(s, (sockaddr *)&Addr, sizeof(Addr));
    if (rtn<0) {
      fprintf(stderr,"Can't Connect to Server '%s' Port %d.\n",server_domain,port);
      */
      std::process::exit(1);
      /*
    }
    // user_name,PasswordでLoginする
    */
    let mut buf_login = String::new();
    /*
    sprintf(buf_login,"LOGIN %s %s\n",user_name,password);
    CsaSend(buf_login);

    loop {
      CsaRecv(buf_login); // LOGIN OKがいつか来るはず。
      */
      let mut tmp_login = String::new();
      /*
      g_writeln(&format!("LOGIN:{} OK", user_name));
      if (strcmp(buf_login,tmp_login)==0) {
        */
        g_writeln("ログイン成功");
        /*
        break;
      } else {
        g_writeln(&format!("ログインに失敗しました。\r\n原因: {}", buf_login));
        close(s);
        std::process::exit(1);
      }
    }
    for(;;) {
      CsaRecv(buf_login);
      if (strcmp(buf_login,"Your_Turn:+\n")==0) {
        if (teban[0]==LAN) {
          swap(teban[0],teban[1]);
        }
      }
      if (strcmp(buf_login,"Your_Turn:-\n")==0) {
        if (teban[1]==LAN) {
          swap(teban[0],teban[1]);
        }
      }
      if (strcmp(buf_login,"END Game_Summary\n")==0) {
        break;
      }
    }
    CsaSend("AGREE\n");
    CsaRecv(buf_login);  // STARTが来るはず。
  }
  
  // 平手の初期配置です。見やすいでしょ？変換はその分複雑ですけど。
  KomaInf HirateBan[9][9]={
    {EKY,EKE,EGI,EKI,EOU,EKI,EGI,EKE,EKY},
    {EMP,EHI,EMP,EMP,EMP,EMP,EMP,EKA,EMP},
    {EFU,EFU,EFU,EFU,EFU,EFU,EFU,EFU,EFU},
    {EMP,EMP,EMP,EMP,EMP,EMP,EMP,EMP,EMP},
    {EMP,EMP,EMP,EMP,EMP,EMP,EMP,EMP,EMP},
    {EMP,EMP,EMP,EMP,EMP,EMP,EMP,EMP,EMP},
    {SFU,SFU,SFU,SFU,SFU,SFU,SFU,SFU,SFU},
    {EMP,SKA,EMP,EMP,EMP,EMP,EMP,SHI,EMP},
    {SKY,SKE,SGI,SKI,SOU,SKI,SGI,SKE,SKY}
  };
  // こちらは面倒でもEHIまで0を並べないといけません。
  int HirateMotigoma[EHI+1]={
  // 空空空空空空空空空空空空空空空空空歩香桂銀金角飛王と杏圭全金馬龍空歩香桂銀金角飛
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
  };

  // ０手目で、平手の局面で、持ち駒なしから開始しましょう。
  KyokumenKomagumi Hirate(0,HirateBan,HirateMotigoma);
  KyokumenKomagumi k(Hirate);
  KyokumenKomagumi::InitKanagomaValue();
  shoki=new Kyokumen(0,HirateBan,HirateMotigoma);
  k.Initialize();

  // これはまだ簡単な思考部なので、初期化も簡単です。
  Sikou sikou;

  // 将棋の局面で、最大の手数は５７９手だそうです。
  Te teBuf[600];
  */
  let mut te_num : i64;
  /*
  // 手前のプレイヤーから開始します。
  int SorE=SELF;
*/
  let mut temp_time : i64 = 0;
/*
  srand(time(&temp_time));
*/
  let mut b_sennitite : bool = false;
/*
  // もしも合法手がなくなったら、詰み＝負けです。
  // 合法手がある間はゲームを継続します。
  // ↓
  // 「投了」により、合法手がある場合でも負けになることがあります。
  // また、「投了」しない限り終わりません
  Te te;
  */
  let mut b_lose : bool = false;
  /*
  loop {
    te_num = k.MakeLegalMoves(SorE,teBuf);
    long byouStart=clock();
    k.SenkeiInit();
    k.Print();
    int NowTeban,NextTeban;
    if (SorE==SELF) {
      NowTeban=teban[0];
      NextTeban=teban[1];
    } else {
      NowTeban=teban[1];
      NextTeban=teban[0];
    }
  */
    let mut b_first : bool = true;
  /*
    switch (NowTeban) {
      case HUMAN:
        if (te_num == 0) {
          te=Te(0);
        } else do {
            */
          if !b_first {
            g_writeln("入力された手が異常です。");
          }

          // 手を入力します。
          let mut buf_human = String::new();
          /*
          gets(buf_human);
          // 入力の方法:from,to,promote
          // ただし、歩を打つときはfromを01、香を打つときはfromを02…とする。
          // promoteは、成るときに*を付ける。
          int from,to;
          int koma,capture;
          char promote[2];
          promote[0]='\0';
          if (strcmp(buf_human,"%TORYO")==0) {
            te=Te(0);
          } else {
            int ss=sscanf(buf_human,"%02x%02x%1s",&from,&to,&promote);
            if (ss<2) continue;
            if (from<OU) {
              koma=SorE|from;
              from=0;
            } else {
              koma=k.ban[from];
            }
            capture=k.ban[to];
            if (ss=3 && promote[0]=='*') {
              te=Te(from,to,koma,capture,1);
            } else {
              te=Te(from,to,koma,capture,0);
            }
            b_first = false;
          }
        } while (IsIllegal(te, te_num, teBuf) && !te.IsNull());
        break;
      case CPU:
        te=sikou.Think(SorE,k);
        break;
      case LAN:
        {
          */
          let mut buf_lan = String::new();
          /*
          char komaStr[3];
          char c;
          CsaRecv(buf_lan);  // 相手の指し手が帰ってくる。（正しい指し手であることはサーバでチェック済み）
          if (buf_lan=="%TORYO\n") {
            te=Te(0);
          } else if (buf_lan=="#WIN\n") {
            te=Te(0);
          } else if (buf_lan=="#TIME_UP\n") {
            te=Te(0);
          } else if (buf_lan=="#LOSE\n") {
            */
            // 持将棋宣言負け。
            b_lose = true;
            /*
            break;
          } else {
            sscanf(buf_lan,"%c%02x%02x%2s,T%d\n",&c,&te.from,&te.to,komaStr,&ByouHistory[k.Tesu]);
            int i;
            for(i=0;i<=RY;i++) {
              if (strcmp(komaStr,CSAKomaStr[i|SELF])==0) {
                break;
              }
            }
            te.koma=i|SorE;
            te.promote=0;
            te.capture=k.ban[te.to];
            te.value=0;
            if (te.from!=0 && k.ban[te.from]!=te.koma) {
              te.promote=1;
              te.koma=k.ban[te.from];
            }
          }
        }
        break;
    }
    if (NextTeban==LAN) {
      // 今の手を送る
      if (te.IsNull()) {
        CsaSend("%TORYO\n");
        */
        let mut buf_a = String::new();
        /*
        CsaRecv(buf_a);
        CsaRecv(buf_a);
      } else {
        */
        let mut buf_b = String::new();
        /*
        char komaStr[3];
        char c;
        int from,to;
        sprintf(buf_b,"%c%02x%02x%s\n",SorE==SELF?'+':'-',te.from,te.to,CSAKomaStr[te.koma|(te.promote?PROMOTED:0)]);

        CsaSend(buf_b);
        CsaRecv(buf_b);  // 指し手が帰ってくる。
        if (buf_b=="#TIME_UP\n") {
          // 時間切れ
          CsaRecv(buf_b);
        }
        if (buf_b=="#LOSE\n") {
          */
          // 時間切れ負け。
          b_lose = true;
          /*
          break;
        }
        sscanf(buf_b,"%c%02x%02x%2s,T%d\n",&c,&from,&to,komaStr,&ByouHistory[k.Tesu]);
      }
    }
    TeHistory[k.Tesu]=te;
    if (teban[0]!=LAN && teban[1]!=LAN) {
      // 自前で計測する。
      ByouHistory[k.Tesu]=(clock()-byouStart)/CLOCKS_PER_SEC;
      if (ByouHistory[k.Tesu]==0) {
        // 最低１秒は必ずカウントする
        ByouHistory[k.Tesu]=1;
      }
    }
    te.Print();
    k.Move(SorE,te);
    int sennitite=0;
    int i;
    for(i=k.Tesu;i>0;i-=2) {
      if (k.HashHistory[i]==k.HashVal) {
        sennitite++;
      }
    }
    if (sennitite>=4) {
      b_sennitite = true;
      break;
    }
    if (SorE==SELF) {
      sikou_jikan_total[0] += ByouHistory[k.Tesu-1];
      SorE=ENEMY;
    } else {
      sikou_jikan_total[1] += ByouHistory[k.Tesu-1];
      SorE=SELF;
    }
    printf("\n総思考時間:先手 %2d:%02d 後手%2d:%02d\n",
      sikou_jikan_total[0]/60, sikou_jikan_total[0]%60,
      sikou_jikan_total[1]/60, sikou_jikan_total[1]%60
    );
    if (te.IsNull() || b_lose) {
      break;
    }
  }
  k.Print();
  if (SorE==SELF && te.IsNull()) {
    */
    g_writeln("先手の勝ち。");
    /*
  } else if (SorE==ENEMY && te.IsNull()) {
    */
    g_writeln("後手の勝ち。");
    /*
  } else if (SorE==SELF && (te_num == 0 || b_lose)) {
    */
    g_writeln("後手の勝ち。");
    /*
  } else if (SorE==ENEMY && (te_num == 0 || b_lose)) {
    */
    g_writeln("先手の勝ち。");
    /*
  } else if (b_sennitite) {
    // 千日手による終局
    // 王手の千日手の判定
    */
    g_writeln("千日手です。");
    /*
    int sennitite=0;
    if (Kyokumen::OuteHistory[k.Tesu]) {
      for(int i=k.Tesu;sennitite<=3&&i>0;i-=2) {
        if (!Kyokumen::OuteHistory[i]) {
          break;
        }
        if (k.HashHistory[i]==k.HashVal) {
          sennitite++;
        }
      }
      if (sennitite==4) {
        // 連続王手の千日手
        */
        g_writeln("連続王手の千日手です。");
        /*
        if (SorE==SELF) { 
          */
          g_writeln("後手の勝ち。");
          /*
        } else {
          */
          g_writeln("先手の勝ち。");
          /*
        }
      }
    } else if (Kyokumen::OuteHistory[k.Tesu-1]) {
      // こちらは未検証
      for(int i=k.Tesu;sennitite<=3&&i>0;i-=2) {
        if (!Kyokumen::OuteHistory[i-1]) {
          break;
        }
        if (k.HashHistory[i]==k.HashVal) {
          sennitite++;
        }
      }
      if (sennitite==4) {
        // 連続王手の千日手
        */
        g_writeln("連続王手の千日手です。");
        /*
        if (SorE==SELF) {
          */
          g_writeln("先手の勝ち。");
          /*
        } else {
          */
          g_writeln("後手の勝ち。");
          /*
        }
      }
    }
  }
  FILE *fp=fopen("log.csa","w");
  if (fp==NULL) {
    fprintf(stderr,"log.csaに書き込みできません。");
  } else {
    */
    g_writeln("N+");
    g_writeln("N-");
    g_writeln("P1-KY-KE-GI-KI-OU-KI-GI-KE-KY");
    g_writeln("P2 * -HI *  *  *  *  * -KA * ");
    g_writeln("P3-FU-FU-FU-FU-FU-FU-FU-FU-FU");
    g_writeln("P4 *  *  *  *  *  *  *  *  * ");
    g_writeln("P5 *  *  *  *  *  *  *  *  * ");
    g_writeln("P6 *  *  *  *  *  *  *  *  * ");
    g_writeln("P7+FU+FU+FU+FU+FU+FU+FU+FU+FU");
    g_writeln("P8 * +KA *  *  *  *  * +HI * ");
    g_writeln("P9+KY+KE+GI+KI+OU+KI+GI+KE+KY");
    g_writeln("+");

  /*
    for(i=0;i<k.Tesu;i++) {
      if (!TeHistory[i].IsNull()) {
        if (i%2==0) {
  */
    g_writeln("+");
  /*
        } else {
  */
    g_writeln("-");
  /*
        }
        fprintf(fp,"%02x%02x%s\n",TeHistory[i].from,TeHistory[i].to,CSAKomaStr[TeHistory[i].koma|(TeHistory[i].promote?PROMOTED:0)]);
        fprintf(fp,"T%d\n",ByouHistory[i]);
      } else {
  */
    g_writeln("%%TORYO");
  /*
      }
    }
    fclose(fp);
  }
  if (teban[0]==LAN || teban[1]==LAN) {
    CsaSend("LOGOUT\n");
#ifdef _WINDOWS
    closesocket(s);
#else
    close(s);
#endif
  }
  printf("%.3lfs",(double(clock()-start))/CLOCKS_PER_SEC);
  return 0;
*/
}
