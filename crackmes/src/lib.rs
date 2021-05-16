//! Library for parsing crackmes from [crackmes.one](https://crackmes.one)

pub mod error;
pub mod list;
mod macros;
pub mod overview;

pub use scraper::{Html, Selector};

use std::fmt;
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BaseCrackme<'html> {
    name: &'html str,
    author: &'html str,
    language: Language,
    date: &'html str,
    platform: Platform,
    stats: Stats,
    id: &'html str,
    solutions: u64,
    comments: u64,
}

impl<'html> fmt::Display for BaseCrackme<'html> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Language: {}", self.language)?;
        writeln!(f, "Upload: {}", self.date)?;
        writeln!(f, "Platform: {}", self.platform)?;
        writeln!(f, "Quality: {:.1}", self.stats.quality)?;
        writeln!(f, "Difficulty: {:.1}", self.stats.difficulty)?;
        writeln!(f, "Solutions: {}", self.solutions)?;
        writeln!(f, "Comments: {}", self.comments)
    }
}

// we allow this so the mapping is more one to one
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, EnumString, Display, Clone)]
pub enum Platform {
    DOS,
    #[strum(serialize = "macos", serialize = "Mac OS X")]
    MacOSX,
    #[strum(serialize = "multiplatform", serialize = "Multiplatform")]
    Multiplatform,
    #[strum(
        serialize = "linux",
        serialize = "unix",
        serialize = "Unix/linux etc.",
        serialize = "Unix/Linux"
    )]
    UnixLinux,
    #[strum(serialize = "windows", serialize = "Windows")]
    Windows,
    #[strum(serialize = "Windows 2000/XP only")]
    Windows2000XP,
    #[strum(serialize = "Windows 7 Only")]
    Windows7,
    #[strum(serialize = "Windows Vista Only")]
    WindowsVista,
    #[strum(serialize = "other", serialize = "Unspecified/other")]
    Other,
}

#[derive(Debug, PartialEq, EnumString, Display, Clone)]
pub enum Language {
    #[strum(serialize = "cpp", serialize = "C/C++")]
    COrCPlusPlus,
    Assembler,
    Java,
    #[strum(serialize = "vb", serialize = "(Visual) Basic")]
    VisualBasic,
    #[strum(serialize = "Borland Delphi")]
    BorlandDelphi,
    #[strum(serialize = "Turbo Pascal")]
    TurboPascal,
    #[strum(serialize = "dotnet", serialize = ".NET")]
    DotNet,
    #[strum(serialize = "other", serialize = "Unspecified/other")]
    Other,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Stats {
    pub quality: f32,
    pub difficulty: f32,
}

impl Stats {
    pub fn new(quality: f32, difficulty: f32) -> Stats {
        Stats {
            quality,
            difficulty,
        }
    }
}
