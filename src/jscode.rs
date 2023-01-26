use wasmedge_quickjs::*;

pub fn execute() {
  let mut ctx = Context::new();

  let mut obj = ctx.new_object();
  obj.set("a", 1.into());
  obj.set("b", ctx.new_string("abc").into());

  ctx.get_global().set("test_obj", obj.into());

  let code = r#"
  print(test_obj.b);
  for (let i=0; i<10; i++) {
    let x = test_obj.a + i;
    print('x', x);
  }
  test_obj;
  "#;
  let r = ctx.eval_global_str(code);
  println!("return value:{:?}", r);
}
