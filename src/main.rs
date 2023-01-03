

#[derive(Debug, Default)]
pub struct Model {
  pub name:String,
  pub index:i32,
}

fn main() {
  let x = Model::default();
  println!(">> {:?}", x);
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
}