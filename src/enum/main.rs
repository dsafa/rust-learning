enum Options {
  Num(u32),
  Nothing,
  Str(String),
}

fn main() {
  let o = Options::Num(1);
  match o {
    Options::Num(n) => println!("{}", n),
    Options::Nothing => println!("none"),
    Options::Str(s) => println!("{}", s),
  }
}