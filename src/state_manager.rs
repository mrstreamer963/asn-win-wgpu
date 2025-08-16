use std::sync::Arc;

use crate::wgpu_utils::{get_state, State};

pub struct StateManager {
    state: Option<State>,
}

impl Drop for StateManager {
    fn drop(&mut self) {
        println!("StateManager drop");
        if self.state.is_some() {
            let _ = self.state.take();
        }
    }
}


impl StateManager {
    pub fn init(&mut self, w: Arc<winit::window::Window>) {
        let s = pollster::block_on(get_state(w.clone()));
        self.set_state(s);
    }


    pub fn set_state(&mut self, s: State) {
        self.state = Some(s);
    }
}

pub fn get_state_manager () -> StateManager{
    StateManager { state: None }
}
