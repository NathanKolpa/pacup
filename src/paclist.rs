use crate::errors::{PackageLineParseError, PackageListError};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PackageLineKind {
    Package,
    Aur,
}

#[derive(Debug)]
pub struct PackageLine {
    line: String,
    kind: PackageLineKind,
    name_start: usize,
    name_end: usize,
}

impl PackageLine {
    fn empty() -> Self {
        Self {
            kind: PackageLineKind::Package,
            line: String::with_capacity(256),
            name_start: 0,
            name_end: 0,
        }
    }

    pub fn name(&self) -> &str {
        &self.line[self.name_start..=self.name_end]
    }

    fn parse(&mut self) -> Result<bool, PackageLineParseError> {
        enum ParseState {
            Kind,
            Whitespace,
            Name,
            End,
        }

        let mut state = ParseState::Kind;
        let mut bytes_read = 0;

        for (index, char) in self.line.char_indices() {
            bytes_read = index;
            if char == '#' || char == '\n' || char == '\r' {
                break;
            }

            match state {
                ParseState::Whitespace => {
                    if !char.is_whitespace() {
                        state = ParseState::Name;
                        self.name_start = index.clone();
                    }
                }
                _ => {}
            }

            match state {
                ParseState::Kind => {
                    self.kind = match char {
                        '+' => PackageLineKind::Package,
                        '*' => PackageLineKind::Aur,
                        _ => {
                            return Err(PackageLineParseError::UnknownPackageType(char.to_string()))
                        }
                    };

                    state = ParseState::Whitespace;
                }
                ParseState::Name => {
                    if char.is_whitespace() {
                        state = ParseState::End;
                    } else {
                        self.name_end = index.clone();
                    }
                }
                ParseState::End => {
                    if !char.is_whitespace() {
                        return Err(PackageLineParseError::UnexpectedString(char.to_string()));
                    }
                }
                _ => {}
            }
        }

        match state {
            ParseState::Kind => {
                if bytes_read == 0 {
                    return Ok(false);
                }

                return Err(PackageLineParseError::ExpectedPackageType);
            }
            ParseState::Whitespace => return Err(PackageLineParseError::ExpectedPackageName),
            _ => {}
        }

        Ok(true)
    }

    pub fn kind(&self) -> PackageLineKind {
        self.kind
    }
}

pub struct PackageListReader {
    file: BufReader<File>,
    line_buffer: PackageLine,
    current_line: usize,
}

impl PackageListReader {
    pub fn new(file: File) -> Self {
        Self {
            file: BufReader::new(file),
            line_buffer: PackageLine::empty(),
            current_line: 0,
        }
    }

    pub fn next_line(&mut self) -> Option<Result<&PackageLine, PackageListError>> {
        loop {
            let line = &mut self.line_buffer.line;

            line.clear();
            let read_result = self.file.read_line(line);
            self.current_line += 1;

            match read_result {
                Ok(0) => return None,
                Err(err) => return Some(Err(PackageListError::ReadFileError(err))),
                _ => {}
            }

            match self.line_buffer.parse() {
                Ok(false) => continue,
                Err(err) => {
                    return Some(Err(PackageListError::LineParseError(
                        self.current_line,
                        err,
                    )))
                }
                _ => {}
            }

            return Some(Ok(&self.line_buffer));
        }
    }

    pub fn next_line_not_in_set(
        &mut self,
        set: &HashSet<&str>,
    ) -> Option<Result<&PackageLine, PackageListError>> {
        loop {
            let line = match self.next_line()? {
                Ok(l) => l,
                Err(e) => return Some(Err(e)),
            };

            if !set.contains(line.name()) {
                return Some(Ok(&self.line_buffer));
            }
        }
    }
}
