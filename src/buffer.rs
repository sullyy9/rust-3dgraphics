use std::ops::{Index, IndexMut};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Color(pub [u8; 4]);

#[derive(Debug, Clone)]
pub struct PixelBuffer {
    pub width: usize,
    pub height: usize,
    data: Box<[u8]>,
}

#[derive(Debug, Clone)]
pub struct ZBuffer {
    pub width: usize,
    pub height: usize,
    data: Box<[usize]>,
}

pub trait ScreenBuffer: Clone {
    fn size(&self) -> (usize, usize);
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Implementations ////////////////////////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////////////////////////////////////////

impl Color {
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl ScreenBuffer for PixelBuffer {
    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl Index<(usize, usize)> for PixelBuffer {
    type Output = [u8];

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let y = (self.height - 1) - index.1;
        let i = (index.0 + (y * self.width)) * 4;
        &self.data[i..(i + 4)]
    }
}

impl Index<(usize, usize)> for ZBuffer {
    type Output = usize;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let i = index.0 + (index.1 * self.width);
        &self.data[i]
    }
}

impl IndexMut<(usize, usize)> for PixelBuffer {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let y = (self.height - 1) - index.1;
        let i = (index.0 + (y * self.width)) * 4;
        &mut self.data[i..(i + 4)]
    }
}

impl IndexMut<(usize, usize)> for ZBuffer {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let i = index.0 + (index.1 * self.width);
        &mut self.data[i]
    }
}

impl PixelBuffer {
    pub fn new<T>(width: T, height: T) -> Self where T: Into<usize> {
        let width = width.into();
        let height = height.into();

        Self {
            width,
            height,
            data: (vec![0; width * height * 4]).into_boxed_slice(),
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl ZBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: (vec![usize::MAX; width * height]).into_boxed_slice(),
        }
    }
}
