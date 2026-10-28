-- ============================================================
-- 1. Root manifest
-- ============================================================

INSERT INTO root_manifest (
    id,
    version_bar,
    site_title
)
VALUES (
    true,
    1,
    'Dioxus Docs'
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 2. Articles
-- ============================================================

INSERT INTO articles (
    id,
    title,
    author,
    created_at,
    updated_at,
    tags
)
VALUES
(
    '11111111-1111-1111-1111-111111111111',
    'Ownership',
    'Dioxus Team',
    now(),
    now(),
    ARRAY['rust', 'memory', 'ownership']
),
(
    '22222222-2222-2222-2222-222222222222',
    'Borrow Checker',
    'Dioxus Team',
    now(),
    now(),
    ARRAY['rust', 'memory', 'borrow-checker']
),
(
    '33333333-3333-3333-3333-333333333333',
    'Getting Started',
    'Dioxus Team',
    now(),
    now(),
    ARRAY['dioxus', 'getting-started']
),
(
    '44444444-4444-4444-4444-444444444444',
    'Signals',
    'Dioxus Team',
    now(),
    now(),
    ARRAY['dioxus', 'signals', 'state']
),
(
    '55555555-5555-5555-5555-555555555555',
    'Components',
    'Dioxus Team',
    now(),
    now(),
    ARRAY['dioxus', 'components']
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 3. Contents — Ownership
-- ============================================================

INSERT INTO contents (
    id,
    article_id,
    "order",
    tags,
    block
)
VALUES
(
    'a0000001-0000-0000-0000-000000000001',
    '11111111-1111-1111-1111-111111111111',
    0,
    ARRAY['rust'],
    '{
        "type": "section",
        "title": "Ownership",
        "paragraph": "Ownership is one of the fundamental concepts of Rust."
    }'::jsonb
),
(
    'a0000001-0000-0000-0000-000000000002',
    '11111111-1111-1111-1111-111111111111',
    1,
    ARRAY['rust', 'memory'],
    '{
        "type": "info",
        "info": "Every value in Rust has an owner."
    }'::jsonb
),
(
    'a0000001-0000-0000-0000-000000000003',
    '11111111-1111-1111-1111-111111111111',
    2,
    ARRAY['rust'],
    jsonb_build_object(
        'type', 'code',
        'title', 'Simple Ownership',
        'code_header', 'Rust',
        'code_content', E'let value = String::from("hello");\nprintln!("{value}");'
    )
),
(
    'a0000001-0000-0000-0000-000000000004',
    '11111111-1111-1111-1111-111111111111',
    3,
    ARRAY['rust'],
    '{
        "type": "warning",
        "warning": "A value can have only one owner at a time."
    }'::jsonb
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 4. Contents — Borrow Checker
-- ============================================================

INSERT INTO contents (
    id,
    article_id,
    "order",
    tags,
    block
)
VALUES
(
    'b0000001-0000-0000-0000-000000000001',
    '22222222-2222-2222-2222-222222222222',
    0,
    ARRAY['rust', 'borrow-checker'],
    '{
        "type": "section",
        "title": "Borrow Checker",
        "paragraph": "The borrow checker verifies references at compile time."
    }'::jsonb
),
(
    'b0000001-0000-0000-0000-000000000002',
    '22222222-2222-2222-2222-222222222222',
    1,
    ARRAY['rust'],
    '{
        "type": "info",
        "info": "References allow you to use a value without taking ownership."
    }'::jsonb
),
(
    'b0000001-0000-0000-0000-000000000003',
    '22222222-2222-2222-2222-222222222222',
    2,
    ARRAY['rust', 'references'],
    jsonb_build_object(
        'type', 'code',
        'title', 'Borrowing a Value',
        'code_header', 'Rust',
        'code_content', E'let value = String::from("hello");\nlet reference = &value;\nprintln!("{reference}");'
    )
),
(
    'b0000001-0000-0000-0000-000000000004',
    '22222222-2222-2222-2222-222222222222',
    3,
    ARRAY['rust'],
    '{
        "type": "warning",
        "warning": "Mutable and immutable borrows cannot violate Rust''s borrowing rules."
    }'::jsonb
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 5. Contents — Getting Started
-- ============================================================

INSERT INTO contents (
    id,
    article_id,
    "order",
    tags,
    block
)
VALUES
(
    'c0000001-0000-0000-0000-000000000001',
    '33333333-3333-3333-3333-333333333333',
    0,
    ARRAY['dioxus'],
    '{
        "type": "section",
        "title": "Introduction",
        "paragraph": "Welcome to Dioxus."
    }'::jsonb
),
(
    'c0000001-0000-0000-0000-000000000002',
    '33333333-3333-3333-3333-333333333333',
    1,
    ARRAY['dioxus'],
    '{
        "type": "info",
        "info": "Dioxus is a Rust framework for building user interfaces."
    }'::jsonb
),
(
    'c0000001-0000-0000-0000-000000000003',
    '33333333-3333-3333-3333-333333333333',
    2,
    ARRAY['dioxus', 'rust'],
    jsonb_build_object(
        'type', 'code',
        'title', 'Hello World',
        'code_header', 'Rust',
        'code_content', E'fn main() {\n    println!("Hello, Dioxus!");\n}'
    )
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 6. Contents — Signals
-- ============================================================

INSERT INTO contents (
    id,
    article_id,
    "order",
    tags,
    block
)
VALUES
(
    'd0000001-0000-0000-0000-000000000001',
    '44444444-4444-4444-4444-444444444444',
    0,
    ARRAY['dioxus', 'signals'],
    '{
        "type": "section",
        "title": "Signals",
        "paragraph": "Signals are reactive state containers."
    }'::jsonb
),
(
    'd0000001-0000-0000-0000-000000000002',
    '44444444-4444-4444-4444-444444444444',
    1,
    ARRAY['dioxus', 'state'],
    '{
        "type": "info",
        "info": "Changing a signal causes dependent components to react."
    }'::jsonb
),
(
    'd0000001-0000-0000-0000-000000000003',
    '44444444-4444-4444-4444-444444444444',
    2,
    ARRAY['dioxus', 'signals'],
    jsonb_build_object(
        'type', 'code',
        'title', 'Signal Example',
        'code_header', 'Rust',
        'code_content', 'let mut count = use_signal(|| 0);'
    )
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 7. Contents — Components
-- ============================================================

INSERT INTO contents (
    id,
    article_id,
    "order",
    tags,
    block
)
VALUES
(
    'e0000001-0000-0000-0000-000000000001',
    '55555555-5555-5555-5555-555555555555',
    0,
    ARRAY['dioxus', 'components'],
    '{
        "type": "section",
        "title": "Components",
        "paragraph": "Components are reusable UI units."
    }'::jsonb
),
(
    'e0000001-0000-0000-0000-000000000002',
    '55555555-5555-5555-5555-555555555555',
    1,
    ARRAY['dioxus'],
    '{
        "type": "info",
        "info": "A component describes a piece of the user interface."
    }'::jsonb
),
(
    'e0000001-0000-0000-0000-000000000003',
    '55555555-5555-5555-5555-555555555555',
    2,
    ARRAY['dioxus', 'components'],
    jsonb_build_object(
        'type', 'code',
        'title', 'Simple Component',
        'code_header', 'Rust',
        'code_content', 'fn Hello() -> Element { rsx! { "Hello" } }'
    )
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 8. NodeMeta — root sections
-- ============================================================

INSERT INTO node_meta (
    id,
    parent_id,
    sort_order,
    title,
    icon,
    description,
    has_children,
    type_node,
    article_id
)
VALUES
(
    'f0000000-0000-0000-0000-000000000001',
    NULL,
    0,
    'Rust',
    NULL,
    'Rust language',
    true,
    'section',
    NULL
),
(
    'f0000000-0000-0000-0000-000000000002',
    NULL,
    1,
    'Dioxus',
    NULL,
    'Dioxus framework',
    true,
    'section',
    NULL
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 9. NodeMeta — Rust / Memory
-- ============================================================

INSERT INTO node_meta (
    id,
    parent_id,
    sort_order,
    title,
    icon,
    description,
    has_children,
    type_node,
    article_id
)
VALUES
(
    'f0000000-0000-0000-0000-000000000003',
    'f0000000-0000-0000-0000-000000000001',
    0,
    'Memory',
    NULL,
    'Memory management in Rust',
    true,
    'section',
    NULL
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 10. NodeMeta — Rust / Memory / Articles
-- ============================================================

INSERT INTO node_meta (
    id,
    parent_id,
    sort_order,
    title,
    icon,
    description,
    has_children,
    type_node,
    article_id
)
VALUES
(
    'f0000000-0000-0000-0000-000000000004',
    'f0000000-0000-0000-0000-000000000003',
    0,
    'Ownership',
    NULL,
    NULL,
    false,
    'article',
    '11111111-1111-1111-1111-111111111111'
),
(
    'f0000000-0000-0000-0000-000000000005',
    'f0000000-0000-0000-0000-000000000003',
    1,
    'Borrow Checker',
    NULL,
    NULL,
    false,
    'article',
    '22222222-2222-2222-2222-222222222222'
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 11. NodeMeta — Dioxus / Articles
-- ============================================================

INSERT INTO node_meta (
    id,
    parent_id,
    sort_order,
    title,
    icon,
    description,
    has_children,
    type_node,
    article_id
)
VALUES
(
    'f0000000-0000-0000-0000-000000000006',
    'f0000000-0000-0000-0000-000000000002',
    0,
    'Getting Started',
    NULL,
    NULL,
    false,
    'article',
    '33333333-3333-3333-3333-333333333333'
),
(
    'f0000000-0000-0000-0000-000000000007',
    'f0000000-0000-0000-0000-000000000002',
    1,
    'Signals',
    NULL,
    NULL,
    false,
    'article',
    '44444444-4444-4444-4444-444444444444'
),
(
    'f0000000-0000-0000-0000-000000000008',
    'f0000000-0000-0000-0000-000000000002',
    2,
    'Components',
    NULL,
    NULL,
    false,
    'article',
    '55555555-5555-5555-5555-555555555555'
)
ON CONFLICT (id) DO NOTHING;


-- ============================================================
-- 12. Root manifest sections
-- ============================================================

INSERT INTO root_manifest_section (
    node_id,
    sort_order
)
VALUES
(
    'f0000000-0000-0000-0000-000000000001',
    0
),
(
    'f0000000-0000-0000-0000-000000000002',
    1
)
ON CONFLICT (node_id) DO NOTHING;
