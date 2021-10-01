use std::process::Command;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::input::{CommandInput, ShortInput};

mod input;

fn command_to_tokens(command: &mut Command) -> TokenStream {
    let output = command.output().unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stdout_trimmed = stdout.trim_end();

    assert!(output.status.code() == Some(0), "{}", stderr);

    (quote! {
        #stdout_trimmed
    })
    .into()
}

/// Command and arguments are separate string literals.
/// Outputs a `&'static str` containing the result of the command.
///
/// example: do this
/// ```
/// # use git_version::run_command;
/// assert!(run_command!("echo", "hello") == "hello");
/// ```
/// and don't do this
/// ```ignore
/// assert!(run_command!("echo 'hello'") == "hello");
/// ```
#[proc_macro]
pub fn run_command(command: TokenStream) -> TokenStream {
    command_to_tokens(&mut parse_macro_input!(command as CommandInput).build())
}

/// Outputs a `&'static str` containing the short hash of git HEAD.
/// Optionally choose the length, default is 7.
///
/// examples:
/// ```
/// # use git_version::git_hash_short;
/// assert!(git_hash_short!().len() == 7);
/// ```
/// or
/// ```
/// # use git_version::git_hash_short;
/// assert!(git_hash_short!(9).len() == 9);
/// ```
#[proc_macro]
pub fn git_hash_short(input: TokenStream) -> TokenStream {
    command_to_tokens(&mut parse_macro_input!(input as ShortInput).build())
}

/// Outputs a `&'static str` containing the full hash of git HEAD
///
/// examples:
/// ```
/// # use git_version::git_hash;
/// const COMMIT: &'static str = git_hash!();
/// ```
#[proc_macro]
pub fn git_hash(_: TokenStream) -> TokenStream {
    command_to_tokens(Command::new("git").args(&["rev-parse", "HEAD"]))
}
