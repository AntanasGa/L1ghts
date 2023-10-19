use std::sync::{
  Arc,
  Mutex,
};


#[derive(Clone)]
pub struct Batcher {
  current: Arc<Mutex<i32>>,
}


impl Batcher {
  pub fn new() -> Self {
    Batcher {
      current: Arc::new(Mutex::new(0)),
    }
  }

  pub fn request(&self) {
    let mut lock = self.current.lock().unwrap();
    *lock += 1;
  }

  pub fn pull(&self) -> bool {
    let mut lock = self.current.lock().unwrap();
    let res = *lock > 0;
    *lock = 0;
    res
  }
}
