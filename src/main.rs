pub const MAX_USER: i16 = 50;
pub const INIT_COIN: i16 = 100;
pub const SKIP_ADD: i16 = 15;
pub const VICTORY_COUNT: i16 = 500;

#[allow(dead_code)]
fn system(cmd: &str) -> std::process::Command {
    std::process::Command::new(cmd)
}
#[allow(dead_code)]
fn clear() {
    if std::env::consts::OS == "windows" {
        system("cls");
    } else {
        system("clear");
    }
}

pub struct Player {
    pub name: String,
    pub coin: i16,
    pub prev: i16,
    pub pay: i16,
}
impl Player {
    pub fn new(name_: &str, coin_: i16) -> Player {
        Player {
            name: name_.to_string(),
            coin: coin_,
            prev: coin_,
            pay: 0,
        }
    }
    pub fn input_pay(&mut self) {
        loop {
            self.prev = self.coin;
            println!(
                "Player {}, please enter your pay ( 0 for skip ): ",
                self.name
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if let Ok(value) = input.trim().parse::<i16>() {
                if value <= self.coin && value >= 0 {
                    self.pay = value;
                    self.coin -= self.pay;
                    if self.pay == 0 {
                        self.coin += SKIP_ADD
                    }
                    break clearscreen::clear().unwrap();
                }
            }
            println!("Illegal input! Please input again.")
        }
    }
}

trait PlayerList {
    fn init(&mut self);
    fn finished(&self) -> bool;
    fn input(&mut self);
    fn calc(&mut self);
    fn show(&self);
}

impl PlayerList for Vec<Player> {
    fn init(&mut self) {
        println!("[ create player ]");
        loop {
            use std::io::stdin;
            println!(
                "Currently {} players, continue to create? (y/N)",
                self.len()
            );
            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            if buf.trim() != "y" && buf != "Y" {
                if self.len() < 2 {
                    println!("You have to create at least 2 players!")
                } else {
                    break;
                }
            }
            buf = String::new();
            println!("Please input name for player: ");
            stdin().read_line(&mut buf).unwrap();
            for i in &self[..] {
                if buf.trim() == i.name {
                    println!("Name Occupied!");
                    continue;
                }
            }
            //for i in self {}
            self.push(Player::new(buf.trim(), INIT_COIN));
        }
    }
    fn finished(&self) -> bool {
        for i in self {
            if i.coin > VICTORY_COUNT {
                println!("[ game end ]");
                println!("{} is winner.", i.name);
                return true;
            }
        }
        println!("[ turn start ]");
        println!("Continue game? (Y/n)");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        if buf.trim() == "n" || buf.trim() == "N" {
            return true;
        }
        false
    }
    fn input(&mut self) {
        for i in self {
            i.input_pay()
        }
    }
    fn calc(&mut self) {
        let mut winner: usize = 0;
        let mut sum: i16 = 0;
        for i in 0..self.len() {
            sum += self[i].pay;
            if self[i].pay > self[winner].pay {
                winner = i
            }
        }
        self[winner].coin += sum;
        /* for i in self {
            i.prev = i.coin;
            i.coin -= i.pay;
            if i.pay == 0 {
                i.coin += SKIP_ADD
            }
        } */
    }
    fn show(&self) {
        println!("[ turn review ]");
        for i in self {
            println!("  [ {} ]", i.name);
            println!("  paid {}, {} -> {}", i.pay, i.prev, i.coin);
        }
    }
}

/* pub unsafe fn init() {
    use std::io::stdin;
    let mut buf = String::new();
    // println!("Please input maximum user count: ");
    // stdin().read_line(&mut buf).unwrap();
    // USER_COUNT = buf.parse().unwrap();
    println!("Please input initial coin number: ");
    stdin().read_line(&mut buf).unwrap();
    INIT_COIN = buf.parse().unwrap();
    println!("Please input skip add: ");
    stdin().read_line(&mut buf).unwrap();
    SKIP_ADD = buf.parse().unwrap();
} */

fn main() {
    //unsafe { init() }
    let mut player: Vec<Player> = Vec::new();
    player.init();
    while !player.finished() {
        player.input();
        player.calc();
        player.show();
    }
    println!("[ exit ]");
}
