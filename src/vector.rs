use std::ops::{Add, AddAssign, Index, IndexMut, MulAssign};

#[derive(Debug, Clone, derive_new::new)]
pub struct Matrix {
    w: usize,
    h: usize,
    m: Vec<f32>,
}

impl Matrix {
    pub fn w(&self) -> usize {
        self.w
    }
    pub fn h(&self) -> usize {
        self.h
    }
    pub fn buffer_len(&self) -> usize {
        self.w * self.h
    }
    pub fn new_black(w: usize, h: usize) -> Self {
        let m = vec![0.; w * h];
        Self { w, h, m }
    }

    pub fn mm(&self) -> (f32, f32) {
        (
            self.m.iter().fold(f32::INFINITY, |a, &b| a.min(b)),
            self.m.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b)),
        )
    }

    pub fn convolve(&self, core: &Matrix) -> Matrix {
        let w = self.w - core.w + 1;
        let h = self.h - core.h + 1;

        let mut result = Matrix::new_black(w, h);
        for x in 0..w {
            for y in 0..h {
                result[(x, y)] = 0.;
                for dx in 0..core.w {
                    for dy in 0..core.h {
                        result[(x, y)] += self[(x + dx, y + dy)] * core[(dx, dy)]
                    }
                }
            }
        }
        result
    }

    pub fn for_each(&mut self, f: &dyn Fn(f32) -> f32) {
        self.m.iter_mut().for_each(|v| *v = f(*v))
    }
    pub fn for_each_num(&mut self, f: &dyn Fn(f32, (usize, usize)) -> f32) {
        for x in 0..self.w {
            for y in 0..self.h {
                self[(x, y)] = f(self[(x, y)], (x, y));
            }
        }
    }

    pub fn resized(&self, w: usize, h: usize) -> Self {
        assert!(self.w <= w && self.h <= h);
        let mut result = Self::new_black(w, h);
        for x in 0..self.w {
            for y in 0..self.h {
                result[(x, y)] = self[(x, y)];
            }
        }
        result
    }
}

#[derive(Debug, Clone, derive_new::new)]
pub struct DMatrix<const D: usize> {
    layers: [Matrix; D],
}

impl<const D: usize> DMatrix<D> {
    pub fn into_raw(self) -> [Matrix; D] {
        self.layers
    }
    pub fn as_raw(&self) -> &[Matrix; D] {
        &self.layers
    }

    pub fn convolve_layers(&self, core: &Matrix) -> DMatrix<D> {
        let layers: Vec<_> = self.layers.iter().map(|l| l.convolve(core)).collect();
        DMatrix::new(layers.try_into().unwrap())
    }

    pub fn for_each(&mut self, f: &dyn Fn(f32) -> f32) {
        self.layers.iter_mut().for_each(|l| l.for_each(f))
    }

    pub fn sum(&self) -> Matrix {
        let mut result = self[0].clone();
        for i in 1..D {
            result += &self[i];
        }
        result
    }

    pub fn mm(&self) -> (f32, f32) {
        self.layers
            .iter()
            .map(|l| l.mm())
            .fold((f32::INFINITY, f32::NEG_INFINITY), |a, b| {
                (a.0.min(b.0), a.1.max(b.1))
            })
    }

    pub fn new_copies(source: &Matrix) -> Self {
        let copies = (0..D).map(|_| source.clone()).collect::<Vec<_>>();
        Self::new(copies.try_into().unwrap())
    }
}

impl<const D: usize> Index<usize> for DMatrix<D> {
    type Output = Matrix;

    fn index(&self, index: usize) -> &Self::Output {
        &self.layers[index]
    }
}
impl<const D: usize> IndexMut<usize> for DMatrix<D> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.layers[index]
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.m[index.0 + index.1 * self.w]
    }
}
impl Index<usize> for Matrix {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}
impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.m[index.0 + index.1 * self.w]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

impl TryFrom<Vec<Vec<f32>>> for Matrix {
    type Error = ();

    fn try_from(value: Vec<Vec<f32>>) -> Result<Self, Self::Error> {
        let w = value.len();
        let h = value[0].len();
        if value.iter().any(|v| v.len() != h) {
            return Err(());
        }
        let m = value.into_iter().flatten().collect();
        Ok(Self { w, h, m })
    }
}

impl MulAssign<f32> for Matrix {
    fn mul_assign(&mut self, rhs: f32) {
        self.m.iter_mut().for_each(|v| *v *= rhs)
    }
}
impl AddAssign<&Matrix> for Matrix {
    fn add_assign(&mut self, rhs: &Matrix) {
        assert!(self.h == rhs.h && self.w == rhs.w);
        for i in 0..self.buffer_len() {
            self[i] += rhs[i]
        }
    }
}
impl Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(mut self, rhs: Matrix) -> Self::Output {
        self += &rhs;
        self
    }
}

impl<const D: usize> MulAssign<f32> for DMatrix<D> {
    fn mul_assign(&mut self, rhs: f32) {
        self.layers.iter_mut().for_each(|l| *l *= rhs)
    }
}
impl<const D: usize> AddAssign<&Matrix> for DMatrix<D> {
    fn add_assign(&mut self, rhs: &Matrix) {
        self.layers.iter_mut().for_each(|l| *l += &rhs)
    }
}
impl<const D: usize> AddAssign<&DMatrix<D>> for DMatrix<D> {
    fn add_assign(&mut self, rhs: &DMatrix<D>) {
        for i in 0..D {
            self[i] += &rhs[i]
        }
    }
}
impl<const D: usize> Add<&Matrix> for DMatrix<D> {
    type Output = DMatrix<D>;

    fn add(mut self, rhs: &Matrix) -> Self::Output {
        self += rhs;
        self
    }
}
impl<const D: usize> Add<&DMatrix<D>> for DMatrix<D> {
    type Output = DMatrix<D>;

    fn add(mut self, rhs: &DMatrix<D>) -> Self::Output {
        self += rhs;
        self
    }
}
