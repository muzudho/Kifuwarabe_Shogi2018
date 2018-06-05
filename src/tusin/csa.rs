#![allow(dead_code)]
use std::io::prelude::*;
use std::net::TcpStream;
///
/// CSA通信
///
/// 参考Webサイト
/// 「ＣＳＡプロトコルによる通信」Ike's page
/// http://usapyon.game.coocan.jp/ComShogi/0c.html
///
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

/*
void CsaRecv(char *buf)
{
  int sum = 0,nRecv;
  char c[1];

  for (;;) {
    // 1byteずつ読み取る。'\n'が来るまで
    nRecv = recv( s, c, 1, 0);
    if ( nRecv < 0 ) {
      fprintf(stderr,"CSARecv:SOCKET ERROR:SOCK NOT OPEN?\n");
      exit(1);
    }
    buf[sum++] = c[0];
    if (c[0]=='\n') {
      break;
    }
  }
  buf[sum++]='\0';
  printf("recv:%s",buf);
}

// 見本のメインです。
int main(int argc,char *argv[])
{
  long start=clock();
  Kyokumen::HashInit();
  int SikouJikanTotal[2]={0,0};

  int i;
  int j;
  if (argc<3) {
    show_usage();
    exit(1);
  }
  for(i=1;i<=2;i++) {
    for(j=CPU;j<=LAN;j++) {
      if (strcmp(argv[i],tebanStr[j])==0) {
        break;
      }
    }
    teban[i-1]=(Teban)j;
    if (j==3) {
      show_usage();
      exit(1);
    }
  }
  char User[256];
  char Password[256];
  if (argc>=5) {
    strcpy(User,argv[3]);
    strcpy(Password,argv[4]);
  } else {
    strcpy(User,"");
    strcpy(Password,"");
  }
  char Server[256];
  if (argc>=6) {
    strcpy(Server,argv[5]);
  } else {
    strcpy(Server,"usapyon.dip.jp");
  }
  int Port;
  if (argc>=7) {
    Port=atoi(argv[6]);
  } else {
    Port=4081;
  }
  if (teban[0]==LAN && teban[1]==LAN) {
    fprintf(stderr,"Can't match LAN against LAN.\n");
    show_usage();
    exit(1);
  }
  if (teban[0]==LAN || teban[1]==LAN) {
    if (User[0]=='\0') {
      show_usage();
      exit(1);
    }
    // Socketを開く
#ifdef _WINDOWS
    WORD version=0x0101;
    WSADATA WSAData;
    WSAStartup(version,&WSAData);
#endif
    // ソケット構築
    s=socket(PF_INET,SOCK_STREAM,0);
    if (s<0) {
      fprintf(stderr,"Can't Create Socket.\n");
      exit(1);
    }
    // 接続先サーバ名解決
    struct hostent *host=gethostbyname(Server);
    if (host==NULL) {
      fprintf(stderr,"Can't Find Server '%s'.\n",Server);
      exit(1);
    }
    // 接続（connect)
#ifdef _WINDOWS
    SOCKADDR_IN Addr;
#else
    struct sockaddr_in Addr;
#endif
    memset(&Addr, 0,sizeof(Addr));
    Addr.sin_family = AF_INET;
    Addr.sin_port = htons(Port);
    memcpy((char *)&Addr.sin_addr, (char *)host->h_addr,host->h_length);
    int rtn = connect(s, (sockaddr *)&Addr, sizeof(Addr));
    if (rtn<0) {
      fprintf(stderr,"Can't Connect to Server '%s' Port %d.\n",Server,Port);
      exit(1);
    }
    // User,PasswordでLoginする
    char buf[256];
    sprintf(buf,"LOGIN %s %s\n",User,Password);
    CsaSend(buf);
    for(;;) {
      CsaRecv(buf); // LOGIN OKがいつか来るはず。
      char tmp[256];
      sprintf(tmp,"LOGIN:%s OK\n",User);
      if (strcmp(buf,tmp)==0) {
        printf("ログイン成功");
        break;
      } else {
        printf("ログインに失敗しました。\r\n原因: %s\n", buf);
#ifdef _WINDOWS
        closesocket(s);
#else
        close(s);
#endif
        exit(1);
      }
    }
    for(;;) {
      CsaRecv(buf);
      if (strcmp(buf,"Your_Turn:+\n")==0) {
        if (teban[0]==LAN) {
          swap(teban[0],teban[1]);
        }
      }
      if (strcmp(buf,"Your_Turn:-\n")==0) {
        if (teban[1]==LAN) {
          swap(teban[0],teban[1]);
        }
      }
      if (strcmp(buf,"END Game_Summary\n")==0) {
        break;
      }
    }
    CsaSend("AGREE\n");
    CsaRecv(buf);  // STARTが来るはず。
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
  int teNum;

  // 手前のプレイヤーから開始します。
  int SorE=SELF;

#ifdef _GCC_
  time_t temp;
  srand(time(&temp));
#else
  long temp;
  srand(time(&temp));
#endif

  int bSennitite=false;

  // もしも合法手がなくなったら、詰み＝負けです。
  // 合法手がある間はゲームを継続します。
  // ↓
  // 「投了」により、合法手がある場合でも負けになることがあります。
  // また、「投了」しない限り終わりません
  Te te;
  bool bLose=false;
  for(;;) {
    teNum=k.MakeLegalMoves(SorE,teBuf);
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
    bool bFirst=true;
    switch (NowTeban) {
      case HUMAN:
        if (teNum==0) {
          te=Te(0);
        } else do {
          if (!bFirst) {
            printf("入力された手が異常です。\n");
          }
          // 手を入力します。
          char buf[80];
          gets(buf);
          // 入力の方法:from,to,promote
          // ただし、歩を打つときはfromを01、香を打つときはfromを02…とする。
          // promoteは、成るときに*を付ける。
          int from,to;
          int koma,capture;
          char promote[2];
          promote[0]='\0';
          if (strcmp(buf,"%TORYO")==0) {
            te=Te(0);
          } else {
            int ss=sscanf(buf,"%02x%02x%1s",&from,&to,&promote);
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
            bFirst=false;
          }
        } while (IsIllegal(te,teNum,teBuf) && !te.IsNull());
        break;
      case CPU:
        te=sikou.Think(SorE,k);
        break;
      case LAN:
        {
          char buf[256];
          char komaStr[3];
          char c;
          CsaRecv(buf);  // 相手の指し手が帰ってくる。（正しい指し手であることはサーバでチェック済み）
          if (strcmp(buf,"%TORYO\n")==0) {
            te=Te(0);
          } else if (strcmp(buf,"#WIN\n")==0) {
            te=Te(0);
          } else if (strcmp(buf,"#TIME_UP\n")==0) {
            te=Te(0);
          } else if (strcmp(buf,"#LOSE\n")==0) {
            // 持将棋宣言負け。
            bLose=true;
            break;
          } else {
            sscanf(buf,"%c%02x%02x%2s,T%d\n",&c,&te.from,&te.to,komaStr,&ByouHistory[k.Tesu]);
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
        char buf[256];
        CsaRecv(buf);
        CsaRecv(buf);
      } else {
        char buf[256];
        char komaStr[3];
        char c;
        int from,to;
        sprintf(buf,"%c%02x%02x%s\n",SorE==SELF?'+':'-',te.from,te.to,CSAKomaStr[te.koma|(te.promote?PROMOTED:0)]);

        CsaSend(buf);
        CsaRecv(buf);  // 指し手が帰ってくる。
        if (strcmp(buf,"#TIME_UP\n")==0) {
          // 時間切れ
          CsaRecv(buf);
        }
        if (strcmp(buf,"#LOSE\n")==0) {
          // 時間切れ負け。
          bLose=true;
          break;
        }
        sscanf(buf,"%c%02x%02x%2s,T%d\n",&c,&from,&to,komaStr,&ByouHistory[k.Tesu]);
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
      bSennitite=true;
      break;
    }
    if (SorE==SELF) {
      SikouJikanTotal[0]+=ByouHistory[k.Tesu-1];
      SorE=ENEMY;
    } else {
      SikouJikanTotal[1]+=ByouHistory[k.Tesu-1];
      SorE=SELF;
    }
    printf("\n総思考時間:先手 %2d:%02d 後手%2d:%02d\n",
      SikouJikanTotal[0]/60,SikouJikanTotal[0]%60,
      SikouJikanTotal[1]/60,SikouJikanTotal[1]%60
    );
    if (te.IsNull() || bLose) {
      break;
    }
  }
  k.Print();
  if (SorE==SELF && te.IsNull()) {
    printf("先手の勝ち。\n");
  } else if (SorE==ENEMY && te.IsNull()) {
    printf("後手の勝ち。\n");
  } else if (SorE==SELF && (teNum==0 || bLose)) {
    printf("後手の勝ち。\n");
  } else if (SorE==ENEMY && (teNum==0 || bLose)) {
    printf("先手の勝ち。\n");
  } else if (bSennitite) {
    // 千日手による終局
    // 王手の千日手の判定
    printf("千日手です。\n");
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
        printf("連続王手の千日手です。\n");
        if (SorE==SELF) { 
          printf("後手の勝ち。\n");
        } else {
          printf("先手の勝ち。\n");
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
        printf("連続王手の千日手です。\n");
        if (SorE==SELF) {
          printf("先手の勝ち。\n");
        } else {
          printf("後手の勝ち。\n");
        }
      }
    }
  }
  FILE *fp=fopen("log.csa","w");
  if (fp==NULL) {
    fprintf(stderr,"log.csaに書き込みできません。");
  } else {
    fprintf(fp,"N+\n");
    fprintf(fp,"N-\n");
    fprintf(fp,"P1-KY-KE-GI-KI-OU-KI-GI-KE-KY\n");
    fprintf(fp,"P2 * -HI *  *  *  *  * -KA * \n");
    fprintf(fp,"P3-FU-FU-FU-FU-FU-FU-FU-FU-FU\n");
    fprintf(fp,"P4 *  *  *  *  *  *  *  *  * \n");
    fprintf(fp,"P5 *  *  *  *  *  *  *  *  * \n");
    fprintf(fp,"P6 *  *  *  *  *  *  *  *  * \n");
    fprintf(fp,"P7+FU+FU+FU+FU+FU+FU+FU+FU+FU\n");
    fprintf(fp,"P8 * +KA *  *  *  *  * +HI * \n");
    fprintf(fp,"P9+KY+KE+GI+KI+OU+KI+GI+KE+KY\n");
    fprintf(fp,"+\n");
    for(i=0;i<k.Tesu;i++) {
      if (!TeHistory[i].IsNull()) {
        if (i%2==0) {
          fprintf(fp,"+");
        } else {
          fprintf(fp,"-");
        }
        fprintf(fp,"%02x%02x%s\n",TeHistory[i].from,TeHistory[i].to,CSAKomaStr[TeHistory[i].koma|(TeHistory[i].promote?PROMOTED:0)]);
        fprintf(fp,"T%d\n",ByouHistory[i]);
      } else {
        fprintf(fp,"%%TORYO\n");
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
}
*/