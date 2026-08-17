CREATE TYPE user_role AS ENUM ('func', 'student', 'admin');

CREATE TABLE users (
	id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	uid TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	email TEXT NOT NULL UNIQUE,
	role user_role NOT NULL DEFAULT 'student',
	password_hash TEXT NOT NULL,
	created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
	last_login_at TIMESTAMPTZ
);

CREATE INDEX idx_users_role ON users(role);
