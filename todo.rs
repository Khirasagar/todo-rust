use rusqlite::{Connection, Result};

struct Person {
    id: i32,
    name: String,
    age: i32,
}

impl Person {
    fn new(name: &str, age: i32) -> Person {
        Person { id: -1, name: name.to_string(), age }
    }
}

fn create_person(conn: &Connection, person: &Person) -> Result<()> {
    conn.execute(
        "INSERT INTO person (name, age) VALUES (?1, ?2)",
        &[&person.name, &person.age],
    )?;
    Ok(())
}

fn read_person(conn: &Connection, id: i32) -> Result<Option<Person>> {
    let mut stmt = conn.prepare("SELECT id, name, age FROM person WHERE id = ?1")?;
    let person_iter = stmt.query_map(&[&id], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;
    for person_result in person_iter {
        if let Ok(person) = person_result {
            return Ok(Some(person));
        }
    }
    Ok(None)
}

fn update_person(conn: &Connection, id: i32, name: &str, age: i32) -> Result<()> {
    conn.execute(
        "UPDATE person SET name = ?1, age = ?2 WHERE id = ?3",
        &[&name, &age, &id],
    )?;
    Ok(())
}

fn delete_person(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM person WHERE id = ?1", &[&id])?;
    Ok(())
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER NOT NULL
                  )",
        [],
    )?;

    let person = Person::new("Alice", 30);
    create_person(&conn, &person)?;

    let retrieved_person = read_person(&conn, 1)?;
    println!("Retrieved Person: {:?}", retrieved_person);

    update_person(&conn, 1, "Alice Updated", 35)?;

    let updated_person = read_person(&conn, 1)?;
    println!("Updated Person: {:?}", updated_person);

    delete_person(&conn, 1)?;

    let deleted_person = read_person(&conn, 1)?;
    println!("Deleted Person: {:?}", deleted_person);

    Ok(())
}
