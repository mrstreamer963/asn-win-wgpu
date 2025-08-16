use std::sync::Arc;

use crate::wgpu_utils::State;

pub struct StateManager {
    state: Option<State>,
}

impl Drop for StateManager {
    fn drop(&mut self) {
        println!("StateManager Drop()")
    }
}

impl StateManager {
    pub fn set_state(&mut self, s: State) {
        println!("StateManager set_state()");
        self.state = Some(s);
    }
    pub fn redraw(&mut self) {
        if let Some(state) = &self.state {
            state.window.request_redraw();
        }

        println!("StateManager redraw()");
    }
}

pub fn get_state_manager(w: Arc<winit::window::Window>) -> StateManager {
    println!("StateManager new()");
    StateManager { state: None }
}
