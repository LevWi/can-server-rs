use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{ self, JoinHandle};

pub struct Builder {

}

pub struct Server {
    work_flag : Arc<AtomicBool>,
    jobs: Vec<JoinHandle<()>>,
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

        Server { work_flag : flag, jobs : vec![listener, sender] }
    }
}

impl Server {
    pub fn stop(self) {
        self.work_flag.store(false, Ordering::Release);
        
        for j in self.jobs.into_iter() {
            j.join().unwrap();
        }

    }
}