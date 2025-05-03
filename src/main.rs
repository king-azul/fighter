use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use rusqlite::{params, Connection, Result};
use std::io::{self, Write};
use std::{thread, time::Duration};

fn populate_db() -> Result<()> {
    // Connect to the store.db file
    let conn = Connection::open("store.db")?;

    // Create the countries table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS countries (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            ticket_price REAL NOT NULL
        )",
        [],
    )?;

    // Insert example countries
    let countries = vec![
        ("Barcelona", 25.0),
        ("Paris", 30.0),
        ("London", 28.5),
        ("Berlin", 27.0),
        ("Lisbon", 22.5),
    ];

    for (name, price) in countries {
        conn.execute(
            "INSERT OR IGNORE INTO countries (name, ticket_price) VALUES (?1, ?2)",
            params![name, price],
        )?;
    }

    println!("Migration and data insertion complete.");
    Ok(())
}

const ARENA_WIDTH: usize = 20;
const ARENA_HEIGHT: usize = 10;
const MOVE_DELAY_MS: u64 = 300;
const FIGHTERS_COUNT: u32 = 4;

#[derive(Clone, Debug)]
struct Location {
    name: String,
    flight_cost: f64,
}

impl Location {
    fn new(name: String, flight_cost: f64) -> Self {
        Location { name, flight_cost }
    }
}

#[derive(Clone, Debug)]
struct Fighter {
    name: String,
    health: i32,
    attack: i32,
    position: (usize, usize),
}

impl Fighter {
    fn new(name: String, health: i32, attack: i32, position: (usize, usize)) -> Self {
        Fighter {
            name,
            health,
            attack,
            position,
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn move_randomly(&mut self, arena: &[[char; ARENA_WIDTH]; ARENA_HEIGHT]) {
        let mut rng = rand::rng();
        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up
        let dir = dirs[rng.random_range(0..4)];

        let new_x = self.position.0 as i32 + dir.0;
        let new_y = self.position.1 as i32 + dir.1;

        // Check if the new position is within bounds
        if new_x >= 0 && new_x < ARENA_WIDTH as i32 && new_y >= 0 && new_y < ARENA_HEIGHT as i32 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;

            // Only move if the position is empty (contains a space)
            if arena[new_y][new_x] == ' ' {
                self.position = (new_x, new_y);
            }
        }
    }

    // fn attack_fighter(&self, other: &mut Fighter) {
    //     other.health -= self.attack;
    // }
}

fn create_arena() -> [[char; ARENA_WIDTH]; ARENA_HEIGHT] {
    println!("Opening the arena...");
    [[' '; ARENA_WIDTH]; ARENA_HEIGHT]
}

fn update_arena(arena: &mut [[char; ARENA_WIDTH]; ARENA_HEIGHT], fighters: &[Fighter]) {
    // Clear the arena
    for row in arena.iter_mut() {
        for cell in row.iter_mut() {
            *cell = ' ';
        }
    }

    // Place fighters
    for fighter in fighters {
        if fighter.is_alive() {
            let (x, y) = fighter.position;
            arena[y][x] = fighter.name.chars().next().unwrap(); // Messy way to get first char in name
        }
    }
}

fn print_arena(arena: &[[char; ARENA_WIDTH]; ARENA_HEIGHT], fighters: &[Fighter]) {
    // Clear the terminal
    io::stdout().execute(Clear(ClearType::All)).unwrap();
    io::stdout().execute(cursor::MoveTo(0, 0)).unwrap();

    // Print top border
    println!("┌{}┐", "─".repeat(ARENA_WIDTH));

    // Print arena
    for row in arena {
        print!("│");
        for &cell in row {
            print!("{}", cell);
        }
        println!("│");
    }

    // Print bottom border
    println!("└{}┘", "─".repeat(ARENA_WIDTH));

    // Print fighter stats
    println!("\nFighter Stats:");
    for fighter in fighters {
        println!(
            "{}: Health: {}, Attack: {}{}",
            fighter.name,
            fighter.health,
            fighter.attack,
            if !fighter.is_alive() {
                " (DEFEATED)"
            } else {
                ""
            }
        );
    }

    io::stdout().flush().unwrap();
}

fn print_combatlog(log: &Vec<String>) {
    for s in log {
        println!("{s}");
    }
}

fn check_for_battles(fighters: &mut [Fighter]) -> Vec<String> {
    // Create a vector of indices for alive fighters
    let alive_indices: Vec<usize> = fighters
        .iter()
        .enumerate()
        .filter(|(_, f)| f.is_alive())
        .map(|(i, _)| i)
        .collect();

    // Check for battles
    let mut turn_combatlog: Vec<String> = vec![];
    for i in 0..alive_indices.len() {
        for j in (i + 1)..alive_indices.len() {
            let idx_i = alive_indices[i];
            let idx_j = alive_indices[j];

            // If fighters are at the same position, they battle
            if fighters[idx_i]
                .position
                .0
                .abs_diff(fighters[idx_j].position.0)
                <= 1
                && fighters[idx_i]
                    .position
                    .1
                    .abs_diff(fighters[idx_j].position.1)
                    <= 1
            {
                // Both fighters attack each other
                let attack_i = fighters[idx_i].attack;
                fighters[idx_j].health -= attack_i;

                let attack_j = fighters[idx_j].attack;
                fighters[idx_i].health -= attack_j;

                let combatlog_entry: String = format!(
                    "{} and {} fought! {} dealt {} damage and {} dealt {} damage!",
                    fighters[idx_i].name,
                    fighters[idx_j].name,
                    fighters[idx_i].name,
                    attack_i,
                    fighters[idx_j].name,
                    attack_j,
                );

                turn_combatlog.push(combatlog_entry);
            }
        }
    }
    turn_combatlog
}

fn get_locations_from_db() -> Result<Vec<Location>> {
    let conn = Connection::open("store.db")?;
    let mut locations: Vec<Location> = vec![];

    let mut stmt =
        conn.prepare("SELECT name, ticket_price FROM countries ORDER BY RANDOM() LIMIT ?1")?;

    let db_vec = stmt
        .query_map(params![FIGHTERS_COUNT], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })?
        .collect::<Result<Vec<_>>>()?;
    for l in db_vec {
        locations.push(Location::new(l.0, l.1));
    }

    Ok(locations)
}

fn place_fighters_initially(arena: &[[char; ARENA_WIDTH]; ARENA_HEIGHT]) -> Vec<Fighter> {
    println!("Placing fighters!");
    let mut rng = rand::rng();
    let mut fighters = Vec::new();
    let location_result = get_locations_from_db();
    match location_result {
        Ok(locations) => {
            for l in locations {
                // Generate random stats
                let health = (100.0 - l.flight_cost) as i32;
                let attack = rng.random_range(10..26);

                // Find empty position
                let mut position;
                loop {
                    position = (
                        rng.random_range(0..ARENA_WIDTH),
                        rng.random_range(0..ARENA_HEIGHT),
                    );

                    // Check if position is empty
                    if arena[position.1][position.0] == ' ' {
                        // Check if it's not occupied by any of our existing fighters
                        if !fighters.iter().any(|f: &Fighter| f.position == position) {
                            break;
                        }
                    }
                }

                fighters.push(Fighter::new(l.name, health, attack, position));
            }
            fighters
        }
        Err(_e) => {
            println!("ERROR: No locations returned from db.");
            fighters
        }
    }
}

fn count_alive_fighters(fighters: &[Fighter]) -> usize {
    fighters.iter().filter(|f| f.is_alive()).count()
}

fn get_winner(fighters: &[Fighter]) -> Option<&Fighter> {
    fighters.iter().find(|f| f.is_alive())
}

fn main() {
    println!("Arena Fighter Game");
    println!("Press Enter to start...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let _ = populate_db();
    let mut arena = create_arena();
    let mut fighters = place_fighters_initially(&arena);
    let mut combatlog = vec![];

    // Game loop
    loop {
        update_arena(&mut arena, &fighters);
        print_arena(&arena, &fighters);
        print_combatlog(&combatlog);

        // Check if game is over
        if count_alive_fighters(&fighters) <= 1 {
            if let Some(winner) = get_winner(&fighters) {
                println!(
                    "\nFighter {} is the champion with {} health remaining!",
                    winner.name, winner.health
                );
            } else {
                println!("\nAll fighters have been defeated! It's a draw!");
            }
            break;
        }

        // Move fighters
        for fighter in fighters.iter_mut().filter(|f| f.is_alive()) {
            fighter.move_randomly(&arena);
        }

        // Check for battles
        combatlog.extend(check_for_battles(&mut fighters)); //updates combatlog with new fights

        thread::sleep(Duration::from_millis(MOVE_DELAY_MS));
    }

    println!("\nGame over! Press Enter to exit...");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
}
