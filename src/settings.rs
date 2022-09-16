use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use glob::Pattern;
use once_cell::sync::Lazy;

use crate::checks::CheckCode;
use crate::fs;
use crate::pyproject::load_config;

#[derive(Debug, Clone)]
pub struct FilePattern {
    pub basename: Pattern,
    pub absolute: Option<Pattern>,
}

impl FilePattern {
    pub fn single(pattern: &str) -> Self {
        FilePattern {
            basename: Pattern::new(pattern).unwrap(),
            absolute: None,
        }
    }

    pub fn user_provided(pattern: &str) -> Self {
        FilePattern {
            basename: Pattern::new(pattern).expect("Invalid pattern."),
            absolute: Some(
                Pattern::new(&fs::normalize_path(Path::new(pattern)).to_string_lossy())
                    .expect("Invalid pattern."),
            ),
        }
    }
}

#[derive(Debug)]
pub struct Settings {
    pub line_length: usize,
    pub exclude: Vec<FilePattern>,
    pub extend_exclude: Vec<FilePattern>,
    pub select: BTreeSet<CheckCode>,
}

impl Hash for Settings {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.line_length.hash(state);
        for value in self.select.iter() {
            value.hash(state);
        }
    }
}

static DEFAULT_EXCLUDE: Lazy<Vec<FilePattern>> = Lazy::new(|| {
    vec![
        FilePattern::single(".bzr"),
        FilePattern::single(".direnv"),
        FilePattern::single(".eggs"),
        FilePattern::single(".git"),
        FilePattern::single(".hg"),
        FilePattern::single(".mypy_cache"),
        FilePattern::single(".nox"),
        FilePattern::single(".pants.d"),
        FilePattern::single(".ruff_cache"),
        FilePattern::single(".svn"),
        FilePattern::single(".tox"),
        FilePattern::single(".venv"),
        FilePattern::single("__pypackages__"),
        FilePattern::single("_build"),
        FilePattern::single("buck-out"),
        FilePattern::single("build"),
        FilePattern::single("dist"),
        FilePattern::single("node_modules"),
        FilePattern::single("venv"),
    ]
});

impl Settings {
    pub fn from_paths(paths: &[PathBuf]) -> Self {
        let config = load_config(paths);
        let mut settings = Settings {
            line_length: config.line_length.unwrap_or(88),
            exclude: config
                .exclude
                .map(|paths| {
                    paths
                        .iter()
                        .map(|path| FilePattern::user_provided(path))
                        .collect()
                })
                .unwrap_or_else(|| DEFAULT_EXCLUDE.clone()),
            extend_exclude: config
                .extend_exclude
                .map(|paths| {
                    paths
                        .iter()
                        .map(|path| FilePattern::user_provided(path))
                        .collect()
                })
                .unwrap_or_default(),
            select: BTreeSet::from_iter(config.select.unwrap_or_else(|| {
                vec![
                    CheckCode::E402,
                    CheckCode::E501,
                    CheckCode::E711,
                    CheckCode::E712,
                    CheckCode::E713,
                    CheckCode::E714,
                    CheckCode::E721,
                    CheckCode::E722,
                    CheckCode::E731,
                    CheckCode::E741,
                    CheckCode::E742,
                    CheckCode::E743,
                    CheckCode::E902,
                    CheckCode::E999,
                    CheckCode::F401,
                    CheckCode::F403,
                    CheckCode::F406,
                    CheckCode::F407,
                    CheckCode::F541,
                    CheckCode::F601,
                    CheckCode::F602,
                    CheckCode::F621,
                    CheckCode::F622,
                    CheckCode::F631,
                    CheckCode::F632,
                    CheckCode::F633,
                    CheckCode::F634,
                    CheckCode::F701,
                    CheckCode::F702,
                    CheckCode::F704,
                    CheckCode::F706,
                    CheckCode::F707,
                    CheckCode::F722,
                    CheckCode::F821,
                    CheckCode::F822,
                    CheckCode::F823,
                    CheckCode::F831,
                    CheckCode::F841,
                    CheckCode::F901,
                    // Disable refactoring codes by default.
                    // CheckCode::R001,
                    // CheckCode::R002,
                ]
            })),
        };
        if let Some(ignore) = &config.ignore {
            settings.ignore(ignore);
        }
        settings
    }

    pub fn select(&mut self, codes: Vec<CheckCode>) {
        self.select.clear();
        for code in codes {
            self.select.insert(code);
        }
    }

    pub fn ignore(&mut self, codes: &[CheckCode]) {
        for code in codes {
            self.select.remove(code);
        }
    }
}
