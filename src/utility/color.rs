#[derive(Default, Clone, Copy, PartialEq, Eq, derive_new::new)]
pub struct Color([u8; 4]);

impl Color {
    pub fn diff(self, other: Color) -> [i16; 4] {
        [
            other.0[0] as i16 - self.0[0] as i16,
            other.0[1] as i16 - self.0[1] as i16,
            other.0[2] as i16 - self.0[2] as i16,
            other.0[3] as i16 - self.0[3] as i16,
        ]
    }
    pub fn diff_l1(self, other: Color) -> u16 {
        (0..4).map(|i| self.0[i].abs_diff(other.0[i]) as u16).sum()
    }
    pub fn diff_li(self, other: Color) -> u16 {
        (0..4)
            .map(|i| self.0[i].abs_diff(other.0[i]) as u16)
            .max()
            .unwrap()
    }

    pub fn format_svg(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2])
    }
}

impl std::ops::Add<Color> for [usize; 4] {
    type Output = Self;

    fn add(self, rhs: Color) -> Self::Output {
        [
            self[0] + rhs.0[0] as usize,
            self[1] + rhs.0[1] as usize,
            self[2] + rhs.0[2] as usize,
            self[3] + rhs.0[3] as usize,
        ]
    }
}

impl From<image::Rgba<u8>> for Color {
    fn from(value: image::Rgba<u8>) -> Self {
        Self::new(value.0)
    }
}

impl Into<image::Rgba<u8>> for Color {
    fn into(self) -> image::Rgba<u8> {
        image::Rgba(self.0)
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color({:02x}{:02x}{:02x}{:02x})",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}
