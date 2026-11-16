//! SigmaOS-brand-logo: brand identity and logo model.
//!
//! Models brand assets, color schemes, typography, logo variants,
//! and brand guidelines for SigmaOS.

use std::collections::BTreeMap;
use std::fmt;

/// Color palette entry
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrandColor {
    pub name: String,
    pub hex: String,
    pub rgb: (u8, u8, u8),
    pub variant: ColorVariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorVariant {
    Primary,
    Secondary,
    Accent,
    Background,
    Foreground,
    Success,
    Warning,
    Error,
}

/// Logo variant type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogoVariant {
    Symbolic,
    Wordmark,
    Full,
    Icon,
    Monochrome,
}

/// Logo asset
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogoAsset {
    pub variant: LogoVariant,
    pub svg: String,
    pub width: u32,
    pub height: u32,
    pub description: String,
}

/// Typography specification
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Typography {
    pub name: String,
    pub family: String,
    pub weight: String,
    pub size: u32,
    pub usage: String,
}

/// Brand identity
pub struct BrandIdentity {
    pub distro_name: String,
    pub codename: String,
    pub version: String,
    pub colors: BTreeMap<String, BrandColor>,
    pub logos: BTreeMap<String, LogoAsset>,
    pub typography: BTreeMap<String, Typography>,
}

impl BrandIdentity {
    pub fn new(distro_name: &str, codename: &str, version: &str) -> Self {
        Self {
            distro_name: distro_name.to_string(),
            codename: codename.to_string(),
            version: version.to_string(),
            colors: BTreeMap::new(),
            logos: BTreeMap::new(),
            typography: BTreeMap::new(),
        }
    }

    pub fn add_color(&mut self, color: BrandColor) -> Result<(), BrandError> {
        if color.name.is_empty() {
            return Err(BrandError::EmptyName);
        }
        if self.colors.contains_key(&color.name) {
            return Err(BrandError::Duplicate(color.name.clone()));
        }
        self.colors.insert(color.name.clone(), color);
        Ok(())
    }

    pub fn get_color(&self, name: &str) -> Option<&BrandColor> {
        self.colors.get(name)
    }

    pub fn color_count(&self) -> usize {
        self.colors.len()
    }

    pub fn add_logo(&mut self, name: String, logo: LogoAsset) -> Result<(), BrandError> {
        if name.is_empty() {
            return Err(BrandError::EmptyName);
        }
        if self.logos.contains_key(&name) {
            return Err(BrandError::Duplicate(name.clone()));
        }
        self.logos.insert(name, logo);
        Ok(())
    }

    pub fn get_logo(&self, name: &str) -> Option<&LogoAsset> {
        self.logos.get(name)
    }

    pub fn logo_count(&self) -> usize {
        self.logos.len()
    }

    pub fn add_typography(&mut self, name: String, typo: Typography) -> Result<(), BrandError> {
        if name.is_empty() {
            return Err(BrandError::EmptyName);
        }
        if self.typography.contains_key(&name) {
            return Err(BrandError::Duplicate(name.clone()));
        }
        self.typography.insert(name, typo);
        Ok(())
    }

    pub fn get_typography(&self, name: &str) -> Option<&Typography> {
        self.typography.get(name)
    }

    pub fn typography_count(&self) -> usize {
        self.typography.len()
    }

    pub fn default_symbolic_svg(&self) -> String {
        format!(
            "<svg viewBox='0 0 100 100' xmlns='http://www.w3.org/2000/svg'>\
            <path d='M10,10 L90,10 L50,50 L90,90 L10,90 Z' fill='{}' stroke='{}' stroke-width='2'/>\
            </svg>",
            self.get_color("primary").map_or_else(|| "#6366f1", |c| &c.hex),
            self.get_color("foreground").map_or_else(|| "#ffffff", |c| &c.hex)
        )
    }
}

impl Default for BrandIdentity {
    fn default() -> Self {
        Self::new("SigmaOS", "Sovereign", "1.0.0")
    }
}

/// Brand errors
#[derive(Debug, PartialEq)]
pub enum BrandError {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for BrandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for BrandError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brand_creation() {
        let brand = BrandIdentity::new("SigmaOS", "Sovereign", "1.0.0");
        assert_eq!(brand.distro_name, "SigmaOS");
        assert_eq!(brand.codename, "Sovereign");
        assert_eq!(brand.version, "1.0.0");
    }

    #[test]
    fn test_add_color() {
        let mut brand = BrandIdentity::default();
        let color = BrandColor {
            name: "primary".to_string(),
            hex: "#6366f1".to_string(),
            rgb: (99, 102, 241),
            variant: ColorVariant::Primary,
        };
        assert!(brand.add_color(color).is_ok());
        assert_eq!(brand.color_count(), 1);
    }

    #[test]
    fn test_empty_color_name_rejected() {
        let mut brand = BrandIdentity::default();
        let color = BrandColor {
            name: "".to_string(),
            hex: "#000000".to_string(),
            rgb: (0, 0, 0),
            variant: ColorVariant::Primary,
        };
        assert_eq!(brand.add_color(color), Err(BrandError::EmptyName));
    }

    #[test]
    fn test_duplicate_color_rejected() {
        let mut brand = BrandIdentity::default();
        let color = BrandColor {
            name: "primary".to_string(),
            hex: "#6366f1".to_string(),
            rgb: (99, 102, 241),
            variant: ColorVariant::Primary,
        };
        brand.add_color(color.clone()).unwrap();
        assert_eq!(brand.add_color(color), Err(BrandError::Duplicate("primary".to_string())));
    }

    #[test]
    fn test_add_logo() {
        let mut brand = BrandIdentity::default();
        let logo = LogoAsset {
            variant: LogoVariant::Symbolic,
            svg: "<svg></svg>".to_string(),
            width: 100,
            height: 100,
            description: "Symbolic logo".to_string(),
        };
        assert!(brand.add_logo("symbolic".to_string(), logo).is_ok());
        assert_eq!(brand.logo_count(), 1);
    }

    #[test]
    fn test_default_svg() {
        let mut brand = BrandIdentity::default();
        let primary = BrandColor {
            name: "primary".to_string(),
            hex: "#6366f1".to_string(),
            rgb: (99, 102, 241),
            variant: ColorVariant::Primary,
        };
        brand.add_color(primary).unwrap();
        let svg = brand.default_symbolic_svg();
        assert!(svg.contains("6366f1"));
        assert!(svg.contains("<svg"));
    }

    #[test]
    fn test_typography() {
        let mut brand = BrandIdentity::default();
        let typo = Typography {
            name: "Display".to_string(),
            family: "Inter".to_string(),
            weight: "Bold".to_string(),
            size: 24,
            usage: "Headings".to_string(),
        };
        assert!(brand.add_typography("display".to_string(), typo).is_ok());
        assert_eq!(brand.typography_count(), 1);
    }
}
