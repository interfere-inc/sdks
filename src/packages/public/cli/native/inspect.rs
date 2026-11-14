use clap::ArgMatches;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::formatter::OutputPipeline;
use fern_cli_sdk::openapi::AppContext;
use interfere_sdk::api::{GetProblemsRequest, GetProblemsRequestArgs};
use serde_json::json;

pub fn run(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let id = matches.get_one::<String>("problem-id").ok_or_else(|| {
        CliError::Validation("Provide a problem ID from `interfere problems list`.".into())
    })?;
    let body = serde_json::to_value(&GetProblemsRequest {
        args: GetProblemsRequestArgs {
            problem_id: id.clone(),
        },
    })
    .map_err(|error| CliError::Other(error.into()))?;
    let problem = ctx.find_method("problems", "get")?;
    let findings = ctx.find_method("findings", "list-by-problem")?;
    let params = matches
        .get_one::<String>("workspace")
        .map(|workspace| json!({ "workspaceSlug": workspace }).to_string());
    let requests = [problem, findings].map(|method| {
        json!({
            "operationId": method.id,
            "method": method.http_method,
            "pathTemplate": method.path,
            "body": body
        })
    });
    let output = if matches.get_flag("dry-run") {
        json!({
            "dryRun": true,
            "baseUrl": ctx.effective_base_url(),
            "requests": requests
        })
    } else {
        json!({
            "problem": ctx.invoke(problem, params.as_deref(), Some(&body.to_string()), None)?,
            "findings": ctx.invoke(findings, params.as_deref(), Some(&body.to_string()), None)?
        })
    };
    let pipeline = OutputPipeline::from_matches(matches, "interfere")
        .map_err(|error| CliError::Validation(error.to_string()))?;
    pipeline
        .emit(&mut std::io::stdout().lock(), &output, false, true)
        .map_err(|error| CliError::Other(error.into()))
}
