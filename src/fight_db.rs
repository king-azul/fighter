use rusqlite::{params, Connection, Result};

#[derive(Clone, Debug)]
pub struct Location {
    pub name: String,
    pub flight_cost: f64,
}

impl Location {
    pub fn new(name: String, flight_cost: f64) -> Self {
        Location { name, flight_cost }
    }
}

pub fn populate_db() -> Result<()> {
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

pub fn get_x_locations_from_db(x: u32) -> Result<Vec<Location>> {
    let conn = Connection::open("store.db")?;
    let mut locations: Vec<Location> = vec![];

    let mut stmt =
        conn.prepare("SELECT name, ticket_price FROM countries ORDER BY RANDOM() LIMIT ?1")?;

    let db_vec = stmt
        .query_map(params![x], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })?
        .collect::<Result<Vec<_>>>()?;
    for l in db_vec {
        locations.push(Location::new(l.0, l.1));
    }

    Ok(locations)
}
