## Static Heroicons in rust

### Example
```rust
fn main() {
    let svg = static_hero_icons:sxt("academic-cap");
    println!("{}", svg);
}
```

### All functions
```rust

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sxt_test() {
        let svg = sxt("academic-cap");
        assert!(svg.contains("svg"));
    }

    #[test]
    fn twn_test() {
        let svg = twn("academic-cap");
        assert!(svg.contains("svg"));
    }

    #[test]
    fn twnf_solid_test() {
        let svg = twnf_solid("academic-cap");
        assert!(svg.contains("svg"));
    }

    #[test]
    fn twnf_out_test() {
        let svg = twnf_out("academic-cap");
        assert!(svg.contains("svg"));
    }
}
```