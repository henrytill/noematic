use rusqlite::{params, Connection, Transaction};
use semver::Version;

use crate::message::{Query, SavePayload, SearchPayload, Site};

const CURRENT_SCHEMA_VERSION: Version = Version::new(0, 1, 0);

const CREATE_SQL: &str = include_str!("create.sql");

const _: () = assert!(!CREATE_SQL.is_empty());

const SELECT_VERSION_TABLE_EXISTS: &str =
    "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = 'schema_version')";

const SELECT_VERSION_EXISTS: &str = "SELECT EXISTS(SELECT 1 FROM schema_version)";

const SELECT_LATEST_VERSION: &str =
    "SELECT major, minor, patch FROM schema_version ORDER BY date_applied DESC LIMIT 1";

pub enum Error {
    Sqlite(rusqlite::Error),
    InvalidVersion,
}

impl From<rusqlite::Error> for Error {
    fn from(other: rusqlite::Error) -> Self {
        Self::Sqlite(other)
    }
}

pub fn init_tables(connection: &mut Connection) -> Result<(), Error> {
    let tx = connection.transaction()?;
    let maybe_version = get_version(&tx)?;
    match maybe_version {
        Some(version) if version == CURRENT_SCHEMA_VERSION => {}
        Some(version) if version < CURRENT_SCHEMA_VERSION => {
            migrate(&tx, version, CURRENT_SCHEMA_VERSION)?;
            insert_version(&tx, CURRENT_SCHEMA_VERSION)?;
        }
        Some(_) => {
            return Err(Error::InvalidVersion);
        }
        None => {
            tx.execute_batch(CREATE_SQL)?;
            insert_version(&tx, CURRENT_SCHEMA_VERSION)?;
        }
    }
    tx.commit()?;
    Ok(())
}

fn get_version(tx: &Transaction) -> Result<Option<Version>, rusqlite::Error> {
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

fn select_version(tx: &Transaction) -> Result<Option<Version>, rusqlite::Error> {
    let mut statement = tx.prepare(SELECT_LATEST_VERSION)?;
    let mut rows = statement.query(())?;
    if let Some(row) = rows.next()? {
        let major: u64 = row.get(0)?;
        let minor: u64 = row.get(1)?;
        let patch: u64 = row.get(2)?;
        let version = Version::new(major, minor, patch);
        Ok(Some(version))
    } else {
        Ok(None)
    }
}

fn migrate(
    _tx: &Transaction,
    _from_version: Version,
    _to_version: Version,
) -> Result<(), rusqlite::Error> {
    Ok(())
}

fn insert_version(tx: &Transaction, version: Version) -> Result<Version, rusqlite::Error> {
    let mut statement = tx.prepare(
        "
        INSERT INTO schema_version (major, minor, patch)
        VALUES (?, ?, ?)
        ",
    )?;
    statement.execute([version.major, version.minor, version.patch])?;
    Ok(version)
}

pub fn upsert_site(connection: &Connection, save_payload: SavePayload) -> Result<(), Error> {
    let mut statement = connection.prepare(
        "
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

pub fn search_sites<F>(
    connection: &Connection,
    search_payload: SearchPayload,
    process: F,
) -> Result<Vec<Site>, Error>
where
    F: Fn(Query) -> String,
{
    let mut stmt = connection.prepare(
        "
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
