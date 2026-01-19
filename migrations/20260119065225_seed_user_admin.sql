-- Add migration script here
INSERT INTO Users (id, name, email, password, role_id)
VALUES (
    UUID_TO_BIN(UUID()),
    'ADMIN-ASRUL',
    'asrulfirdaus93@gmail.com',
    '$2b$12$xR0hwCWlPSjfJ.SFpQ31n./sJu6I4qqJ6zcTXdum8luVsL6NM4L4e',
    (SELECT id FROM Roles WHERE name = 'ADMIN')
);