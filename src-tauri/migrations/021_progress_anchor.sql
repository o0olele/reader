ALTER TABLE reading_progress ADD COLUMN anchor_index INTEGER NOT NULL DEFAULT -1;
ALTER TABLE reading_progress ADD COLUMN anchor_ratio REAL NOT NULL DEFAULT 0;
