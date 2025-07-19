pub trait Input {
    fn down(&self) -> bool;
    fn pressed(&self) -> bool;
    fn released(&self) -> bool;
}
