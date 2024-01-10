// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(warnings, unused, unused_imports)]
mod prisma;

use std::sync::Arc;

use prisma::{user, PrismaClient};
use prisma_client_rust::{NewClientError, QueryError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Router export error: {0}")]
    RouterExportError(String),

    #[error("Prisma client error: {0}")]
    PrismaClientError(#[from] NewClientError),
}

struct Database {
    prisma: PrismaClient,
}

impl Database {
    fn new(prisma: PrismaClient) -> Self {
        Self { prisma }
    }

    async fn create_user(&self, name: String) -> Result<user::Data, QueryError> {
        let user: user::Data = self.prisma.user().create(name, vec![]).exec().await?;

        // Return user object
        Ok(user)
    }
}

#[derive(Clone)]
struct Ctx {
    db: Arc<Database>,
}

impl Ctx {
    async fn new() -> Result<Self, NewClientError> {
        let client: PrismaClient = PrismaClient::_builder().build().await?;
        let db = Database::new(client);
        Ok(Self { db: db.into() })
    }
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
                match ctx.db.create_user(name).await {
                    Ok(user) => Ok(rspc::selection!(user, { id, display_name })),
                    Err(err) => Err(rspc::Error::with_cause(
                        rspc::ErrorCode::BadRequest,
                        format!("Error creating user: {err}"),
                        err,
                    )),
                }
            })
        })
        .build()
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let ctx = Arc::new(Ctx::new().await?);
    let r = router();
    let export_path = "../app/trpc/bindings.ts";
    if let Err(err) = r.export_ts(export_path) {
        return Err(AppError::RouterExportError(err.to_string()));
    }

    let ctx_clone = Arc::clone(&ctx);
    tauri::Builder::default()
        .plugin(rspc::integrations::tauri::plugin(r.into(), move || {
            (*ctx_clone).clone()
        }))
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .map_err(|err| AppError::InternalError(format!("Tauri run error: {}", err)))?;

    Ok(())
}
