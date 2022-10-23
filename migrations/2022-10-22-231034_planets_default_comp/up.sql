ALTER TABLE planets
ADD default_component SERIAL NOT NULL REFERENCES components(id);
