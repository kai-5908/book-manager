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
    '$2b$12$OzCLf9E5ie0VcnwzfFsT7uMs.s9t4daKJ53lVtQGxMmWp0/HbRtlC',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
