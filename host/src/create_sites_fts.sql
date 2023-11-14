CREATE VIRTUAL TABLE IF NOT EXISTS sites_fts USING fts5(url, title, inner_text);
