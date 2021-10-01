use std::process::Command;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn git_hash(_: TokenStream) -> TokenStream {
    let mut command = Command::new("git");
    command.args(&["rev-parse", "--short", "HEAD"]);

    let output = command.output().unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert!(output.status.code() == Some(0), "{}", stderr);

    (quote! {
        #stdout
    })
    .into()
}
