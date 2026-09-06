BEGIN TRANSACTION;

DELETE FROM grant_manuscript;
DELETE FROM grant;
DELETE FROM manuscript;

DELETE FROM sqlite_sequence
WHERE name IN ('grant', 'manuscript');


-- Grants
-- Monetary values are stored as integer cents.

INSERT INTO grant (
    name,
    funder,
    call_name,
    status,
    amount_requested,
    amount_received,
    currency,
    deadline,
    submitted_at,
    decision_at,
    notes
)
VALUES
(
    'Urban Pollinator Microgrant',
    'Northbridge Research Trust',
    '2026 Microgrant Call',
    'Planning',
    850000,
    NULL,
    'EUR',
    '2026-09-28',
    NULL,
    NULL,
    'Demonstration data'
),
(
    'Open Tools Seed Funding',
    'Example Science Foundation',
    'Open Tools 2026',
    'Submitted',
    1500000,
    NULL,
    'EUR',
    '2026-10-12',
    '2026-09-01',
    NULL,
    'Demonstration data'
),
(
    'Early Career Mobility Grant',
    'Baltic Academic Council',
    'Mobility Awards 2026',
    'Submitted',
    320000,
    NULL,
    'EUR',
    '2026-11-05',
    '2026-08-25',
    NULL,
    'Demonstration data'
),
(
    'Community Data Pilot',
    'Civic Knowledge Fund',
    'Pilot Funding Round',
    'Accepted',
    2100000,
    1500000,
    'EUR',
    '2026-05-14',
    '2026-05-10',
    '2026-07-18',
    'Demonstration data'
),
(
    'Small Methods Development Grant',
    'Helix Research Society',
    'Methods Development 2026',
    'Rejected',
    1200000,
    NULL,
    'EUR',
    '2026-03-31',
    '2026-03-29',
    '2026-06-12',
    'Demonstration data'
),
(
    'Research Software Support Award',
    'Fictional Open Research Fund',
    'Software Support 2025',
    'Rejected',
    950000,
    NULL,
    'EUR',
    '2025-12-15',
    '2025-12-10',
    '2026-02-03',
    'Demonstration data'
),
(
    'Interdisciplinary Workshop Fund',
    'Example University Network',
    'Workshop Support 2026',
    'Rejected',
    400000,
    NULL,
    'EUR',
    '2026-01-20',
    '2026-01-18',
    '2026-03-02',
    'Demonstration data'
),
(
    'Prototype Evaluation Grant',
    'North Coast Innovation Fund',
    'Prototype Call 2026',
    'Planning',
    1800000,
    NULL,
    'EUR',
    '2026-12-01',
    NULL,
    NULL,
    'Demonstration data'
),
(
    'Conference Participation Award',
    'International Methods Association',
    'Conference Awards 2026',
    'Rejected',
    180000,
    NULL,
    'EUR',
    '2026-08-30',
    '2026-08-12',
    '2026-09-03',
    'Demonstration data'
),
(
    'Exploratory Research Award',
    'Mock Funding Council',
    'Exploratory Research 2026',
    'Rejected',
    2500000,
    NULL,
    'EUR',
    '2026-02-18',
    '2026-02-15',
    '2026-05-06',
    'Demonstration data'
),
(
    'Open Scholarship Infrastructure Grant',
    'Fictional Research Infrastructure Fund',
    'Infrastructure Call 2026',
    'Rejected',
    3200000,
    NULL,
    'EUR',
    '2026-04-22',
    '2026-04-20',
    '2026-07-01',
    'Demonstration data'
),
(
    'Research Exchange Travel Award',
    'Northern Academic Exchange',
    'Travel Awards 2026',
    'Rejected',
    270000,
    NULL,
    'EUR',
    '2026-06-10',
    '2026-06-07',
    '2026-07-22',
    'Demonstration data'
),
(
    'Small Dataset Reuse Award',
    'Example Data Science Trust',
    'Dataset Reuse 2026',
    'Rejected',
    600000,
    NULL,
    'EUR',
    '2026-07-07',
    '2026-07-03',
    '2026-08-14',
    'Demonstration data'
),
(
    'Methods Training Support Grant',
    'Demo Academic Development Fund',
    'Training Support 2026',
    'Submitted',
    550000,
    NULL,
    'EUR',
    '2026-11-30',
    '2026-09-02',
    NULL,
    'Demonstration data'
);


-- Manuscripts

INSERT INTO manuscript (
    title,
    short_name,
    journal,
    status,
    next_action,
    submitted_at,
    decision_at,
    published_at,
    doi,
    notes
)
VALUES
(
    'A Small Study of Very Serious Spreadsheet Problems',
    'Spreadsheet Problems',
    'Journal of Demonstrational Research',
    'Drafting',
    'Finish methods draft',
    NULL,
    NULL,
    NULL,
    NULL,
    'Demonstration data'
),
(
    'Local-First Research Tools in Small Academic Teams',
    'Local-First Tools',
    'Open Methods Quarterly',
    'Submitted',
    'Wait for editor decision',
    '2026-08-18',
    NULL,
    NULL,
    NULL,
    'Demonstration data'
),
(
    'Why Researchers Keep Inventing Their Own Trackers',
    'Research Trackers',
    'Academic Workflow Review',
    'Revision',
    'Address reviewer 2',
    '2026-04-11',
    '2026-07-30',
    NULL,
    NULL,
    'Demonstration data'
),
(
    'Metadata Practices in Fictional Field Studies',
    'Metadata Practices',
    'Data Practice Notes',
    'Accepted',
    'Check proofs',
    '2026-02-20',
    '2026-08-22',
    NULL,
    NULL,
    'Demonstration data'
),
(
    'A Pilot Taxonomy of Deadline-Induced Panic',
    'Deadline Panic',
    'Journal of Entirely Plausible Studies',
    'Idea',
    'Write outline',
    NULL,
    NULL,
    NULL,
    NULL,
    'Demonstration data'
);


-- Demonstration grant-manuscript relationships

INSERT INTO grant_manuscript (grant_id, manuscript_id)
VALUES
    (4, 2),
    (4, 3),
    (1, 5);

COMMIT;
