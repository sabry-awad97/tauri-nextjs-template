// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Router export error: {0}")]
    RouterExportError(String),
}

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
}

#[derive(Default)]
struct Database;

impl Database {
    async fn create_user(&self, name: String) -> Result<User, rspc::Error> {
        Ok(User { id: 1, name })
    }
}

#[derive(Default)]
struct Ctx {
    db: Database,
}

// Greet query function
async fn greet(_ctx: Ctx, name: String) -> String {
    // Greeting message
    format!("Hello, {}!", name)
}

fn router() -> rspc::Router<Ctx> {
    rspc::Router::<Ctx>::new()
        .query("greet", |t| {
            t(|ctx, input: String| async move { greet(ctx, input).await })
        })
        .mutation("createUser", |t| {
            t(|ctx, name| async move {
                let user = ctx.db.create_user(name).await?;
                Ok(rspc::selection!(user, { id, name }))
            })
        })
        .build()
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let r = router();
    let export_path = "../app/trpc/bindings.ts";
    if let Err(err) = r.export_ts(export_path) {
        return Err(AppError::RouterExportError(err.to_string()));
    }

    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(r.into(), Ctx::default))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .map_err(|err| AppError::InternalError(format!("Tauri run error: {}", err)))?;

    Ok(())
}
