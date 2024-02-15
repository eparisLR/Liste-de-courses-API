-- Your SQL goes here
CREATE TABLE Recipes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE Ingredients (
    id SERIAL PRIMARY KEY,
    name VARCHAR(30) NOT NULL
);

CREATE TABLE  Categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

CREATE TABLE Steps (
    id SERIAL PRIMARY KEY,
    instructions TEXT NOT NULL,
    recipe_id INTEGER REFERENCES Recipes(id)
);

CREATE TABLE RecipesIngredients (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER REFERENCES Recipes(id),
    ingredient_id INTEGER REFERENCES Ingredients(id)
);

CREATE TABLE RecipesCategories (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER REFERENCES Recipes(id),
    category_id INTEGER REFERENCES Categories(id)
);