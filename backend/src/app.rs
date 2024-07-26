use axum::http::{
    header::{CONTENT_TYPE, COOKIE},
    HeaderValue, Method,
};
use axum_login::{
    login_required,
    tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use time::Duration;
use tokio::{signal, task::AbortHandle};
use tower_http::cors::CorsLayer;
use tower_sessions::cookie::{Key, SameSite};
use tower_sessions_sqlx_store::{sqlx::SqlitePool, SqliteStore};
use tracing::info;

use crate::{
    db::{auth::AuthBackend, DB},
    error::BackendError,
    routes,
};

#[derive(Clone, Debug)]
pub struct App {
    pub db: DB,
}

impl App {
    pub async fn new() -> Result<Self, BackendError> {
        let db = DB::new("db.sqlite").await?;
        db.create_tables().await?;

        Ok(Self { db })
    }

    pub async fn serve(&self) -> Result<(), BackendError> {
        // Session layer.
        //
        // This uses `tower-sessions` to establish a layer that will provide the session
        // as a request extension.
        let pool = SqlitePool::connect(&self.db.path).await?;
        let session_store = SqliteStore::new(pool);
        session_store.migrate().await?;

        // remove sessions after 1m
        let deletion_task = tokio::task::spawn(
            session_store
                .clone()
                .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
        );

        // Generate a cryptographic key to sign the session cookie.
        let key = Key::generate();

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(SameSite::Lax)
            .with_http_only(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)))
            .with_signed(key);

        // Auth service.
        //
        // This combines the session layer with our backend to establish the auth
        // service which will provide the auth session as a request extension.
        let backend = AuthBackend::new(self.db.clone());
        let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer)
            .with_data_key("auth_data")
            .build();

        let app = routes::protected::router()
            .route_layer(login_required!(AuthBackend))
            .merge(routes::auth::router())
            .layer(auth_layer)
            .layer(
                CorsLayer::new()
                    .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
                    // .allow_origin(Any)
                    .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                    .allow_credentials(true)
                    .allow_headers([CONTENT_TYPE, COOKIE])
                    .allow_private_network(true),
            )
            .with_state(self.clone());

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        info!("Listening on 0.0.0.0:3000");

        // Ensure we use a shutdown signal to abort the deletion task.
        axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
            .await?;

        let _ = deletion_task.await?;

        Ok(())
    }
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
