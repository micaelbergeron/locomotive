#![feature(plugin)]
#![plugin(peg_syntax_ext)]

pub mod steamapps;
pub mod acf;

#[cfg(test)]
mod tests {
    #[test]
    fn has_default_steam_lib() {
        use std::path::Path;
        assert_eq!(super::steamapps::find_libraries(), [Path::new("~/.steam/steam/steamapps/")]);
    }
}
