#[derive(Debug)]
struct C;

#[derive(Debug)]
struct B<'b> {
  c: &'b C,
}

#[derive(Debug)]
struct A<'a> {
  b: B<'a>,
  c: &'a C, // now this is a reference too
}

// impl<'a> A<'a> {
//   fn new() -> A<'a> {
//     let c1 = C;
//     A {
//       c: &c1,
//       b: B { c: &c1 },
//     }
//   }
// }

impl<'a> A<'a> {
  fn new<'b:'a>(c: &'b C) -> A<'a> {
    /*
    这里 c 的 lifetime 必须比 a 长, 不然 c 如果销毁了, a 这个结构体就无法单独存在
    也就是 a 不能超过 c, 所以下面对 c2 借用 (&c2) 的 lifetime 要大于 a2, 在 a2 销毁前, c2 无法被 mut 借用了
    */
    A {
      c: &c,
      b: B { c: &c },
    }
  }
}

pub fn run2() {
  let mut c2 = C;
  let a2 = A::new(&c2);
  // let _ = &mut c2;
  // drop(a2);
}

pub fn run1() {
  let mut c1 = C;
  let b1 = B {
    c: &c1,
  };
  let a1 = A {
    c: &c1,
    b: b1,
  };
  println!("{:?} - {:?} - {:?} - {:?}", a1, a1.c, a1.b, a1.b.c);
  // let _ = &mut c1;
  // drop(a1);
}

pub fn run() {
  run1();
  run2();
}
