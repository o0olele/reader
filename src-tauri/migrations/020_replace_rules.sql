CREATE TABLE replace_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    "group" TEXT,
    pattern TEXT NOT NULL CHECK (length(pattern) > 0),
    replacement TEXT NOT NULL DEFAULT '',
    is_regex INTEGER NOT NULL DEFAULT 1 CHECK (is_regex IN (0, 1)),
    scope TEXT,
    scope_title INTEGER NOT NULL DEFAULT 0 CHECK (scope_title IN (0, 1)),
    scope_content INTEGER NOT NULL DEFAULT 1 CHECK (scope_content IN (0, 1)),
    exclude_scope TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    enabled INTEGER NOT NULL DEFAULT 1 CHECK (enabled IN (0, 1))
);
CREATE INDEX idx_replace_rules_order ON replace_rules(sort_order, id);
