use rusqlite::{params, Connection, Result};

#[derive(Clone, Debug)]
pub struct Location {
    pub name: String,
    pub temp: f64,
    pub ticket_price: f64,
}

impl Location {
    pub fn new(name: String, temp: f64, ticket_price: f64) -> Self {
        Location {
            name,
            temp,
            ticket_price,
        }
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
            temp REAL NOT NULL,
            ticket_price REAL NOT NULL
        )",
        [],
    )?;

    // Insert example countries
    let countries = vec![
        ("Barcelona", 35.0, 447.89),
        ("Paris", 22.2, 95.84),
        ("London", 28.6, 447.65),
        ("Berlin", 22.9, 63.13),
        ("Lisbon", 30.4, 497.24),
        ("Rome", 23.4, 204.75),
        ("Madrid", 27.5, 54.82),
        ("Amsterdam", 32.2, 188.56),
        ("Vienna", 33.5, 205.33),
        ("Prague", 21.0, 156.45),
        ("Brussels", 18.9, 121.67),
        ("Zurich", 25.2, 215.43),
        ("Athens", 29.7, 135.89),
        ("Copenhagen", 19.4, 122.58),
        ("Oslo", 17.1, 232.10),
        ("Stockholm", 24.5, 170.85),
        ("Helsinki", 18.8, 194.50),
        ("Budapest", 26.3, 91.44),
        ("Warsaw", 27.9, 185.31),
        ("Belgrade", 34.0, 272.40),
        ("Zagreb", 19.1, 195.74),
        ("Tallinn", 15.7, 212.09),
        ("Moscow", 30.1, 315.02),
        ("Saint Petersburg", 31.3, 298.91),
        ("Stockholm", 19.4, 172.32),
        ("Reykjavik", 12.8, 389.62),
        ("Sarajevo", 22.6, 302.71),
        ("Bucharest", 33.8, 213.34),
        ("Warsaw", 29.1, 198.43),
        ("Belgrade", 28.3, 264.56),
        ("Skopje", 24.8, 204.19),
        ("Podgorica", 26.1, 230.22),
        ("Sarajevo", 22.0, 174.66),
        ("Zagreb", 26.7, 179.83),
        ("Ljubljana", 18.9, 167.74),
        ("Tirana", 25.4, 234.55),
        ("Riga", 22.0, 210.11),
        ("Vilnius", 19.5, 171.90),
        ("Chisinau", 29.0, 194.22),
        ("Bucharest", 25.2, 221.88),
        ("Tirana", 23.8, 149.33),
        ("Pristina", 30.5, 247.98),
        ("Helsinki", 17.0, 254.56),
        ("Vilnius", 28.2, 153.18),
        ("Bratislava", 22.1, 202.15),
        ("Chisinau", 31.4, 259.32),
        ("Madrid", 33.0, 81.71),
        ("Rome", 26.9, 211.98),
        ("Oslo", 14.9, 237.40),
        ("Bern", 27.1, 283.56),
        ("Luxembourg", 29.3, 212.75),
        ("Monaco", 34.4, 229.94),
        ("Andorra la Vella", 30.8, 239.25),
        ("Vaduz", 25.9, 261.32),
        ("San Marino", 22.5, 179.60),
        ("Malta", 28.7, 198.50),
        ("Naples", 27.4, 215.71),
        ("Florence", 23.1, 247.44),
        ("Milan", 29.2, 175.60),
        ("Turin", 19.8, 132.75),
        ("Amsterdam", 26.6, 168.25),
        ("Rotterdam", 24.4, 155.87),
        ("The Hague", 22.7, 174.62),
        ("Leiden", 25.5, 146.83),
        ("Gothenburg", 23.3, 230.05),
        ("Aarhus", 21.6, 156.60),
        ("Bergen", 19.3, 168.43),
        ("Sofia", 22.4, 189.58),
        ("Tbilisi", 30.2, 324.51),
        ("Almaty", 24.3, 276.02),
        ("Astana", 32.6, 318.78),
        ("Baku", 33.2, 301.34),
        ("Yerevan", 28.0, 215.60),
        ("Cairo", 38.1, 400.55),
        ("Beirut", 25.8, 180.44),
        ("Dubai", 35.7, 350.32),
        ("Abu Dhabi", 34.2, 367.15),
        ("Kuwait", 36.1, 388.00),
        ("Manama", 32.8, 329.21),
        ("Muscat", 34.5, 389.48),
        ("Doha", 33.5, 357.99),
        ("Istanbul", 30.7, 222.72),
        ("Jeddah", 31.4, 298.88),
        ("Mecca", 37.0, 360.61),
        ("Riyadh", 36.9, 410.78),
        ("Dammam", 32.9, 302.67),
        ("Amman", 28.1, 214.85),
        ("Damascus", 33.4, 275.90),
        ("Tehran", 27.8, 318.63),
        ("Baghdad", 35.9, 342.85),
        ("Kuwait City", 33.2, 392.57),
        ("Cairo", 38.2, 396.30),
        ("Tel Aviv", 32.3, 211.78),
        ("Rabat", 27.3, 192.60),
        ("Casablanca", 29.5, 208.73),
        ("Tangier", 26.7, 161.02),
        ("Fez", 31.2, 253.41),
        ("Marrakesh", 36.3, 289.54),
        ("Dakar", 32.0, 198.97),
        ("Lagos", 29.8, 217.62),
        ("Accra", 28.3, 229.44),
        ("Abuja", 30.2, 239.79),
        ("Cotonou", 29.7, 212.38),
        ("Lome", 27.0, 185.91),
        ("Monrovia", 28.8, 263.47),
        ("Freetown", 30.1, 274.53),
        ("Banjul", 26.4, 245.70),
        ("Conakry", 24.9, 271.38),
        ("Ouagadougou", 33.6, 326.52),
        ("Lima", 19.2, 232.10),
        ("Quito", 22.3, 157.34),
        ("Bogotá", 21.1, 243.99),
        ("Caracas", 26.5, 302.88),
        ("Montevideo", 24.8, 221.13),
        ("Buenos Aires", 30.0, 174.21),
        ("Santiago", 25.1, 196.77),
        ("Asunción", 27.4, 210.96),
        ("La Paz", 16.5, 197.43),
        ("Sucre", 18.0, 231.04),
        ("Brasília", 33.6, 289.35),
        ("São Paulo", 26.8, 182.90),
        ("Rio de Janeiro", 30.4, 258.31),
        ("Salvador", 32.7, 299.72),
        ("Fortaleza", 28.5, 271.49),
        ("Recife", 29.6, 255.37),
        ("Porto Alegre", 24.7, 211.54),
    ];

    for (name, temp, ticket_price) in countries {
        conn.execute(
            "INSERT OR IGNORE INTO countries (name, temp, ticket_price) VALUES (?1, ?2, ?3)",
            params![name, temp, ticket_price],
        )?;
    }

    println!("Migration and data insertion complete.");
    Ok(())
}

pub fn get_x_locations_from_db(x: u32) -> Result<Vec<Location>> {
    let conn = Connection::open("store.db")?;
    let mut locations: Vec<Location> = vec![];

    let mut stmt =
        conn.prepare("SELECT name, temp, ticket_price FROM countries ORDER BY RANDOM() LIMIT ?1")?;

    let db_vec = stmt
        .query_map(params![x], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, f64>(2)?,
            ))
        })?
        .collect::<Result<Vec<_>>>()?;
    for l in db_vec {
        locations.push(Location::new(l.0, l.1, l.2));
    }

    Ok(locations)
}
