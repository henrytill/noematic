CREATE TRIGGER sites_au AFTER UPDATE ON sites
BEGIN
    UPDATE sites_fts SET
        url = new.url,
        title = new.title,
        inner_text = new.inner_text
    WHERE rowid = old.id;
END;
