CREATE TABLE articles (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    tags TEXT[] NOT NULL DEFAULT '{}'
);

CREATE TABLE contents (
    id UUID PRIMARY KEY,
    article_id UUID NOT NULL
        REFERENCES articles(id)
        ON DELETE CASCADE,

    "order" INTEGER NOT NULL
        CHECK ("order" >= 0),

    tags TEXT[] NOT NULL DEFAULT '{}',

    block JSONB NOT NULL,

    CHECK (block ? 'type'),
    CHECK (block ->> 'type' IN (
        'code',
        'warning',
        'section',
        'info'
    ))
);

CREATE TABLE node_meta (
    id UUID PRIMARY KEY,

    parent_id UUID
        REFERENCES node_meta(id)
        ON DELETE CASCADE,

    sort_order INTEGER NOT NULL
        CHECK (sort_order >= 0),

    title TEXT NOT NULL,
    icon TEXT,
    description TEXT,

    has_children BOOLEAN NOT NULL DEFAULT false,

    type_node TEXT NOT NULL
        CHECK (type_node IN ('section', 'article')),

    article_id UUID
        REFERENCES articles(id)
        ON DELETE SET NULL,

    CHECK (
        (type_node = 'section' AND article_id IS NULL)
        OR
        (type_node = 'article' AND article_id IS NOT NULL)
    )
);

CREATE TABLE root_manifest (
    id BOOLEAN PRIMARY KEY DEFAULT true CHECK (id),
    version_bar INTEGER NOT NULL CHECK (version_bar >= 0),
    site_title TEXT NOT NULL
);

CREATE TABLE root_manifest_section (
    node_id UUID PRIMARY KEY
        REFERENCES node_meta(id)
        ON DELETE CASCADE,

    sort_order INTEGER NOT NULL
        CHECK (sort_order >= 0)
);
