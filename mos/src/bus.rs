use super::Pinout;
pub trait Bus {
    fn read(&mut self, pinout: Pinout) -> Pinout;
    fn write(&mut self, pinout: Pinout) -> Pinout;
}