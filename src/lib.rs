#[macro_use]
extern crate diesel;

use chrono::prelude::*;

use lazy_static::lazy_static;
use mime::TEXT_PLAIN_UTF_8;
use uuid::Uuid;

use std::collections::BTreeSet;

mod db;
mod models;
mod schema;
mod util;

pub use db::*;
pub use util::*;

#[derive(Debug, PartialEq)]
pub struct RawJot {
    pub content: String,
    pub creation_date: DateTime<Utc>,
    pub tags: Vec<String>,
}

pub fn insert_jot(conn: &SqliteConnection, jot: &RawJot) {
    lazy_static! {
        static ref UTF_8_MIME: String = TEXT_PLAIN_UTF_8.to_string();
    }

    let mut jot_id = mk_jot_id(&jot);
    let dev_id = get_device_id();
    let creation_date = jot.creation_date.to_rfc3339();

    let mut dup_id: Option<Vec<u8>> = None;

    let jot_count: Result<i64, _> = schema::jots::table
        .filter(schema::jots::jot_id.eq(&jot_id))
        .count()
        .get_result(&*conn);

    match jot_count {
        Ok(num_rows) => {
            if num_rows > 0 {
                dup_id = Some(jot_id.clone());
                jot_id = fmt_uuid(Uuid::new_v4());
            }
        }
        _ => panic!(),
    };

    let new_jot = models::Jot::new(
        jot_id.clone(),
        Some(creation_date.clone()),
        jot.content.as_bytes().to_vec(),
        UTF_8_MIME.clone(),
        dev_id.clone(),
        dup_id,
    );

    let _ = conn.transaction::<_, diesel::result::Error, _>(|| {
        diesel::insert_into(schema::jots::table)
            .values(&new_jot)
            .execute(&*conn)?;

        let mut new_tags: Vec<models::Tag> = Vec::with_capacity(jot.tags.len());
        let mut mappings: Vec<models::Mapping> = Vec::with_capacity(jot.tags.len());

        for tag in jot.tags.iter() {
            let id = mk_tag_id(tag);
            let old_score = match schema::tags::table.find(&id).first::<models::Tag>(&*conn) {
                Ok(t) => t.get_score(),
                _ => 0,
            };

            let new_score = old_score + 1;

            let new_tag = models::Tag::new(
                tag.clone(),
                id.clone(),
                dev_id.clone(),
                Some(creation_date.clone()),
                new_score,
            );

            if old_score == 0 {
                new_tags.push(new_tag);
            } else {
                // TODO: figure out bulk update.
                diesel::update(schema::tags::table.filter(schema::tags::tag_id.eq(&id)))
                    .set(schema::tags::score.eq(new_score))
                    .execute(&*conn)?;
            }

            // now the mapping
            let mapping_id = mk_mapping_id(&jot_id, &id);

            let mapping =
                models::Mapping::new(mapping_id, id, jot_id.clone(), Some(creation_date.clone()));
            mappings.push(mapping);
        }
        diesel::insert_into(schema::tags::table)
            .values(&new_tags)
            .execute(&*conn)?;

        diesel::insert_into(schema::tag_map::table)
            .values(&mappings)
            .execute(&*conn)?;

        Ok(())
    });
}

pub fn parse_tags(tagline: &str) -> Vec<String> {
    let tags: BTreeSet<String> = tagline
        .split(',')
        .map(|t| t.trim().to_owned())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
        .collect();

    tags.into_iter().collect()
}
