use winit::dpi::{PhysicalPosition,PhysicalSize};

#[derive(Debug, Default)]
pub struct ImeState {
    /// Whether the IME is enabled.
    enabled: bool,

    /// Current IME preedit.
    preedit: Option<Preedit>,

    /// IME position
    area: (PhysicalPosition<u32>, PhysicalSize<u32>),
}

impl ImeState {
    pub fn new() -> Self {
        Default::default()
    }

    #[inline]
    pub fn set_enabled(&mut self, is_enabled: bool) {
        if is_enabled {
            self.enabled = is_enabled
        } else {
            // clear all and create new
            *self = Default::default();
        }
    }

    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    #[inline]
    pub fn set_preedit(&mut self, preedit: Option<Preedit>) {
        self.preedit = preedit;
    }

    #[inline]
    pub fn preedit(&self) -> Option<&Preedit> {
        self.preedit.as_ref()
    }

    #[inline]
    pub fn set_area(&mut self, p: PhysicalPosition<u32>, s: PhysicalSize<u32> ) {
        self.area = (p,s);
    }

    #[inline]
    pub fn area(&self) -> (PhysicalPosition<u32>, PhysicalSize<u32>) {
        return self.area;
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Preedit {
    /// The preedit text.
    text: String,

    /// Byte offset from cusor start
    /// `None` means that the cursor is invisible.
    cursor_offset: Option<usize>,
}

impl Preedit {
    pub fn new(text: String, cursor_offset: Option<usize>) -> Self {
        Self {
            text,
            cursor_offset,
        }
    }
    pub fn preedit_text(&self) -> &String {
        &self.text
    }
    pub fn cursor_offset(&self) -> Option<usize> {
        if let Some(offset) = self.cursor_offset {
            Some(offset)
        } else {
            None
        }
    }
}
