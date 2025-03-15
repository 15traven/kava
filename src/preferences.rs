use redb::{Database, TableDefinition, Error as DBError};
 
const DB_NAME: &str = "preferences.redb";
const TABLE: TableDefinition<&str, bool> = TableDefinition::new("preferences");
 
pub const PREF_RUN_ACTIVATED: &str = "run_activated";
pub const PREF_TOGGLE_WITH_LEFT_CLICK: &str = "activate_on_left_click";

pub struct Preferences {
    db: Database
}
 
 impl Preferences {
    pub fn new() -> Result<Self, DBError> {
        let db = Database::create(DB_NAME)?;
 
        let txn = db.begin_write()?;
        {
            let _table = txn.open_table(TABLE)?;
        }
        txn.commit()?;
 
        Ok(Self { db })
    }
 
    pub fn init(&self) -> Result<(), DBError> {
        if !self.exists(PREF_RUN_ACTIVATED).unwrap() {
            self.save_preference(PREF_RUN_ACTIVATED, false)?;
        }

        if !self.exists(PREF_TOGGLE_WITH_LEFT_CLICK).unwrap() {
            self.save_preference(PREF_TOGGLE_WITH_LEFT_CLICK, true)?;
        }

        Ok(())
    }
 
    pub fn save_preference(&self, key: &str, value: bool) -> Result<(), DBError> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(TABLE)?;
            table.insert(key, value)?;
        }
        txn.commit()?;

        Ok(())
    }

    pub fn load_preference(&self, key: &str) -> Result<bool, DBError> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(TABLE)?;
        let res = table.get(key)?.unwrap();

        Ok(res.value())
    }

    pub fn toggle_preference(&self, key: &str) -> Result<(), DBError> {
        if let Ok(value) = self.load_preference(key) {
            match value {
                true => {
                    self.save_preference(key, false)?;
                }
                false => {
                    self.save_preference(key, true)?;
                }
            }
        }

        Ok(())
    }

    fn exists(&self, key: &str) -> Result<bool, DBError> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(TABLE)?;
        let res = table.get(key)?;

        Ok(res.is_some())
    }
}