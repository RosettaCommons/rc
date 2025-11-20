use yansi::{Color, Painted};

pub use yansi::Paint;

#[allow(dead_code)]
const ORANGE: Color = Color::Rgb(255, 140, 0);

#[allow(dead_code)]
pub trait PaintExt: Paint {
    fn orange(&self) -> Painted<&Self>;
    fn on_orange(&self) -> Painted<&Self>;
}

impl<T: Paint + ?Sized> PaintExt for T {
    fn orange(&self) -> Painted<&Self> {
        self.fg(ORANGE)
    }
    fn on_orange(&self) -> Painted<&Self> {
        self.bg(ORANGE)
    }
}
