CREATE TRIGGER sites_ad AFTER DELETE ON sites
BEGIN
    DELETE FROM sites_fts WHERE rowid = old.id;
END;
