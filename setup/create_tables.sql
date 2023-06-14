CREATE TABLE IF NOT EXISTS ingredients (
  id              INTEGER PRIMARY KEY NOT NULL,
  name            TEXT NOT NULL,
  energy_density  DOUBLE PRECISION NOT NULL
);

CREATE TABLE IF NOT EXISTS recipes (
  id              INTEGER PRIMARY KEY NOT NULL,
  name            TEXT NOT NULL,
);

-- TODO: Add foreign key constraints
CREATE TABLE IF NOT EXISTS recipes_ingredients (
  recipe_id       INTEGER NOT NULL,
  ingredient_id   INTEGER NOT NULL,
  quantity        DOUBLE PRECISION NOT NULL,
  measurement     TEXT NOT NULL
);

-- TODO: Add primary key constraint for (recipe_id, step_number) if possible
CREATE TABLE IF NOT EXISTS recipes_instructions (
  recipe_id       INTEGER NOT NULL,
  step_number     INTEGER NOT NULL,
  step_text       TEXT NOT NULL
);
