#[derive(Clone, Default)]
pub struct NixString(String);
impl NixString {
    pub fn new(s: String) -> Self {
        NixString(s)
    }
}
impl std::fmt::Display for NixString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}
impl<S: AsRef<str>> From<S> for NixString {
    fn from(s: S) -> Self {
        NixString::new(s.as_ref().to_owned())
    }
}

#[derive(Clone, Default)]
pub struct NixMultiLineString(Vec<String>);
impl<'d> NixMultiLineString {
    pub fn new(strings: Vec<String>) -> Self {
        NixMultiLineString(strings)
    }
}

impl std::fmt::Display for NixMultiLineString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "''")?;
        self.0
            .iter()
            .map(|s| write!(f, "\n    {}", s))
            .collect::<std::fmt::Result>()?;
        write!(f, "\n  ''")?;
        Ok(())
    }
}
impl std::ops::Add for NixMultiLineString {
    type Output = Self;
    fn add(mut self, mut other: Self) -> Self::Output {
        self.0.append(&mut other.0);
        self
    }
}

impl<S: Clone + Into<String>> From<&[S]> for NixMultiLineString {
    fn from(s: &[S]) -> Self {
        Self::new(s.iter().cloned().map(Into::into).collect())
    }
}

#[derive(Clone, Default)]
pub struct NixList(Vec<String>);
impl<'d> NixList {
    pub fn new(list: Vec<String>) -> Self {
        NixList(list)
    }
}

impl std::fmt::Display for NixList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        self.0
            .iter()
            .map(|s| write!(f, "\n    {}", s))
            .collect::<std::fmt::Result>()?;
        write!(f, "\n  ]")?;
        Ok(())
    }
}

impl std::ops::Add for NixList {
    type Output = Self;
    fn add(mut self, mut other: Self) -> Self::Output {
        self.0.append(&mut other.0);
        self
    }
}

impl<S: Clone + Into<String>> From<&[S]> for NixList {
    fn from(s: &[S]) -> Self {
        Self::new(s.iter().cloned().map(Into::into).collect())
    }
}

#[derive(Default, askama::Template)]
#[template(path = "shell.nix", escape = "none")]
pub struct NixShellTemplate {
    pub name: NixString,
    pub build_inputs: NixList,
    pub shell_hook: NixMultiLineString,
}

impl std::ops::Add for NixShellTemplate {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let name = format!("{}+{}", self.name.0, other.name.0).into();
        Self {
            name,
            build_inputs: self.build_inputs + other.build_inputs,
            shell_hook: self.shell_hook + other.shell_hook,
        }
    }
}

impl std::iter::Sum for NixShellTemplate {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Default::default(), |a, b| a + b)
    }
}
