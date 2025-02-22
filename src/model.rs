use rusqlite::Connection;

pub fn cents_to_dollar_string(cents: i32) -> String {
    let decimals = cents % 100;
    let units = cents / 100;
    return format!("{}.{:0>2}", units, decimals);
}

#[cfg(test)]
mod cents_to_dollar_string_tests {
    use super::cents_to_dollar_string;

    #[test]
    fn should_add_decimal_cents() {
        assert_eq!("4.07", cents_to_dollar_string(407));
    }

    #[test]
    fn should_pad_with_zeroes() {
        assert_eq!("0.00", cents_to_dollar_string(0));
    }
}

pub struct InflightRecord {
    pub date: String,
    pub description: String,
    pub earnings_in_cents: i32,
    pub spendings_in_cents: i32,
}

impl InflightRecord {
    pub fn new(
        date: String,
        description: String,
        earnings_in_cents: i32,
        spendings_in_cents: i32,
    ) -> Self {
        InflightRecord {
            date,
            description,
            earnings_in_cents,
            spendings_in_cents,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Record {
    pub date: String,
    pub description: String,
    pub earnings_in_cents: i32,
    pub spendings_in_cents: i32,
    pub initial_balance_in_cents: i32,
}

impl Record {
    pub fn new(
        date: String,
        description: String,
        earnings_in_cents: i32,
        spendings_in_cents: i32,
        initial_balance_in_cents: i32,
    ) -> Self {
        Record {
            date,
            description,
            earnings_in_cents,
            spendings_in_cents,
            initial_balance_in_cents,
        }
    }

    pub fn get_remaining_balance(&self) -> i32 {
        return self.initial_balance_in_cents + self.earnings_in_cents - self.spendings_in_cents;
    }
}

#[cfg(test)]
mod record_tests {
    use super::Record;

    #[test]
    fn should_increase_balance_on_earnings() {
        let record = Record::new(
            "2025-02-20".to_owned(),
            "Some earnings".to_owned(),
            10,
            0,
            2,
        );
        assert!(record.get_remaining_balance() == 12);
    }

    #[test]
    fn should_decrease_balance_on_spendings() {
        let record = Record::new(
            "2025-02-20".to_owned(),
            "Some earnings".to_owned(),
            0,
            10,
            8,
        );
        assert!(record.get_remaining_balance() == -2);
    }

    #[test]
    fn should_change_balance_by_difference_on_earnings_and_spendings() {
        let record = Record::new(
            "2025-02-20".to_owned(),
            "Some earnings".to_owned(),
            10,
            6,
            4,
        );
        assert!(record.get_remaining_balance() == 8);
    }

    #[test]
    fn should_not_change_balance_with_equal_earnings_and_spendings() {
        let record = Record::new("2025-02-20".to_owned(), "Some earnings".to_owned(), 7, 7, 2);
        assert_eq!(2, record.get_remaining_balance());
    }
}

pub trait DatabaseHandler {
    fn create_record(&mut self, record: &Record) -> ();
    fn read_records(&self) -> Vec<Record>;
}

pub struct SqliteDatabaseConnection {
    connection: Connection,
}

impl SqliteDatabaseConnection {
    pub fn create_or_open(path: &str) -> SqliteDatabaseConnection {
        let connection = Connection::open(path).expect("Could not open SQLite connection.");
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS financial_records (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    date TEXT,
                    description TEXT,
                    earnings_in_cents INTEGER,
                    spendings_in_cents INTEGER,
                    initial_balance_in_cents INTEGER
                )",
                (),
            )
            .expect("Could not create table.");
        SqliteDatabaseConnection { connection }
    }
}

impl DatabaseHandler for SqliteDatabaseConnection {
    fn create_record(&mut self, record: &Record) -> () {
        self.connection
            .execute(
                "INSERT INTO financial_records (
                date,
                description,
                earnings_in_cents,
                spendings_in_cents,
                initial_balance_in_cents
            ) VALUES (
                ?1,
                ?2,
                ?3,
                ?4,
                ?5
            )",
                (
                    &record.date,
                    &record.description,
                    &record.earnings_in_cents,
                    &record.spendings_in_cents,
                    &record.initial_balance_in_cents,
                ),
            )
            .expect("Could not insert values into database");
    }

    fn read_records(&self) -> Vec<Record> {
        let mut query = self
            .connection
            .prepare(
                "SELECT
                    date,
                    description,
                    earnings_in_cents,
                    spendings_in_cents,
                    initial_balance_in_cents
                FROM
                    financial_records
                ORDER BY
                    id;",
            )
            .unwrap();
        let result = query.query(()).unwrap();
        result
            .mapped(|row| {
                Ok(Record {
                    date: row.get(0)?,
                    description: row.get(1)?,
                    earnings_in_cents: row.get(2)?,
                    spendings_in_cents: row.get(3)?,
                    initial_balance_in_cents: row.get(4)?,
                })
            })
            .collect::<Vec<Result<Record, _>>>()
            .into_iter()
            .map(|res| res.unwrap())
            .collect::<Vec<Record>>()
    }
}

pub struct RecordCollection<T: DatabaseHandler> {
    records: Vec<Record>,
    db_handler: T,
}

impl<T: DatabaseHandler> RecordCollection<T> {
    pub fn new(db_handler: T) -> RecordCollection<T> {
        RecordCollection {
            records: db_handler.read_records(),
            db_handler,
        }
    }

    pub fn add_record(&mut self, new_record: InflightRecord) -> () {
        let balance = match self.records.last() {
            None => 0,
            Some(record) => record.get_remaining_balance(),
        };
        let new_record = Record::new(
            new_record.date,
            new_record.description,
            new_record.earnings_in_cents,
            new_record.spendings_in_cents,
            balance,
        );
        self.db_handler.create_record(&new_record);
        self.records.push(new_record);
    }

    pub fn list_records(&self) -> &Vec<Record> {
        &self.records
    }
}

#[cfg(test)]
mod record_collection_tests {
    use super::{DatabaseHandler, InflightRecord, Record, RecordCollection};

    struct FakeDatabaseHandler {
        records: Vec<Record>,
    }

    impl FakeDatabaseHandler {
        pub fn new() -> FakeDatabaseHandler {
            FakeDatabaseHandler {
                records: Vec::<Record>::new(),
            }
        }
    }

    impl DatabaseHandler for FakeDatabaseHandler {
        fn create_record(&mut self, record: &Record) -> () {
            self.records.push(record.clone());
        }

        fn read_records(&self) -> Vec<Record> {
            self.records.to_owned()
        }
    }

    #[test]
    pub fn adds_record_with_right_balance() {
        let db_handler = FakeDatabaseHandler::new();
        let mut record_collection = RecordCollection::new(db_handler);
        let new_record = InflightRecord::new(
            "2025-02-15".to_owned(),
            "Some money earned".to_owned(),
            10,
            2,
        );
        record_collection.add_record(new_record);
        let records = record_collection.list_records();
        assert_eq!(1usize, records.len());
        let last_record = records.last().unwrap();
        assert_eq!("2025-02-15", last_record.date);
        assert_eq!("Some money earned", last_record.description);
        assert_eq!(10, last_record.earnings_in_cents);
        assert_eq!(2, last_record.spendings_in_cents);
        assert_eq!(8, last_record.get_remaining_balance());
    }

    #[test]
    pub fn persists_records() {
        let db_handler = FakeDatabaseHandler::new();
        let mut record_collection = RecordCollection::new(db_handler);
        let new_record =
            InflightRecord::new("2025-02-22".to_owned(), "Money spent".to_owned(), 0, 10);
        let expected_record =
            Record::new("2025-02-22".to_owned(), "Money spent".to_owned(), 0, 10, 0);
        record_collection.add_record(new_record);
        let db_handler = record_collection.db_handler;
        let record_collection = RecordCollection::new(db_handler);
        let records = record_collection.list_records();
        assert_eq!(1, records.len());
        assert_eq!(expected_record, records[0]);
    }
}

mod persistence_integration_tests {
    use super::{InflightRecord, Record, RecordCollection, SqliteDatabaseConnection};

    #[test]
    pub fn sqlite_persists_records() {
        let db_handler =
            SqliteDatabaseConnection::create_or_open("./data/financial_records_tests.db");
        let mut record_collection = RecordCollection::new(db_handler);
        let new_record =
            InflightRecord::new("2025-02-22".to_owned(), "Money spent".to_owned(), 0, 10);
        let expected_record =
            Record::new("2025-02-22".to_owned(), "Money spent".to_owned(), 0, 10, 0);
        record_collection.add_record(new_record);
        let db_handler = record_collection.db_handler;
        let record_collection = RecordCollection::new(db_handler);
        let records = record_collection.list_records();
        assert_eq!(1, records.len());
        assert_eq!(expected_record, records[0]);
    }
}
