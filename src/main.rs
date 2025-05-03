mod fight_db;

use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::{random_range, rng, seq::SliceRandom, Rng};
use std::io::{self, Write};
use std::{thread, time::Duration};

const ARENA_WIDTH: usize = 80;
const ARENA_HEIGHT: usize = 10;
const MOVE_DELAY_MS: u64 = 400; // How fast everything progresses
const FIGHTERS_COUNT: u32 = 10;

#[derive(Clone, Debug)]
struct Fighter {
    name: String,
    health: i32,
    attack: i32,
    ac: i32, // Armour class, how difficult they are to hit. 0-20, 0 will be hit all of the time, 20 will never be hit
    position: (usize, usize),
}

impl Fighter {
    fn new(name: String, health: i32, attack: i32, position: (usize, usize)) -> Self {
        Fighter {
            name,
            health,
            attack,
            ac: 8,
            position,
        }
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn move_randomly(&mut self, arena: &[[char; ARENA_WIDTH]; ARENA_HEIGHT]) {
        let mut rng = rand::rng();
        let dirs = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
            (0, 0),
        ];
        let dir = dirs[rng.random_range(0..dirs.len())];

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

    fn move_towards(
        &mut self,
        arena: &[[char; ARENA_WIDTH]; ARENA_HEIGHT],
        target: (usize, usize),
    ) {
        let mut new_x: usize = self.position.0;
        if self.position.0 < target.0 {
            new_x = (self.position.0 as i32 + 1) as usize
        } else if self.position.0 > target.0 {
            new_x = (self.position.0 as i32 - 1) as usize
        }

        let mut new_y: usize = self.position.1;
        if self.position.1 < target.1 {
            new_y = (self.position.1 as i32 + 1) as usize
        } else if self.position.1 > target.1 {
            new_y = (self.position.1 as i32 - 1) as usize
        }

        // Only move if the position is empty (contains a space)
        if arena[new_y][new_x] == ' ' {
            self.position = (new_x, new_y);
        }
    }

    fn increase_attack(&mut self, x: i32) {
        self.attack += x;
    }
    fn increase_health(&mut self, x: i32) {
        self.health += x;
    }
    fn increase_ac(&mut self, x: i32) {
        self.ac += x;
    }
    fn roll_20(&self) -> i32 {
        random_range(0..21)
    }
    fn find_opponent_position(&self, fighters: &[Fighter]) -> Option<(usize, usize)> {
        for fighter in fighters {
            if fighter.health > 0 && fighter.health < self.health {
                return Some(fighter.position);
            }
        }
        None
    }
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
        if !fighter.is_alive() {
            println!("{}  (DEFEATED)", fighter.name);
        } else {
            println!(
                "{}: Health: {}, Attack: {}",
                fighter.name, fighter.health, fighter.attack
            );
        }
    }
    io::stdout().flush().unwrap();
}

fn print_combatlog(log: &[String]) {
    for s in log.iter().rev().take(5).rev() {
        println!("{s}");
    }
}

fn check_for_battles(fighters: &mut [Fighter]) -> Vec<String> {
    // Check for battles
    let mut turn_combatlog: Vec<String> = vec![];
    for attacker_id in 0..fighters.len() {
        if !fighters[attacker_id].is_alive() {
            continue;
        }
        for defender_id in 0..fighters.len() {
            if !fighters[defender_id].is_alive()
                || fighters[attacker_id].name == fighters[defender_id].name
            {
                continue;
            }

            // If fighters are at the same position, they battle
            if fighters[attacker_id]
                .position
                .0
                .abs_diff(fighters[defender_id].position.0)
                <= 1
                && fighters[attacker_id]
                    .position
                    .1
                    .abs_diff(fighters[defender_id].position.1)
                    <= 1
            {
                // Attack
                if fighters[attacker_id].roll_20() > fighters[defender_id].ac {
                    fighters[defender_id].health -= fighters[attacker_id].attack;

                    let combatlog_entry: String = format!(
                        "{} attacked {} and dealt {} damage!",
                        fighters[attacker_id].name,
                        fighters[defender_id].name,
                        fighters[attacker_id].attack
                    );

                    turn_combatlog.push(combatlog_entry)
                } else {
                    let combatlog_entry: String = format!(
                        "{} attacked {} but missed!",
                        fighters[attacker_id].name, fighters[defender_id].name
                    );
                    turn_combatlog.push(combatlog_entry)
                }
            }
        }
    }
    turn_combatlog
}

fn place_fighters_initially(arena: &[[char; ARENA_WIDTH]; ARENA_HEIGHT]) -> Vec<Fighter> {
    println!("Placing fighters!");
    let mut rng = rand::rng();
    let mut fighters = Vec::new();
    let location_result = fight_db::get_x_locations_from_db(FIGHTERS_COUNT);
    match location_result {
        Ok(locations) => {
            for l in locations {
                // Generate stats
                let health = (1000.0 - (25.0 - l.temp).abs() * 100.0) as i32; // How far from 25 decreed C is the country
                let attack = ((1000.0 - l.ticket_price) / 10.0) as i32;

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
        Err(_e) => fighters,
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

    let _ = fight_db::populate_db();
    let mut arena = create_arena();
    let mut fighters = place_fighters_initially(&arena);
    let mut combatlog = vec![];
    update_arena(&mut arena, &fighters);
    print_arena(&arena, &fighters);

    // Buffs for the fighters pre-battle
    println!("Who do you bestow with the sword (+attack)?\n");
    let mut sword_choice = String::new();
    io::stdin()
        .read_line(&mut sword_choice)
        .expect("Failed to read line");
    println!("...and who do you bless with the shield (+health)?\n");
    let mut shield_choice = String::new();
    io::stdin()
        .read_line(&mut shield_choice)
        .expect("Failed to read line");
    for f in fighters.iter_mut() {
        if f.name.to_lowercase() == sword_choice.to_lowercase().trim() {
            f.increase_attack(30);
        }
        if f.name.to_lowercase() == shield_choice.to_lowercase().trim() {
            f.increase_health(100);
        }
    }
    update_arena(&mut arena, &fighters);
    print_arena(&arena, &fighters);

    println!("The fighters await your signal. Press ENTER to begin...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Game loop
    loop {
        fighters.shuffle(&mut rng());
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
        for i in 0..fighters.len() {
            if fighters[i].is_alive() {
                match fighters[i].find_opponent_position(&fighters) {
                    Some(opponent_position) => {
                        if random_range(0..11) < 6 {
                            // Its boring if they track their targets perfectly, so I use chance
                            fighters[i].move_towards(&arena, opponent_position)
                        } else {
                            fighters[i].move_randomly(&arena);
                        }
                    }
                    None => fighters[i].move_randomly(&arena),
                }
            }
        }

        // Check for battles
        combatlog.extend(check_for_battles(&mut fighters)); //updates combatlog with new fights

        thread::sleep(Duration::from_millis(MOVE_DELAY_MS));
    }

    println!("\nGame over! Press Enter to exit...");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
}
