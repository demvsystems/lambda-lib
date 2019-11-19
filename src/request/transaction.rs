use lazy_static::lazy_static;
use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref TRANSACTION_ID: Mutex<Option<String>> = Mutex::new(None);
}

pub fn write_id(transaction_id: &str) {
    let mut tid: MutexGuard<Option<String>> = TRANSACTION_ID.lock().unwrap();
    *tid = Some(transaction_id.to_string());
}

pub fn read_id() -> String {
    let tid: MutexGuard<Option<String>> = TRANSACTION_ID.lock().unwrap();
    match *tid {
        Some(ref tid) => tid.clone(),
        None => "Unbekannt".to_string(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn log_transaction_id() {
        assert_eq!("Unbekannt", super::read_id().as_str());
        super::write_id("Foo");
        assert_eq!("Foo", super::read_id().as_str());
    }
}
