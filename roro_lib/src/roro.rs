
#[derive(Debug)]
pub struct Roro {
}

impl Roro {
    /// New Roro
    pub fn new() -> Self {
        println!("-> Roro::new()");
        Self {}
    }
}

#[cfg(test)]
mod tests_roro {
    use super::Roro;

    #[test]
    fn test_roro1() {
        Roro::new();
    }
}
