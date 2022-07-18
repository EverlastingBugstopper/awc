use apollo_compiler::ApolloCompiler;
use awc::AwcDiagnostic;
use tide::{log, Request};

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    let mut app = tide::new();
    app.with(log::LogMiddleware::new());
    app.at("/").serve_file("./public/index.html")?;
    app.at("/public/*").serve_dir("./public")?;
    app.at("/").post(validate);
    app.listen("[::]:8080").await?;

    Ok(())
}

async fn validate(mut req: Request<()>) -> tide::Result {
    let graphql = req.body_string().await?;
    let awc = ApolloCompiler::new(&graphql);
    let diagnostics: AwcDiagnostic = awc.into();
    Ok(diagnostics.json().into())
}
