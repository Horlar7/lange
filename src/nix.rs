#[derive(Clone, Default)]
pub(crate) struct NixString(String);
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
pub(crate) struct NixMultiLineString(Vec<String>);
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
impl From<Vec<String>> for NixMultiLineString {
    fn from(v: Vec<String>) -> Self {
        Self::new(v)
    }
}
impl From<&[String]> for NixMultiLineString {
    fn from(v: &[String]) -> Self {
        v.to_owned().into()
    }
}
impl From<&str> for NixMultiLineString {
    fn from(s: &str) -> Self {
        s.split('\n')
            .map(str::to_string)
            .collect::<Vec<String>>()
            .into()
    }
}
impl From<String> for NixMultiLineString {
    fn from(s: String) -> Self {
        s.into()
    }
}

#[derive(Clone, Default)]
pub(crate) struct NixList(Vec<String>);
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
impl From<&[&str]> for NixList {
    fn from(v: &[&str]) -> Self {
        Self::new(v.iter().map(|s| s.to_string()).collect())
    }
}

#[derive(Default, askama::Template, derive_builder::Builder)]
#[builder(default, setter(into))]
#[template(path = "shell.nix", escape = "none")]
pub(crate) struct NixShellTemplate {
    name: NixString,
    build_inputs: NixList,
    shell_hook: NixMultiLineString,
}

impl NixShellTemplate {
    pub fn render(&self) -> String {
        askama::Template::render(self).unwrap()
    }
}
