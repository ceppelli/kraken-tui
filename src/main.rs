
#[derive(Debug, Default)]
pub struct Model {
  pub name:String,
  pub index:i32,
}

fn print_msg(msg:String) {
  println!("[print_msg] msg:{:?}", msg);
}

fn print_num(num:i32) {
  println!("[print_num] number:{:?}", num);
}

fn main() {
  let x = Model::default();
  println!("[main] model:{:?}", x);

  print_msg(x.name);
  print_num(x.index);
}


// tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default() -> Result<(), String>{
    let x = Model::default();
    assert_eq!(x.index, 0);
    assert_eq!(x.name, "");
    Ok(())
  }

  // test print_msg
  #[test]
  fn test_print_msg() -> Result<(), String>{
    let x = Model::default();
    print_msg(x.name);
    Ok(())
  }

  // test print_num
  #[test]
  fn test_print_num() -> Result<(), String>{
    let x = Model::default();
    print_num(x.index);
    Ok(())
  }

}