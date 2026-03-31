
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Version(usize);

impl Version {
    pub fn new() -> Self {
        Self(0)
    }
    
    pub fn next(self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}
