use crate::light::{
  color::Color,
  controller::{MemoryController, MemoryControllerExt, U32Memory, U32MemoryController},
  show::State,
  Lights, Utils,
};

use super::Show;

const N: usize = Lights::N;
pub struct QuickShow;
impl Show for QuickShow {
  fn update(&mut self, lights: &mut Lights, _utils: &mut Utils) -> State {
    let mut mem = U32Memory::new();
    let mut ctrl = U32MemoryController::new(lights, &mut mem);

    ctrl.set_all(Color::new(1.0, 0.0, 1.0, 0.0));
    ctrl.display();

    State::Finished
  }
}
