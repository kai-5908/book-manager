INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$nCQESDIr.Cv/JOL/dqA7EeC1sLHedskz5hXXLHwmcTNJXtlEQNkxO',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
