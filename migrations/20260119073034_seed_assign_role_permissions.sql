-- Add migration script here
INSERT INTO Role_Permissions (role_id, permission_id)
SELECT r.id, p.id
FROM Roles r
JOIN Permissions p
WHERE r.name = 'ADMIN';

INSERT INTO Role_Permissions (role_id, permission_id)
SELECT r.id, p.id
FROM Roles r
JOIN Permissions p
WHERE r.name = 'OWNER'
    AND p.name IN (
        'kost:create',
        'kost:update',
        'kost:delete',
        'kost:view'
    );
