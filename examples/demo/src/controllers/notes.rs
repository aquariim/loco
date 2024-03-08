#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]
use axum::{extract::Query, response::IntoResponse};
use loco_rs::prelude::*;
use sea_orm::Condition;
use serde::{Deserialize, Serialize};

use crate::{
    models::_entities::notes::{ActiveModel, Column, Entity, Model},
    views::notes::PaginationResponse,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListQueryParams {
    pub title: Option<String>,
    pub content: Option<String>,
    #[serde(flatten)]
    pub pagination: model::query::PaginationQuery,
}

impl Params {
    fn update(&self, item: &mut ActiveModel) {
        item.title = Set(self.title.clone());
        item.content = Set(self.content.clone());
    }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

pub async fn list(
    State(ctx): State<AppContext>,
    Query(params): Query<ListQueryParams>,
) -> Result<impl IntoResponse> {
    let pagination_query = model::query::PaginationQuery {
        page_size: params.pagination.page_size,
        page: params.pagination.page,
    };

    let paginated_notes = model::query::exec::paginate(
        &ctx.db,
        Entity::find(),
        Some(model::query::dsl::with(params.into_query()).build()),
        &pagination_query,
    )
    .await?;

    /*
    if let Some(settings) = &ctx.config.settings {
        let settings = common::settings::Settings::from_json(settings)?;
        println!("allow list: {:?}", settings.allow_list);
    }*/

    format::render()
        .cookies(&[
            cookie::Cookie::new("foo", "bar"),
            cookie::Cookie::new("baz", "qux"),
        ])?
        .etag("foobar")?
        .json(PaginationResponse::response(paginated_notes))
}

pub async fn add(State(ctx): State<AppContext>, Json(params): Json<Params>) -> Result<Json<Model>> {
    let mut item = ActiveModel {
        ..Default::default()
    };
    params.update(&mut item);
    let item = item.insert(&ctx.db).await?;
    format::json(item)
}

pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Json<Model>> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<()> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Json<Model>> {
    format::json(load_item(&ctx, id).await?)
}

impl ListQueryParams {
    #[must_use]
    pub fn into_query(&self) -> Condition {
        let mut condition = model::query::dsl::condition();

        if let Some(title) = &self.title {
            condition = condition.like(Column::Title, title);
        }
        if let Some(content) = &self.content {
            condition = condition.like(Column::Content, content);
        }
        condition.build()
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("notes")
        .add("/", get(list))
        .add("/", post(add))
        .add("/:id", get(get_one))
        .add("/:id", delete(remove))
        .add("/:id", post(update))
}
