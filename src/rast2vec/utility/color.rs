#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Color {
    pub(super) rgba: [u8; 4],
}

impl Color {
    pub fn diff(self, other: Color) -> usize {
        (0..4)
            .map(|n| {
                let d = (self.rgba[n].abs_diff(other.rgba[n])) as usize;
                d * d
            })
            .sum()
    }
}

#[allow(unused)]
impl Color {
    pub fn black() -> Self {
        Self {
            rgba: [0, 0, 0, 255],
        }
    }
    pub fn dark_red() -> Self {
        Self {
            rgba: [100, 0, 0, 255],
        }
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Color(#{:02x}{:02x}{:02x}{:02x}",
            self.rgba[0], self.rgba[1], self.rgba[2], self.rgba[3]
        )
    }
}
