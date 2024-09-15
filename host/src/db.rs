mod schema_version;

use anyhow::Error;
use rusqlite::{params, Connection, Transaction};

use self::schema_version::SchemaVersion;
use crate::message::{Query, SaveRequestPayload, SearchRequestPayload, Site};

const MSG_INVALID_SCHEMA_VERSION: &str = "Invalid schema version";

const CREATE_SQL: &str = include_str!("create.sql");

const _: () = assert!(!CREATE_SQL.is_empty());

const SELECT_VERSION_TABLE_EXISTS: &str = "\
SELECT EXISTS
(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'schema_version')
";

const SELECT_VERSION_EXISTS: &str = "\
SELECT EXISTS
(SELECT 1 FROM schema_version)
";

const SELECT_LATEST_VERSION: &str = "\
SELECT major, minor, patch
FROM schema_version
ORDER BY applied_at DESC
LIMIT 1
";

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
    let table_exists: bool = tx.query_row(SELECT_VERSION_TABLE_EXISTS, [], |row| row.get(0))?;
    if !table_exists {
        return Ok(None);
    }
    let version_exists: bool = tx.query_row(SELECT_VERSION_EXISTS, [], |row| row.get(0))?;
    if version_exists {
        let maybe_version = select_version(tx)?;
        return Ok(maybe_version);
    }
    Ok(None)
}

fn select_version(tx: &Transaction) -> Result<Option<SchemaVersion>, rusqlite::Error> {
    let mut statement = tx.prepare(SELECT_LATEST_VERSION)?;
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

pub fn search_sites(
    connection: &Connection,
    search_payload: SearchRequestPayload,
    process: impl Fn(Query) -> String,
) -> Result<Vec<Site>, Error> {
    let mut stmt = connection.prepare(
        "\
SELECT s.url, s.title, snippet(sites_fts, 2, '<b>', '</b>', '...', 40)
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
        let url = row.get(0)?;
        let title = row.get(1)?;
        let snippet = row.get(2)?;
        results.push(Site { url, title, snippet });
    }
    Ok(results)
}
