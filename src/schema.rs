// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    ingredients (id) {
        id -> Int4,
        #[max_length = 30]
        name -> Varchar,
    }
}

diesel::table! {
    recipes (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
    }
}

diesel::table! {
    recipescategories (id) {
        id -> Int4,
        recipe_id -> Nullable<Int4>,
        category_id -> Nullable<Int4>,
    }
}

diesel::table! {
    recipesingredients (id) {
        id -> Int4,
        recipe_id -> Nullable<Int4>,
        ingredient_id -> Nullable<Int4>,
    }
}

diesel::table! {
    steps (id) {
        id -> Int4,
        instructions -> Text,
        recipe_id -> Nullable<Int4>,
    }
}

diesel::joinable!(recipescategories -> categories (category_id));
diesel::joinable!(recipescategories -> recipes (recipe_id));
diesel::joinable!(recipesingredients -> ingredients (ingredient_id));
diesel::joinable!(recipesingredients -> recipes (recipe_id));
diesel::joinable!(steps -> recipes (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    ingredients,
    recipes,
    recipescategories,
    recipesingredients,
    steps,
);
