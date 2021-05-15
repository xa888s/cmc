//! Library for parsing crackmes from [crackmes.one](https://crackmes.one)

pub mod error;
pub mod list;
mod macros;
pub mod overview;

pub use scraper::{Html, Selector};

use std::borrow::Cow;
use std::fmt;
use strum::{Display, EnumString};

#[derive(Debug, PartialEq, Clone)]
pub struct CrackMe<'a> {
    name: &'a str,
    author: &'a str,
    language: Language,
    date: &'a str,
    platform: Platform,
    stats: Stats,
    id: &'a str,
    solutions: u64,
    comments: u64,
    description: Option<Cow<'a, str>>,
}

impl<'a> CrackMe<'a> {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn try_set_description<T>(&mut self, description: T) -> Result<(), T>
    where
        T: Into<Cow<'a, str>>,
    {
        if self.description.is_none() {
            self.description = Some(description.into());
            Ok(())
        } else {
            Err(description)
        }
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl<'a> fmt::Display for CrackMe<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "Author: {}", self.author)?;
        writeln!(f, "Language: {}", self.language)?;
        writeln!(f, "Upload: {}", self.date)?;
        writeln!(f, "Platform: {}", self.platform)?;
        writeln!(f, "Quality: {:.1}", self.stats.quality)?;
        writeln!(f, "Difficulty: {:.1}", self.stats.difficulty)?;
        writeln!(f, "Solutions: {}", self.solutions)?;
        writeln!(f, "Comments: {}", self.comments)?;

        if let Some(description) = self.description.as_deref() {
            writeln!(
                f,
                "Description:{}{}",
                // If it is a long description (with new lines) then print it on a seperate line, else
                // print it on the same line
                if description.contains('\n') {
                    '\n'
                } else {
                    ' '
                },
                description
            )?;
        }

        Ok(())
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
