
ALTER TABLE todos ADD COLUMN category_id INTEGER NOT NULL REFERENCES categories(id);
