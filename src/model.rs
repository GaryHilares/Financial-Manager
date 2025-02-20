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

pub struct RecordCollection {
    records: Vec<Record>,
}

impl RecordCollection {
    pub fn new() -> RecordCollection {
        RecordCollection {
            records: Vec::<Record>::new(),
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
        self.records.push(new_record);
    }

    pub fn list_records(&self) -> &Vec<Record> {
        &self.records
    }
}

#[cfg(test)]
mod record_collection_tests {
    use super::{InflightRecord, RecordCollection};

    #[test]
    pub fn adds_record_with_right_balance() {
        let mut record_collection = RecordCollection::new();
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
}
