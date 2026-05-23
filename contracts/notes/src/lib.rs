#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Env,
    String,
    Symbol,
    Vec,
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Habit {
    id: u64,
    habit_name: String,
    target_days: u32,
    streak: u32,
    completed_today: bool,
}

// Key untuk storage
const HABIT_DATA: Symbol = symbol_short!("HABIT");

#[contract]
pub struct HabitTrackerContract;

#[contractimpl]
impl HabitTrackerContract {

    // Ambil semua habit
    pub fn get_habits(env: Env) -> Vec<Habit> {
        return env
            .storage()
            .instance()
            .get(&HABIT_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // Tambah habit baru
    pub fn create_habit(
        env: Env,
        habit_name: String,
        target_days: u32,
    ) -> String {

        let mut habits: Vec<Habit> = env
            .storage()
            .instance()
            .get(&HABIT_DATA)
            .unwrap_or(Vec::new(&env));

        let habit = Habit {
            id: env.prng().gen::<u64>(),
            habit_name,
            target_days,
            streak: 0,
            completed_today: false,
        };

        habits.push_back(habit);

        env.storage()
            .instance()
            .set(&HABIT_DATA, &habits);

        String::from_str(
            &env,
            "Habit berhasil ditambahkan"
        )
    }

    // Check-in habit (menambah streak)
    pub fn check_in_habit(
        env: Env,
        id: u64,
    ) -> String {

        let mut habits: Vec<Habit> = env
            .storage()
            .instance()
            .get(&HABIT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..habits.len() {

            let mut habit = habits.get(i).unwrap();

            if habit.id == id {

                if habit.completed_today {
                    return String::from_str(
                        &env,
                        "Hari ini sudah check-in"
                    );
                }

                habit.streak += 1;
                habit.completed_today = true;

                habits.set(i, habit);

                env.storage()
                    .instance()
                    .set(&HABIT_DATA, &habits);

                return String::from_str(
                    &env,
                    "Check-in berhasil"
                );
            }
        }

        String::from_str(
            &env,
            "Habit tidak ditemukan"
        )
    }

    // Hapus habit
    pub fn delete_habit(
        env: Env,
        id: u64,
    ) -> String {

        let mut habits: Vec<Habit> = env
            .storage()
            .instance()
            .get(&HABIT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..habits.len() {

            if habits.get(i).unwrap().id == id {

                habits.remove(i);

                env.storage()
                    .instance()
                    .set(&HABIT_DATA, &habits);

                return String::from_str(
                    &env,
                    "Habit berhasil dihapus"
                );
            }
        }

        String::from_str(
            &env,
            "Habit tidak ditemukan"
        )
    }
}

mod test;