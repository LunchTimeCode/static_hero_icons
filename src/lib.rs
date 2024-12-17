use include_directory::{include_directory, Dir};
use std::path::Path;

static PROJECT_DIR: Dir<'_> = include_directory!("icons");

pub fn sxt(name: &str) -> &'static str {
    icon(name, "16/solid")
}

pub fn twn(name: &str) -> &'static str {
    icon(name, "20/solid")
}

pub fn twnf_solid(name: &str) -> &'static str {
    icon(name, "24/solid")
}

pub fn twnf_out(name: &str) -> &'static str {
    icon(name, "24/outline")
}

fn icon(name: &str, path: &str) -> &'static str {
    let name = format!("{name}.svg");
    let path = Path::new(path).join(name);
    let file = PROJECT_DIR.get_file(path.clone()).unwrap_or_else(|| {
        panic!(
            "could not find icon with this name: {}",
            path.to_str().unwrap()
        )
    });
    file.contents_utf8().expect("could not read file")
}

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
