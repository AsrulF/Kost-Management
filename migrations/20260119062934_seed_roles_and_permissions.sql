-- Add migration script here
INSERT INTO Roles (id, name)
VALUES
(UUID_TO_BIN(UUID()), 'ADMIN'),
(UUID_TO_BIN(UUID()), 'OWNER'),
(UUID_TO_BIN(UUID()), 'MEMBER');

INSERT INTO Permissions (id, name)
VALUES
(UUID_TO_BIN(UUID()), 'room:create'),
(UUID_TO_BIN(UUID()), 'room:update'),
(UUID_TO_BIN(UUID()), 'room:delete'),
(UUID_TO_BIN(UUID()), 'room:view_all'),
(UUID_TO_BIN(UUID()), 'room:view'),
(UUID_TO_BIN(UUID()), 'kost:create'),
(UUID_TO_BIN(UUID()), 'kost:update'),
(UUID_TO_BIN(UUID()), 'kost:delete'),
(UUID_TO_BIN(UUID()), 'kost:view_all'),
(UUID_TO_BIN(UUID()), 'kost:view'),
(UUID_TO_BIN(UUID()), 'user:create'),
(UUID_TO_BIN(UUID()), 'user:update'),
(UUID_TO_BIN(UUID()), 'user:delete'),
(UUID_TO_BIN(UUID()), 'user:view');