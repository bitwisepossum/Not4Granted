CREATE TABLE IF NOT EXISTS grant (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,

    name TEXT NOT NULL,
    funder TEXT NOT NULL,
    call_name TEXT,

    status TEXT NOT NULL DEFAULT 'Planning'
        CHECK (
            status IN (
                'Planning',
                'Submitted',
                'Accepted',
                'Rejected'
            )
        ),

    amount_requested INTEGER,
    amount_received INTEGER,
    currency TEXT NOT NULL DEFAULT 'EUR',

    deadline TEXT,
    submitted_at TEXT,
    decision_at TEXT,

    notes TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS manuscript (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,

    title TEXT NOT NULL,
    short_name TEXT,
    journal TEXT,

    status TEXT NOT NULL DEFAULT 'Idea'
        CHECK (
            status IN (
                'Idea',
                'Drafting',
                'Submitted',
                'Revision',
                'Accepted',
                'Published',
                'Rejected'
            )
        ),

    next_action TEXT,

    submitted_at TEXT,
    decision_at TEXT,
    published_at TEXT,

    doi TEXT,
    notes TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS grant_manuscript (
    grant_id INTEGER NOT NULL,
    manuscript_id INTEGER NOT NULL,

    PRIMARY KEY (grant_id, manuscript_id),

    FOREIGN KEY (grant_id)
        REFERENCES grant(id)
        ON DELETE CASCADE,

    FOREIGN KEY (manuscript_id)
        REFERENCES manuscript(id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_grant_status
    ON grant(status);

CREATE INDEX IF NOT EXISTS idx_grant_deadline
    ON grant(deadline);

CREATE INDEX IF NOT EXISTS idx_manuscript_status
    ON manuscript(status);