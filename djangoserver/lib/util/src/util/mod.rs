
pub mod comp;
pub mod selectors;
pub mod distance;

#[cfg(test)]
mod tests {
    #[test]
    fn comp_works() {
        use comp::Comp;
        use newtypes::Km;
        let c = Comp::new(Km::from_f64_checked(1.0), Km::from_f64(1.0));
        let d = Comp::new(Km::from_f64_checked(2.0), Km::from_f64(0.5));
        assert!(d > c);
        let e = Comp::new(None, Km::from_f64(0.5));
        assert!(e > d);
    }
}
