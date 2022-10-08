CREATE TABLE users (
  id TEXT PRIMARY KEY,
  created TIMESTAMP,
  username VARCHAR(128) NOT NULL UNIQUE,

  password TEXT NOT NULL,
  reset_token TEXT,
  reset_expiry TIMESTAMP,

  email_address TEXT NOT NULL,
  verified BOOLEAN,
  verification_token TEXT,

  following TEXT[] NOT NULL DEFAULT '{}',
  blocked TEXT[] NOT NULL DEFAULT '{}',
  sessions UUID[] NOT NULL DEFAULT '{}',

  banned BOOLEAN NOT NULL DEFAULT FALSE,
  admin BOOLEAN NOT NULL DEFAULT FALSE,
  notification_setting INTEGER NOT NULL DEFAULT 1,

  cap_waived BOOLEAN NOT NULL DEFAULT FALSE,
  bytes_used BIGINT NOT NULL DEFAULT 0,

  profile_picture TEXT,
  profile_banner TEXT,
  profile_bio VARCHAR(4000),

  tfa_secret TEXT,
  tfa_enabled BOOLEAN NOT NULL DEFAULT FALSE,
  tfa_backup TEXT[] NOT NULL DEFAULT '{}',

  token_geofenced BOOLEAN NOT NULL DEFAULT FALSE,
  token_expires BOOLEAN NOT NULL DEFAULT TRUE,
  token_ip_locked BOOLEAN NOT NULL DEFAULT FALSE
)
