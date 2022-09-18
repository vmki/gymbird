use crate::models::State;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::fs;
use crate::database::{ID, exercise::Exercise};

pub fn image_name_to_exercise_name<T>(name: T) -> String
where
    T: AsRef<str>,
{
    let name = name.as_ref();
    let mut starting_idx: usize = 0;

    let mut chars: Vec<char> = name.chars().collect();

    if let Some(_) = chars[0].to_digit(10) {
        starting_idx = 2;

        while let Some(_) = chars[starting_idx - 2].to_digit(10) {
            starting_idx += 1;
        }
    }

    name.split_at(starting_idx)
        .1
        .replace("+", " ")
        .split_at(
            name.len()
                - starting_idx
                - 1
                - chars.iter().rev().position(|x| x == &'.').unwrap(),
        )
        .0
        .to_string()
}

pub async fn insert_exercises(state: State) {
    let state_lock = state.lock().await;
    let rows = state_lock.inner().query("SELECT * FROM exercises", &[]).await.unwrap();

    if rows.is_empty() {
        let exercises: Vec<Exercise> = fs::read_dir("../data").unwrap().map(|d| d.unwrap()).map(|entry| {
            entry.file_name()
        }).map(|name| {
            let name_str = name.to_str().unwrap();
            let exercise_name = image_name_to_exercise_name(name_str);
            Exercise::new(exercise_name, name_str.into())
        }).collect();

        for Exercise { id, image_name, name } in exercises {
            state_lock.inner().execute("INSERT INTO exercises (id, name, image_name) VALUES ($1, $2, $3);", &[&id, &name, &image_name]).await.unwrap();
        }
    }
}
