INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users(name, email, password_hash, role_id)
SELECT
    'Ui Kozeki',
    'ui.kozeki@trinity.com',
    '$2b$12$Xp9ni3CaqLzgin74tbgENeaZ2zNcPRHsQQBw4hCt.mDTJFsoYW4sa',
    role_id
FROM
    roles
WHERE
    name = 'Admin';