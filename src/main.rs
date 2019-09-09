
use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc}; // 並列処理を制御するための機構

//構造体
struct Philosopher {
    name: String,
    left: usize,
    right: usize, // 両側にフォーク
}

struct Table {
    // Mutexのベクトルを保持
    forks: Vec<Mutex<()>>,
}

//名前には&str型ではなくString型を選んだ。
//一般的に、データを所有する方を用いたほうが、データを参照する方の利用よりも簡単である。

// implブロック
// Philosopher構造体に関する定義
impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right, // インスタンス生成時に必要になるものを記載
        }
    }

    fn eat(&self, table: &Table) {
        // Tableが保持するフォークのリストにアクセスする。
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(200));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000)); //ミリ秒待つ
        println!("{} is done eating.", self.name);
    }
}

fn main() {

    // 新しいTableを作ってArc<T>に包んでいる
    // Arc: アトミック参照カウント。複数スレッドからTableを共有するために必要
    let table = Arc::new(Table { forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ]});

    let philosophers = vec![
        Philosopher::new("Judith Butler", 0, 1), // フォーク番号を入れる
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        Philosopher::new("Michel Foucault", 0, 4), // デッドロックを防ぐために4と0を入れ替える
    ];

    //新しい束縛、handlesを追加。<_>は型プレースホルダ。
    //handlesは何らかの型のベクトルであるが、何の型かはRustが解決してくれという意味
    // into_iter() 哲学者の所有権を持つイテレータの生成。イテレータに対してmapを呼び出し、
    // その引数として要素ごとに順番に呼ばれるクロージャを渡す。
    let handles: Vec<_> = philosophers.into_iter().map(|p|{
        let table = table.clone();
        // 平行実行される部分
        thread::spawn(move || {
            p.eat(&table);
        })
        // collect() map 呼び出しの結果をまとめ上げる。
    }).collect();

    // ハンドルへのjoin()呼び出し。各スレッド実行が完了するまで実行をブロックする。
    for h in handles {
        h.join().unwrap();
    }
}
