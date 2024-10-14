mod schema_version;

use anyhow::Error;
use rusqlite::{params, Connection, Transaction};

use self::schema_version::SchemaVersion;
use crate::message::{
    Query, RemoveRequestPayload, SaveRequestPayload, SearchRequestPayload,
    SearchResponseSitePayload,
};

const MSG_INVALID_SCHEMA_VERSION: &str = "Invalid schema version";

const CREATE_SQL: &str = include_str!("create.sql");

#[allow(clippy::const_is_empty)]
const _: () = assert!(!CREATE_SQL.is_empty());

pub fn init_tables(connection: &mut Connection) -> Result<(), Error> {
    let tx = connection.transaction()?;
    let maybe_version = get_version(&tx)?;
    match maybe_version {
        Some(version) if version == SchemaVersion::CURRENT => {}
        Some(version) if version < SchemaVersion::CURRENT => {
            migrate(&tx, version, SchemaVersion::CURRENT)?;
            insert_version(&tx, SchemaVersion::CURRENT)?;
        }
        Some(_) => {
            return Err(Error::msg(MSG_INVALID_SCHEMA_VERSION));
        }
        None => {
            tx.execute_batch(CREATE_SQL)?;
            insert_version(&tx, SchemaVersion::CURRENT)?;
        }
    }
    tx.commit()?;
    Ok(())
}

fn get_version(tx: &Transaction) -> Result<Option<SchemaVersion>, rusqlite::Error> {
    let table_exists: bool = {
        let query = "SELECT EXISTS (SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'schema_version')";
        tx.query_row(query, [], |row| row.get(0))?
    };
    if !table_exists {
        return Ok(None);
    }
    let version_exists: bool = {
        let query = "SELECT EXISTS (SELECT 1 FROM schema_version)";
        tx.query_row(query, [], |row| row.get(0))?
    };
    if version_exists {
        let maybe_version = select_version(tx)?;
        return Ok(maybe_version);
    }
    Ok(None)
}

fn select_version(tx: &Transaction) -> Result<Option<SchemaVersion>, rusqlite::Error> {
    let mut statement = tx.prepare(
        "\
SELECT major, minor, patch
FROM schema_version
ORDER BY applied_at DESC
LIMIT 1
",
    )?;
    let mut rows = statement.query(())?;
    if let Some(row) = rows.next()? {
        let major: u64 = row.get(0)?;
        let minor: u64 = row.get(1)?;
        let patch: u64 = row.get(2)?;
        let version = SchemaVersion::new(major, minor, patch);
        Ok(Some(version))
    } else {
        Ok(None)
    }
}

fn migrate(
    _tx: &Transaction,
    _from_version: SchemaVersion,
    _to_version: SchemaVersion,
) -> Result<(), rusqlite::Error> {
    Ok(())
}

fn insert_version(
    tx: &Transaction,
    version: SchemaVersion,
) -> Result<SchemaVersion, rusqlite::Error> {
    let mut statement = tx.prepare(
        "\
INSERT INTO schema_version (major, minor, patch)
VALUES (?, ?, ?)
",
    )?;
    statement.execute([version.major(), version.minor(), version.patch()])?;
    Ok(version)
}

pub fn upsert_site(connection: &Connection, save_payload: SaveRequestPayload) -> Result<(), Error> {
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
    statement.execute(params![save_payload.url, save_payload.title, save_payload.inner_text])?;
    Ok(())
}

pub fn remove(connection: &Connection, payload: RemoveRequestPayload) -> Result<(), Error> {
    let mut statement = connection.prepare("DELETE FROM sites WHERE url = ?")?;
    statement.execute([payload.url])?;
    Ok(())
}

pub fn search_sites(
    connection: &Connection,
    search_payload: SearchRequestPayload,
    process: impl Fn(Query) -> String,
) -> Result<(Vec<SearchResponseSitePayload>, bool), Error> {
    let mut stmt = connection.prepare(
        "\
SELECT s.url, s.title, snippet(sites_fts, 2, '<b>', '</b>', '...', 40)
FROM sites_fts
JOIN sites s ON sites_fts.rowid = s.id
WHERE sites_fts MATCH ?
ORDER BY rank
LIMIT ? OFFSET ?
",
    )?;
    let query_string = process(search_payload.query);
    let limit = search_payload.page_length + 1; // extra row for has_more
    let offset = search_payload.page_num * search_payload.page_length;
    let mut rows = stmt.query(params![query_string, limit, offset])?;
    let mut results = Vec::new();
    let mut count = 0usize;
    let mut has_more = false;
    while let Some(row) = rows.next()? {
        count += 1;
        if count > search_payload.page_length {
            has_more = true;
            break;
        }
        let url = row.get(0)?;
        let title = row.get(1)?;
        let snippet = row.get(2)?;
        results.push(SearchResponseSitePayload { url, title, snippet });
    }
    Ok((results, has_more))
}
