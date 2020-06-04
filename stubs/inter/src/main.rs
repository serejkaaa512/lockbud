use std::sync;
use parking_lot;

struct Foo {
    mu1: sync::Arc<sync::Mutex<i32>>,
    rw1: sync::RwLock<i32>,
    mu2: parking_lot::Mutex<i32>,
    rw2: parking_lot::RwLock<i32>,
}

impl Foo {
    fn new() -> Self {
        Self {
            mu1: sync::Arc::new(sync::Mutex::new(1)),
            rw1: sync::RwLock::new(1),
            mu2: parking_lot::Mutex::new(1),
            rw2: parking_lot::RwLock::new(1),
        }
    }

    fn std_mutex_1(&self) {
        let guard1 = self.mu1.lock().unwrap();
        let guard2 = guard1;
        match *guard2 {
            1 => {},
            _ => { self.std_mutex_2(); },
        };
    }

    fn std_mutex_2(&self) {
        *self.mu1.lock().unwrap() += 1;
    }

    fn std_rwlock_read_1(&self) {
        match *self.rw1.read().unwrap() {
            1 => { self.std_rwlock_write_2(); },
            _ => { self.std_rwlock_read_2(); },
        };
    }

    fn std_rwlock_write_1(&self) {
        match *self.rw1.write().unwrap() {
            1 => { self.std_rwlock_write_2(); },
            _ => { self.std_rwlock_read_2(); },
        };
    }

    fn std_rwlock_read_2(&self) {
        *self.rw1.read().unwrap();
    }

    fn std_rwlock_write_2(&self) {
        *self.rw1.write().unwrap() += 1;
    }

    fn parking_lot_mutex_1(&self) {
        match *self.mu2.lock() {
            1 => {},
            _ => { self.parking_lot_mutex_2(); },
        };
    }

    fn parking_lot_mutex_2(&self) {
        *self.mu2.lock() += 1;
    }

    fn parking_lot_rwlock_read_1(&self) {
        match *self.rw2.read() {
            1 => { self.parking_lot_rwlock_write_2(); },
            _ => { self.parking_lot_rwlock_read_2(); },
        };
    }

    fn parking_lot_rwlock_write_1(&self) {
        match *self.rw2.write() {
            1 => { self.parking_lot_rwlock_write_2(); },
            _ => { self.parking_lot_rwlock_read_2(); },
        };
    }

    fn parking_lot_rwlock_read_2(&self) {
        *self.rw2.read();
    }

    fn parking_lot_rwlock_write_2(&self) {
        *self.rw2.write() += 1;
    }
}

fn main() {
    
}
