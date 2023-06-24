BEGIN;

CREATE TABLE IF NOT EXISTS db_version (
  version         INTEGER PRIMARY KEY NOT NULL
);
INSERT INTO db_version VALUES (0);

CREATE TABLE IF NOT EXISTS ingredients (
  id              INTEGER PRIMARY KEY NOT NULL,
  name            TEXT NOT NULL,
  energy_density  DOUBLE PRECISION NOT NULL
);

CREATE TABLE IF NOT EXISTS recipes (
  id              INTEGER PRIMARY KEY NOT NULL,
  name            TEXT NOT NULL,
  hidden          BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS categories (
  id              INTEGER PRIMARY KEY NOT NULL,
  name            TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS recipes_ingredients (
  recipe_id       INTEGER NOT NULL,
  version_id      INTEGER NOT NULL,
  ingredient_id   INTEGER NOT NULL,
  list_order      INTEGER NOT NULL,
  quantity        DOUBLE PRECISION NOT NULL,
  measurement     INTEGER NOT NULL,
  PRIMARY KEY (recipe_id, version_id, ingredient_id)
);

CREATE TABLE IF NOT EXISTS recipes_instructions (
  recipe_id       INTEGER NOT NULL,
  version_id      INTEGER NOT NULL,
  step_number     INTEGER NOT NULL,
  step_text       TEXT NOT NULL,
  PRIMARY KEY (recipe_id, version_id, step_number)
);

CREATE TABLE IF NOT EXISTS recipes_categories (
  recipe_id       INTEGER NOT NULL,
  category_id     INTEGER NOT NULL,
  PRIMARY KEY (recipe_id, category_id)
);

COMMIT;
