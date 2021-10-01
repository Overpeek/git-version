use proc_macro2::Span;
use std::process::Command;
use syn::{
    parse::{Parse, ParseStream},
    LitInt, LitStr, Token,
};

// CommandInput

pub struct CommandInput {
    command: LitStr,
    args: Vec<LitStr>,
}

impl Parse for CommandInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let command = input.parse()?;
        let _ = input.parse::<Option<Token![,]>>()?;
        let mut args = Vec::new();
        while !input.is_empty() {
            args.push(input.parse()?);
            let _ = input.parse::<Option<Token![,]>>()?;
        }

        Ok(Self { command, args })
    }
}

impl CommandInput {
    pub fn build(self) -> Command {
        let mut command = Command::new(self.command.value());
        command.args(self.args.into_iter().map(|s| s.value()));
        command
    }
}

// ShortInput

pub struct ShortInput {
    n: LitInt,
}

impl Parse for ShortInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            Ok(Self {
                n: LitInt::new("7", Span::call_site()),
            })
        } else {
            Ok(Self { n: input.parse()? })
        }
    }
}

impl ShortInput {
    pub fn build(self) -> Command {
        let mut command = Command::new("git");
        command.args(&["rev-parse", format!("--short={}", self.n).as_str(), "HEAD"]);
        command
    }
}
