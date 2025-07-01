pub mod bootstrap;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ApplicationEventHandler: Send + Sync {
    async fn on_application_start(&self) -> Result<()> {
        Ok(())
    }

    async fn on_data_update(&self) -> Result<()> {
        Ok(())
    }

    async fn on_application_shutdown(&self) -> Result<()> {
        Ok(())
    }
}

pub struct EventManager {
    pub handler: Box<dyn ApplicationEventHandler>,
}

impl EventManager {
    pub fn new(handler: Box<dyn ApplicationEventHandler>) -> Self {
        Self { handler }
    }

    pub async fn dispatch(&self) -> Result<()> {
        self.handler.on_application_start().await?;
        self.handler.on_data_update().await?;
        self.handler.on_application_shutdown().await?;
        Ok(())
    }
}