
//構造体
struct Philosopher {
    name : String,
}
//名前には&str型ではなくString型を選んだ。
//一般的に、データを所有する方を用いたほうが、データを参照する方の利用よりも簡単である。

// implブロック
// Philosopher構造体に関する定義
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Butler"),
        Philosopher::new("Gilles Deleuze"),
        Philosopher::new("Karl Marx"),
        Philosopher::new("Emma Goldman"),
        Philosopher::new("Michel Foucault"),
    ];

    for p in &philosophers {
        p.eat();
    }
}
