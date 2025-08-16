use std::sync::Arc;

use crate::wgpu_utils::State;

pub struct StateManager {
    window: Arc<winit::window::Window>,
    state: Option<State>,
}

impl StateManager {
    pub fn set_state(&mut self, s: State) {
        println!("StateManager set_state()");
        self.state = Some(s);
    }
    pub fn redraw(&mut self) {
        self.window.request_redraw();
        println!("StateManager redraw()");
    }
}

pub fn get_state_manager(w: Arc<winit::window::Window>) -> StateManager {
    println!("StateManager new()");
    StateManager {
        state: None,
        window: w,
    }
}
