CREATE TYPE project_status AS ENUM (
    'published',
    'removed',
    'inactive',
    'active',
    'in_progress',
    'developed'
);

CREATE TABLE projects (
    id         UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    name       TEXT           NOT NULL,
    stack      TEXT[]         NOT NULL DEFAULT '{}',
    status     project_status NOT NULL DEFAULT 'in_progress',
    url        TEXT,
    created_at TIMESTAMPTZ    NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ    NOT NULL DEFAULT now()
);