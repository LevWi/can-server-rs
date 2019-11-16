use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{ self, JoinHandle};

pub struct Server {
    work_flag : Arc<AtomicBool>,
    sender: Option<JoinHandle<()>>,
    listener: Option<JoinHandle<()>>
}

impl Server {
    pub fn new() -> Server {
        Server { work_flag : Arc::new(AtomicBool::new(false)), sender : None, listener : None }
    }

    pub fn start(&mut self) {

        let work_flg = self.work_flag.clone();
        self.listener = Some( thread::Builder::new()
                .name("listener".into())
                .spawn( move || { 
           
            while work_flg.load(Ordering::Acquire) {
                // TODO
            } 

        } ).unwrap()); 


        let work_flg = self.work_flag.clone();
        self.sender = Some( thread::Builder::new()
                .name("sender".into())
                .spawn( move || { 
           
            while work_flg.load(Ordering::Acquire) {
                // TODO
            } 

        } ).unwrap());
    }

    pub fn stop(&mut self) {
        self.work_flag.store(false, Ordering::Release);
        
        if self.sender.is_some() {
            //error self.sender.unwrap().join();
        }

    }
}

impl Drop for Server {
    fn drop(&mut self) {
        //TODO
    }
}