CREATE TABLE IF NOT EXISTS documents (
    id SERIAL PRIMARY KEY,
    markdown_content TEXT NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
