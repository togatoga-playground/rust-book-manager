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
    '$2b$12$QPYXNL9a4VsBBw4j877gVOt5g1wfjBzA0xrDkDbJQScgQhQk31o76',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
