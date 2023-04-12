use crate::rast2vec::space::Pos;

use crate::utility::Color;

pub struct ImageRGBA {
    pub(super) inner: ndarray::Array<Color, ndarray::Dim<[usize; 2]>>,
}

impl ImageRGBA {
    pub fn new(size: Pos) -> Self {
        let inner = ndarray::Array::from_elem((size.y, size.x), Default::default());
        Self { inner }
    }

    pub fn size(&self) -> Pos {
        Pos::new(self.inner.dim().0, self.inner.dim().1)
    }

    pub fn get(&self, pos: Pos) -> Option<Color> {
        self.inner.get((pos.y, pos.x)).map(|&c| c)
    }

    pub fn as_raw(&self) -> &ndarray::Array<Color, ndarray::Dim<[usize; 2]>> {
        &self.inner
    }

    pub fn h(&self) -> usize {
        self.inner.dim().0
    }
    pub fn w(&self) -> usize {
        self.inner.dim().1
    }
}

impl From<ndarray::Array<Color, ndarray::Dim<[usize; 2]>>> for ImageRGBA {
    fn from(inner: ndarray::Array<Color, ndarray::Dim<[usize; 2]>>) -> Self {
        Self { inner }
    }
}
