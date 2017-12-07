#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }

  fn create_square(size: u32) -> Rectangle {
    return Rectangle {width: size, height: size}
  }
}

fn main() {
  let rect = Rectangle {width: 20, height: 20};
  let rect2 = Rectangle {width: 10, height: 5};
  println!("Area of {:#?} is {}", &rect, rect.area());
  println!("Can hold {:#?}? {}", &rect2, rect.can_hold(&rect2));
  let square = Rectangle::create_square(2);
}