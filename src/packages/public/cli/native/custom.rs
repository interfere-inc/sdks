use std::sync::Arc;

use clap::{Arg, ArgAction, Command};
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::auth::LoginContext;
use fern_cli_sdk::openapi::OpenApiBinding;

use crate::login::{select_method, Host, InterfereLoginFlow};

pub fn register(app: CliApp, login: Arc<InterfereLoginFlow>) -> CliApp {
    app.command(
        Command::new("login")
            .about("Sign in to Interfere; opens your browser, or shows a code to enter from any device")
            .arg(
                Arg::new("device")
                    .long("device")
                    .action(ArgAction::SetTrue)
                    .help(
                        "Use device sign-in: print a one-time code and URL to finish signing in from \
                         any browser. Chosen automatically over SSH, in CI, or without a display.",
                    ),
            )
            .arg(
                Arg::new("no-browser")
                    .long("no-browser")
                    .action(ArgAction::SetTrue)
                    .help("Never open a browser from this machine automatically"),
            ),
        Box::new(move |matches, _| {
            let device = matches.get_flag("device");
            let no_browser = matches.get_flag("no-browser");
            let ctx = LoginContext {
                cli_name: "interfere".into(),
                no_browser,
            };
            login.run_with(&ctx, select_method(device, &Host::detect()))
        }),
    )
    .command_under(
        &["problems"],
        Command::new("inspect")
            .about("Read a problem, its timeline, and related findings")
            .arg(Arg::new("problem-id").required(true))
            .arg(
                Arg::new("workspace")
                    .long("workspace")
                    .help("Workspace slug"),
            ),
        OpenApiBinding::handler(super::inspect::run),
    )
}
