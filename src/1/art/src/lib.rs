//! #芸術
//!
//! 芸術的な概念をモデル化するライブラリ。

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// RYBカラーモデルによる主色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }
    #[derive(Debug,PartialEq)]
    /// RYBカラーモデルによる副色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}
pub mod utils {
//    use kinds::*;
    use super::kinds::*;
    ///2つの主色を同じ割合で混合し、副色にする
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_utils_mix() {
        assert_eq!(utils::mix(kinds::PrimaryColor::Red, kinds::PrimaryColor::Yellow), kinds::SecondaryColor::Orange);
    }
}

