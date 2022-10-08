CREATE TABLE components (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL DEFAULT 'Unnamed',
  component_id TEXT NOT NULL,
  planet TEXT NOT NULL REFERENCES planets(id)
)
