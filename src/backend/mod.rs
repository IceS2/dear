use async_trait::async_trait;
// use enum_dispatch::enum_dispatch;

#[cfg(feature = "sql")]
pub(crate) mod sql;


// #[derive(Debug)]
// #[enum_dispatch(Backend)]
// pub(crate) enum BackendType {
// }

#[async_trait]
// #[enum_dispatch]
pub(crate) trait Backend {
    async fn init(&self);

    async fn save<Entity : Send>(&self, entity: Entity)
    where
        Self: BackendSaveStrategy<Entity>,
    {
        BackendSaveStrategy::save(self, entity).await;
    }

    async fn list<Entity>(&self)
    where
        Self: BackendListStrategy<Entity>,
    {
        BackendListStrategy::list(self).await;
    }
}

#[async_trait]
pub(crate) trait BackendSaveStrategy<Entity> {
    async fn save(&self, entity: Entity);
}

#[async_trait]
pub(crate) trait BackendListStrategy<Entity> {
    async fn list(&self);
}
