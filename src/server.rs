use std::sync::{ Arc, Weak };
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{ self, JoinHandle};
use std::collections::{ HashMap, HashSet };
use std::time::{ Duration, Instant };

type CanId = u8;

// Can receivers combine its ???
enum SubscribeType {
    ByChange,
    WaitAfterNew(Duration),
    IfLost(Duration),
    Periodic(Duration)
}

pub struct CanCache(HashMap<CanId, (Instant , [u8; 8])>);

//pub struct Subscribers(HashMap<CanId, HashMap<u32, Box<dyn FnMut(CanId, [u8])>>>);

pub struct Subscribers(Vec<Box<dyn FnMut(CanId, [u8])>>);

pub struct Server {
    work_flag : Arc<AtomicBool>,
    subscribers : Arc<Subscribers>,
    cache : CanCache,
    jobs: ( JoinHandle<()>, JoinHandle<()>),
}

impl Server {
    pub fn stop(self) {
        self.work_flag.store(false, Ordering::Release);
        self.jobs.0.join().unwrap();
        self.jobs.1.join().unwrap();
    }

    pub fn subscribe(&mut self, can_id : CanId, sub_type : SubscribeType ) {
        // TODO
    }
}


#[test]
fn check_subscriptions() {
    use std::sync::mpsc::channel;

    let mut t = 5;
    // This send is always successful
    {
        let (tx, rx) = channel::<Box<dyn FnMut(u8)>>();
        tx.send(Box::new( |x| { t += 1; } )).unwrap();
    }
}

pub struct Builder {
    //TODO
}

impl Builder {

    pub fn start(self) -> Server {

        let flag = Arc::new(AtomicBool::new(true));
        let work_flg = flag.clone();

        let listener = thread::Builder::new()
                .name("listener".into())
                .spawn( move || { 
           
            while work_flg.load(Ordering::Acquire) {
                //TODO
            } 

        } ).unwrap();

        let work_flg = flag.clone();
        let sender = thread::Builder::new()
                .name("sender".into())
                .spawn( move || { 
           
            while work_flg.load(Ordering::Acquire) {
                //TODO
            } 

        } ).unwrap();

        Server { work_flag : flag, jobs : (listener, sender) }
    }
}