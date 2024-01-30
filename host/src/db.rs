use std::{convert::Infallible, io};

use rusqlite::{params, Connection};

use crate::{
    message::{Query, SavePayload, SearchPayload, Site},
    schema_version::{self, SchemaVersion, SchemaVersioner},
};

const CREATE_SQL: &str = include_str!("create.sql");

const _: () = assert!(!CREATE_SQL.is_empty());

pub enum Error {
    Io(io::Error),
    Sqlite(rusqlite::Error),
    Semver(semver::Error),
    InvalidSchemaVersion,
}

impl From<rusqlite::Error> for Error {
    fn from(other: rusqlite::Error) -> Self {
        Self::Sqlite(other)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

impl From<schema_version::Error> for Error {
    fn from(other: schema_version::Error) -> Self {
        match other {
            schema_version::Error::Io(e) => Self::Io(e),
            schema_version::Error::Semver(e) => Self::Semver(e),
        }
    }
}

pub fn init_tables<A>(connection: &mut Connection, schema_versioner: &mut A) -> Result<(), Error>
where
    A: SchemaVersioner,
    Error: From<A::Error>,
{
    let maybe_version = schema_versioner.read()?;
    match maybe_version {
        Some(version) if version == SchemaVersion::CURRENT => {}
        Some(version) if version < SchemaVersion::CURRENT => {
            migrate(connection, version, SchemaVersion::CURRENT)?;
            schema_versioner.write(&SchemaVersion::CURRENT)?;
        }
        Some(_) => {
            return Err(Error::InvalidSchemaVersion);
        }
        None => {
            connection.execute_batch(CREATE_SQL)?;
            schema_versioner.write(&SchemaVersion::CURRENT)?;
        }
    }
    Ok(())
}

fn migrate(
    _connection: &mut Connection,
    _from_version: SchemaVersion,
    _to_version: SchemaVersion,
) -> Result<(), rusqlite::Error> {
    Ok(())
}

pub fn upsert_site(connection: &Connection, save_payload: SavePayload) -> Result<(), Error> {
    let mut statement = connection.prepare(
        "\
INSERT INTO sites (url, title, inner_text)
VALUES (?, ?, ?)
ON CONFLICT (url) DO UPDATE SET
    title = excluded.title,
    inner_text = excluded.inner_text,
    updated_at = CURRENT_TIMESTAMP
",
    )?;
    statement.execute(params![
        save_payload.url,
        save_payload.title,
        save_payload.inner_text
    ])?;
    Ok(())
}

pub fn search_sites(
    connection: &Connection,
    search_payload: SearchPayload,
    process: impl Fn(Query) -> String,
) -> Result<Vec<Site>, Error> {
    let mut stmt = connection.prepare(
        "\
SELECT s.url, s.title, s.inner_text
FROM sites_fts
JOIN sites s ON sites_fts.rowid = s.id
WHERE sites_fts MATCH ?
ORDER BY rank
",
    )?;
    let query_string = process(search_payload.query);
    let mut rows = stmt.query([query_string])?;
    let mut results = Vec::new();
    while let Some(row) = rows.next()? {
        results.push(Site {
            url: row.get(0)?,
            title: row.get(1)?,
            inner_text: row.get(2)?,
        });
    }
    Ok(results)
}
