-- Add migration script here
CREATE TABLE Roles (
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE Permissions (
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE Role_Permissions (
    role_id BINARY(16),
    permission_id BINARY(16),
    PRIMARY KEY (role_id, permission_id),
    FOREIGN KEY (role_id)
        REFERENCES Roles(id)
        ON DELETE CASCADE,
    FOREIGN KEY (permission_id)
        REFERENCES Permissions(id)
        ON DELETE CASCADE
);

CREATE TABLE Users (
    id BINARY(16) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) COLLATE utf8mb4_unicode_ci NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    role_id BINARY(16),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (role_id)
        REFERENCES Roles(id)
);

CREATE TABLE Kosts (
    id BINARY(16) PRIMARY KEY,
    user_id BINARY(16) NOT NULL,
    kost_name TEXT NOT NULL,
    kost_address TEXT NOT NULL,
    kost_contact VARCHAR(100) COLLATE utf8mb4_unicode_ci NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id)
        REFERENCES Users(id)
        ON DELETE CASCADE
);

CREATE TABLE Rooms (
    id BINARY(16) PRIMARY KEY,
    kost_id BINARY(16) NOT NULL,
    room_number INT NOT NULL,
    UNIQUE (kost_id, room_number),
    room_vacancy BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (kost_id)
        REFERENCES Kosts(id)
        ON DELETE CASCADE
);

CREATE TABLE Bookings (
    id BINARY(16) PRIMARY KEY,
    room_id BINARY(16) NOT NULL,
    user_id BINARY(16) NOT NULL,
    check_in DATETIME NOT NULL,
    check_out DATETIME,
    payment_status ENUM('PENDING', 'PAID', 'OVERDUE', 'CANCELLED') NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (room_id)
        REFERENCES Rooms(id)
        ON DELETE CASCADE,
    FOREIGN KEY (user_id)
        REFERENCES Users(id)
        ON DELETE CASCADE,
    CHECK (check_out IS NULL or check_out > check_in)
);

CREATE INDEX idx_kosts_user_id ON Kosts(user_id);
CREATE INDEX idx_rooms_kost_id ON Rooms(kost_id);
CREATE INDEX idx_bookings_room_id ON Bookings(room_id);
CREATE INDEX idx_bookings_user_id ON Bookings(user_id);