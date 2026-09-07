use std::fmt;
use std::str::FromStr;

use crate::rules::{Context, Domino, Seat, Team};

use super::{error, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sort {
    Context,
    Chair,
    Domino,
    /// Literal-only sorts: they cannot declare roles.
    Number,
    Team,
}

impl Sort {
    pub fn is_role(self) -> bool {
        matches!(self, Self::Context | Self::Chair | Self::Domino)
    }
    pub(crate) fn values(self) -> Vec<Value> {
        match self {
            Self::Context => Context::ALL.into_iter().map(Value::Context).collect(),
            Self::Chair => Seat::ALL.into_iter().map(Value::Chair).collect(),
            Self::Domino => Domino::ALL.into_iter().map(Value::Domino).collect(),
            Self::Number | Self::Team => Vec::new(),
        }
    }
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Context => "context",
            Self::Chair => "chair",
            Self::Domino => "domino",
            Self::Number => "number",
            Self::Team => "team",
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
    Context(Context),
    Chair(Seat),
    Domino(Domino),
    Number(u32),
    Team(Team),
}

impl Value {
    pub fn sort(self) -> Sort {
        match self {
            Self::Context(_) => Sort::Context,
            Self::Chair(_) => Sort::Chair,
            Self::Domino(_) => Sort::Domino,
            Self::Number(_) => Sort::Number,
            Self::Team(_) => Sort::Team,
        }
    }
    pub(crate) fn literal(s: &str) -> Option<Self> {
        if let Ok(d) = s.parse::<Domino>() {
            return Some(Self::Domino(d));
        }
        if let Some(s) = s.strip_prefix('S') {
            return Seat::from_index(s.parse().ok()?).map(Self::Chair);
        }
        if let Some(s) = s.strip_prefix('q') {
            return if s == "*" {
                Some(Self::Context(Context::Called))
            } else {
                Context::from_index(s.parse().ok()?).map(Self::Context)
            };
        }
        match s {
            "T0" => Some(Self::Team(Team::T0)),
            "T1" => Some(Self::Team(Team::T1)),
            _ => s.parse().ok().map(Self::Number),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Context(x) => x.fmt(f),
            Self::Chair(x) => x.fmt(f),
            Self::Domino(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
            Self::Team(x) => x.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Role {
    pub name: String,
    pub sort: Sort,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Role(String),
    Literal(Value),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Atom {
    pub predicate: String,
    pub args: Vec<Term>,
    /// Undefined predicates satisfy neither a positive nor a negative atom.
    pub negated: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Scheme {
    /// Same-sort equality classes. All other same-sort classes are DISTINCT.
    /// Overlapping groups are transitively closed when compiled.
    pub equal: Vec<Vec<String>>,
    pub atoms: Vec<Atom>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Fix {
    pub roles: Vec<Role>,
    pub outputs: Vec<String>,
    pub cases: Vec<Scheme>,
}

pub(crate) fn valid_name(s: &str) -> bool {
    let mut chars = s.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic())
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

#[derive(Debug)]
enum SExpr {
    Word(String),
    List(Vec<SExpr>),
}
impl SExpr {
    fn word(&self) -> Result<&str> {
        match self {
            Self::Word(s) => Ok(s),
            _ => Err(error("expected an identifier or literal")),
        }
    }
    fn list(&self) -> Result<&[SExpr]> {
        match self {
            Self::List(xs) => Ok(xs),
            _ => Err(error("expected a parenthesized form")),
        }
    }
    fn tagged(&self, tag: &str) -> Result<&[SExpr]> {
        let xs = self.list()?;
        if xs.first().map(Self::word).transpose()? != Some(tag) {
            return Err(error(format!("expected ({tag} ...)")));
        }
        Ok(&xs[1..])
    }
}

fn read(tokens: &[String], at: &mut usize, depth: usize) -> Result<SExpr> {
    if depth > 32 {
        return Err(error("query nesting exceeds 32"));
    }
    let token = tokens
        .get(*at)
        .ok_or_else(|| error("unexpected end of query"))?;
    *at += 1;
    match token.as_str() {
        ")" => Err(error("unexpected closing parenthesis")),
        "(" => {
            let mut xs = Vec::new();
            while tokens.get(*at).map(String::as_str) != Some(")") {
                xs.push(read(tokens, at, depth + 1)?);
            }
            *at += 1;
            Ok(SExpr::List(xs))
        }
        _ => Ok(SExpr::Word(token.clone())),
    }
}

fn atom(expr: &SExpr) -> Result<Atom> {
    let mut xs = expr.list()?;
    let mut negated = false;
    if xs.first().map(SExpr::word).transpose()? == Some("not") {
        if xs.len() != 2 {
            return Err(error("not takes exactly one atom"));
        }
        negated = true;
        xs = xs[1].list()?;
    }
    let name = xs.first().ok_or_else(|| error("empty atom"))?.word()?;
    let args = xs[1..]
        .iter()
        .map(|x| {
            let s = x.word()?;
            Ok(Value::literal(s).map_or_else(|| Term::Role(s.to_owned()), Term::Literal))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Atom {
        predicate: name.to_owned(),
        args,
        negated,
    })
}

impl FromStr for Fix {
    type Err = super::Error;
    fn from_str(source: &str) -> Result<Self> {
        if source.len() > 1_048_576 {
            return Err(error("query exceeds 1 MiB"));
        }
        let source = source
            .lines()
            .map(|l| l.split(';').next().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n");
        let source = source.replace('(', " ( ").replace(')', " ) ");
        let tokens: Vec<String> = source.split_whitespace().map(str::to_owned).collect();
        let mut at = 0;
        let tree = read(&tokens, &mut at, 0)?;
        if at != tokens.len() {
            return Err(error("trailing forms after fix"));
        }
        let forms = tree.tagged("fix")?;
        if forms.len() < 2 {
            return Err(error("fix requires roles and out forms"));
        }
        let mut fix = Fix::default();
        for role in forms[0].tagged("roles")? {
            let pair = role.list()?;
            if pair.len() != 2 {
                return Err(error("role syntax is (sort name)"));
            }
            let sort = match pair[0].word()? {
                "context" => Sort::Context,
                "chair" => Sort::Chair,
                "domino" => Sort::Domino,
                _ => return Err(error("role sort must be context, chair, or domino")),
            };
            fix.roles.push(Role {
                name: pair[1].word()?.to_owned(),
                sort,
            });
        }
        fix.outputs = forms[1]
            .tagged("out")?
            .iter()
            .map(|x| x.word().map(str::to_owned))
            .collect::<Result<_>>()?;
        for form in &forms[2..] {
            let mut case = Scheme::default();
            for expr in form.tagged("case")? {
                let xs = expr.list()?;
                if xs.first().map(SExpr::word).transpose()? == Some("same") {
                    if xs.len() < 3 {
                        return Err(error("same requires at least two role names"));
                    }
                    case.equal.push(
                        xs[1..]
                            .iter()
                            .map(|x| x.word().map(str::to_owned))
                            .collect::<Result<_>>()?,
                    );
                } else {
                    case.atoms.push(atom(expr)?);
                }
            }
            fix.cases.push(case);
        }
        Ok(fix)
    }
}

impl fmt::Display for Fix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(fix\n  (roles")?;
        for role in &self.roles {
            write!(f, " ({} {})", role.sort, role.name)?;
        }
        write!(f, ")\n  (out")?;
        for name in &self.outputs {
            write!(f, " {name}")?;
        }
        writeln!(f, ")")?;
        for case in &self.cases {
            write!(f, "  (case")?;
            for group in &case.equal {
                write!(f, "\n    (same {})", group.join(" "))?;
            }
            for atom in &case.atoms {
                write!(
                    f,
                    "\n    {}({}",
                    if atom.negated { "(not " } else { "" },
                    atom.predicate
                )?;
                for arg in &atom.args {
                    match arg {
                        Term::Role(s) => write!(f, " {s}")?,
                        Term::Literal(v) => write!(f, " {v}")?,
                    }
                }
                write!(f, "){}", if atom.negated { ")" } else { "" })?;
            }
            writeln!(f, ")")?;
        }
        write!(f, ")")
    }
}
