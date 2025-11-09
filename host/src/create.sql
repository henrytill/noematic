CREATE TABLE IF NOT EXISTS schema_version (
    major INTEGER NOT NULL,
    minor INTEGER NOT NULL,
    patch INTEGER NOT NULL,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sites (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    inner_text TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE VIRTUAL TABLE IF NOT EXISTS sites_fts USING fts5(url, title, inner_text);

CREATE TRIGGER sites_ai AFTER INSERT ON sites
    BEGIN
        INSERT INTO sites_fts (rowid, url, title, inner_text)
        VALUES (new.id, new.url, new.title, new.inner_text);
    END;

CREATE TRIGGER sites_au AFTER UPDATE ON sites
    BEGIN
        UPDATE sites_fts
           SET url = new.url,
               title = new.title,
               inner_text = new.inner_text
         WHERE rowid = old.id;
    END;

CREATE TRIGGER sites_ad AFTER DELETE ON sites
    BEGIN
        DELETE FROM sites_fts
         WHERE rowid = old.id;
    END;
