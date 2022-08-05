use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::thread::{JoinHandle};
use chrono::{DateTime, Utc};

pub struct Racer {
    thread_handlers: Vec<JoinHandle<()>>,
    goal: u64,
    winners: Arc<Mutex<Vec<(u8, DateTime<Utc>)>>>,
}

impl Racer {
    pub fn new<>(goal: u64, thread_amount: u8) -> Racer {
        let mut v = Vec::with_capacity(thread_amount as usize);
        let barrier = Arc::new(Barrier::new(thread_amount as usize));
        let mut f = Arc::new(Mutex::new(Vec::with_capacity(thread_amount as usize)));
        for i in 0..thread_amount {
            let c = Arc::clone(&barrier);
            let h = Arc::clone(&f);
            let handler = thread::Builder::new().name(format!("{} thread", i)).spawn(move || {
                println!("thread with name: {} ready for race at {:?}", thread::current().name().unwrap(), Utc::now());
                c.wait();
                for i in 0..goal {
                    println!("thread with name:{} says: {}", thread::current().name().unwrap(), i);
                }
                let (m, k) = (i, Utc::now());
                println!("thread with name: {} finished race at {:?}", thread::current().name().unwrap(), k);
                h.lock().unwrap().push((m, k));
                println!("thread with name: {} finished executing", thread::current().name().unwrap())
            }).unwrap();
            v.push(handler);
        }
        Racer {
            thread_handlers: v,
            goal,
            winners: f,
        }
    }
}


fn main() {
    let r = Racer::new(3, 3);
    for x in r.thread_handlers {
        x.join().expect("упс, потік схоже всьо");
    }

    let p = r.winners.lock().unwrap();
    let m: Vec<&DateTime<Utc>> = p.iter().map(|(_, t)| t).collect();
    println!("{:?}", m.windows(2).all(|o| o[0] < o[1]));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        //я типу не знаю що варто перевірити, тож перевірю чи кожен наступний вигравший по часу закінчив пізніше минулого(не знайшов як читати з стандартного виходу,
        // хотів перевірити чи дійсно всі дійшли до кінця....)
        let r = Racer::new(2, 2);
        for x in r.thread_handlers {
            x.join().expect("упс, потік схоже всьо");
        }
        let p = r.winners.lock().unwrap();
        let m: Vec<&DateTime<Utc>> = p.iter().map(|(_, t)| t).collect();
        assert!(m.windows(2).all(|o| o[0] < o[1]), "some of winners not on his place");
    }
}