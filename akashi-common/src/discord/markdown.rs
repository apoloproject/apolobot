#![allow(dead_code)]

use std::fmt::Display;

use regex::{Captures, Regex};

/// From [Assyst2](https://github.com/Jacherr/assyst2/blob/master/assyst-common/src/markdown.rs#L17)
pub trait Markdown {
    fn escape_italics(&self) -> String;
    fn escape_bold(&self) -> String;
    fn escape_codestring(&self) -> String;
    fn escape_codeblock(&self, language: impl Display) -> String;
    fn escape_spoiler(&self) -> String;
    fn escape_strikethrough(&self) -> String;
    fn escape_underline(&self) -> String;

    fn italics(&self) -> String;
    fn bold(&self) -> String;
    fn codestring(&self) -> String;
    fn codeblock(&self, language: impl Display) -> String;
    fn spoiler(&self) -> String;
    fn strikethrough(&self) -> String;
    fn underline(&self) -> String;
    fn url(&self, url: impl Display, comment: Option<impl Display>) -> String;
}

fn cut(t: impl Display, to: usize) -> String {
    t.to_string().chars().take(to).collect::<String>()
}

impl<T> Markdown for T
where
    T: Display,
{
    fn escape_italics(&self) -> String {
        Regex::new(r"[_*]")
            .unwrap()
            .replace_all(&cut(self, 1998), |x: &Captures| {
                format!("\\{}", x.get(0).unwrap().as_str())
            })
            .into_owned()
    }
    fn escape_bold(&self) -> String {
        Regex::new(r"\*\*")
            .unwrap()
            .replace_all(&cut(self, 1998), r"\*\*")
            .into_owned()
    }
    fn escape_codestring(&self) -> String {
        Regex::new(r"`")
            .unwrap()
            .replace_all(&cut(self, 1998), r"'")
            .into_owned()
    }
    fn escape_codeblock(&self, language: impl Display) -> String {
        Regex::new(r"```")
            .unwrap()
            .replace_all(
                &cut(self, 1988 - language.to_string().len()),
                "`\u{200b}`\u{200b}`",
            )
            .into_owned()
    }

    fn escape_spoiler(&self) -> String {
        Regex::new(r"__")
            .unwrap()
            .replace_all(&cut(self, 1996), r"\|\|")
            .into_owned()
    }
    fn escape_strikethrough(&self) -> String {
        Regex::new(r"~~")
            .unwrap()
            .replace_all(&cut(self, 1996), r"\~\~")
            .into_owned()
    }
    fn escape_underline(&self) -> String {
        Regex::new(r"\|\|")
            .unwrap()
            .replace_all(&cut(self, 1996), r"\_\_")
            .into_owned()
    }

    fn italics(&self) -> String {
        format!("_{}_", self.escape_italics())
    }

    fn bold(&self) -> String {
        format!("**{}**", self.escape_bold())
    }

    fn codestring(&self) -> String {
        format!("`{}`", self.escape_codestring())
    }

    fn codeblock(&self, language: impl Display) -> String {
        let t = self.escape_codeblock(&language);
        format!("```{language}\n{t}\n```")
    }

    fn spoiler(&self) -> String {
        format!("||{}||", self.escape_italics())
    }

    fn strikethrough(&self) -> String {
        format!("~~{}~~", self.escape_italics())
    }

    fn underline(&self) -> String {
        format!("__{}__", self.escape_italics())
    }

    fn url(&self, url: impl Display, comment: Option<impl Display>) -> String {
        format!(
            "[{self}]({url}{})",
            match comment {
                Some(c) => format!(" '{c}'"),
                None => String::new(),
            }
        )
    }
}
