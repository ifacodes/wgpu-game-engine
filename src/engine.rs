use anyhow::Result;

use crate::window::{Window, WindowTrait};

pub struct Engine {
    window: Window,
    pub counter: u32,
}

impl Engine {
    pub fn new(window: Window) -> Result<Self> {
        Ok(Self { window, counter: 0 })
    }

    pub fn update(&mut self) -> Result<()> {
        self.counter = self.counter.wrapping_add(1);
        if self.counter > 100 {
            self.window.set_title(format!("{}", self.counter).as_str());
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        Ok(())
    }
}
