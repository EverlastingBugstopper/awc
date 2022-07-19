use awc::{ApolloCompiler, AwcDiagnostic};
use tide::{log, Request};

const IS_DOCKER: Option<&str> = option_env!("IS_DOCKER");
const DEV_PUBLIC_PATH: &str = "./awc-web/public";
const DOCKER_PUBLIC_PATH: &str = "./public";

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::start();
    let mut app = tide::new();
    app.with(log::LogMiddleware::new());
    let public_path = if IS_DOCKER.is_some() {
        DOCKER_PUBLIC_PATH
    } else {
        DEV_PUBLIC_PATH
    };
    let index_path = format!("{}/{}", public_path, "index.html");

    log::info!("serving {} at '/'", index_path);
    app.at("/").serve_file(index_path)?;
    log::info!("serving {} at /public/*", public_path);
    app.at("/public/*").serve_dir(public_path)?;
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
