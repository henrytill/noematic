CREATE TRIGGER sites_ai AFTER INSERT ON sites
BEGIN
    INSERT INTO sites_fts (rowid, url, title, inner_text) VALUES (new.id, new.url, new.title, new.inner_text);
END;
