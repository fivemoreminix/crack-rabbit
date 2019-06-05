//! Keeps a back-buffer of terminal cells for editing before updating the terminal.
//! The terminal is also handled by a generic trait that can be applied to any backend supporting
//! its major functions.

pub use rodio;

pub trait Terminal {
    /// Set cell (x, y) with `c`. This expects the upper-left most cell is (x: 0, y: 0). This function
    /// will be called often.
    fn set_cell(&mut self, x: u32, y: u32, c: char);
    fn get_cell(&self, x: u32, y: u32) -> char {
        unimplemented!() // most terminals may not support this
    }
    /// Expects the terminal to render all changes after being called.
    fn update(&mut self);
    /// Returns the number of cell columns.
    fn get_width(&self) -> u32;
    /// Returns the number of cell rows.
    fn get_height(&self) -> u32;
    /// Returns (number of cells wide, number of cells tall).
    fn get_size(&self) -> (u32, u32) {
        (self.get_width(), self.get_height())
    }
    /// User of terminal can send a "handler function" that should be called every time the terminal's
    /// dimensions change, with the new dimensions in columns, rows order.
    fn set_resize_handler<F>(&mut self, handler: F) where F: Fn(u32, u32) {
        unimplemented!() // may not always be relevant
    }
}

pub struct Backbuffer<'a, B: Terminal> {
    backend: &'a mut B,
    buffer: [[char; 32]; 32],
}

impl<'a, B: Terminal> Backbuffer<'a, B> {
    pub fn new(backend: &'a mut B) -> Backbuffer<'a, B> {
        Backbuffer { backend, buffer: [[' '; 32]; 32] }
    }
}

impl<'a, B: Terminal> Terminal for Backbuffer<'a, B> {
    fn set_cell(&mut self, x: u32, y: u32, c: char) {
        self.buffer[x as usize][y as usize] = c;
    }
    fn update(&mut self) {
        // send it to the actual terminal implementation
    }
    fn get_width(&self) -> u32 {
        32
    }
    fn get_height(&self) -> u32 {
        32
    }
    fn set_resize_handler<F>(&mut self, handler: F) where F: Fn(u32, u32) {
        // Call on actual terminal with same function and parameters
        self.backend.set_resize_handler(handler)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        
    }
}