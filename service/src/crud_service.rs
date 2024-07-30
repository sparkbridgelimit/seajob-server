use async_trait::async_trait;
use sea_orm::entity::prelude::*;
use sea_orm::{DbErr, EntityTrait, PrimaryKeyTrait};

use seajob_common::db;

#[async_trait]
pub trait CRUDService<E>
where
    E: EntityTrait,
    E::Model: Send + Sync,
    <E::PrimaryKey as PrimaryKeyTrait>::ValueType: Send + Sync,
{
    async fn find_by_id(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<E::Model>, DbErr>;
    async fn find_all(&self) -> Result<Vec<E::Model>, DbErr>;
}

pub struct CRUDServiceImpl<E>
where
    E: EntityTrait,
    E::Model: Send + Sync,
    <E::PrimaryKey as PrimaryKeyTrait>::ValueType: Send + Sync,
{
    entity: std::marker::PhantomData<E>,
}

impl<E> CRUDServiceImpl<E>
where
    E: EntityTrait,
    E::Model: Send + Sync,
    <E::PrimaryKey as PrimaryKeyTrait>::ValueType: Send + Sync,
{
    pub fn new() -> Self {
        Self {
            entity: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<E> CRUDService<E> for CRUDServiceImpl<E>
where
    E: EntityTrait,
    E::Model: Send + Sync,
    <E::PrimaryKey as PrimaryKeyTrait>::ValueType: Send + Sync,
{
    async fn find_by_id(
        &self,
        id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<E::Model>, DbErr> {
        E::find_by_id(id).one(db::conn()).await
    }

    async fn find_all(&self) -> Result<Vec<E::Model>, DbErr> {
        E::find().all(db::conn()).await
    }
}
