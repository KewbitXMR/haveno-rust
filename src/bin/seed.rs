use anyhow::Result;
use haveno_basic_bootstrap::events::{ApplicationEventHandler, EventManager};
use haveno_basic_bootstrap::events::bootstrap;
struct FilterSetApp;

#[async_trait::async_trait]
impl ApplicationEventHandler for FilterSetApp {
    async fn on_application_start(&self) -> Result<()> {
        bootstrap::run_seed_bootstrap().await
    }

    async fn on_data_update(&self) -> Result<()> {
        println!("🔁 Data update event not yet implemented.");
        Ok(())
    }

    async fn on_application_shutdown(&self) -> Result<()> {
        println!("👋 Application shutting down.");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app: Box<dyn ApplicationEventHandler> = Box::new(FilterSetApp);
    let manager = EventManager::new(app);
    manager.dispatch().await?;
    Ok(())
}