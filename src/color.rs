//! Currently contains:
//!  - Common ANSI terminal colors
//! 
//! Will soon also contain:
//!  - RGB wrapper type
//!  - Color posterization (palettes)
//!  - Generic colors for interoperability between terminal implementations

pub type RGB = [u8; 3]; // Placeholder RGB "wrapper"

pub const ANSI_COLORS: [RGB; 9] = [
    [0,   0,   0],   // black
    [255, 0,   0],   // red
    [0,   255, 0],   // green
    [255, 255, 0],   // yellow
    [0,   0,   255], // blue
    [255, 0,   255], // magenta
    [0,   255, 255], // cyan
    [211, 211, 211], // grey
    [255, 255, 255], // white
];

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ANSIColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Grey,
    White,
}

impl ANSIColor {
    pub fn as_rgb(&self) -> &RGB {
        &ANSI_COLORS[self as usize]
    }
    //! Receives an RGB array (`[u8; 3]`) reference and returns a terminal safe (ANSI) color by rounding to the nearest RGB color value.
    pub fn posterize(rgb: &RGB) -> ANSIColor {
        
    }
}
