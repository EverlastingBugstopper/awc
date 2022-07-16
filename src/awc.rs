use apollo_compiler::ApolloCompiler;
use tide::{log, Request};

use awc::AwcDiagnostic;

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    let mut app = tide::new();
    app.with(log::LogMiddleware::new());
    app.at("/").serve_file("public/index.html")?;
    app.at("/public/*").serve_dir("public")?;
    app.at("/validate").get(validate_usage);
    app.at("/validate").post(validate);
    app.listen("[::]:8080").await?;

    Ok(())
}

async fn validate_usage(mut _req: Request<()>) -> tide::Result {
    Ok(
        "usage: cat document.graphql | curl -X POST --data-binary @- https://awc.fly.dev/validate"
            .into(),
    )
}

async fn validate(mut req: Request<()>) -> tide::Result {
    let graphql = req.body_string().await?;
    let awc = ApolloCompiler::new(&graphql);
    let diagnostics: AwcDiagnostic = awc.into();
    Ok(diagnostics.json().into())
}
