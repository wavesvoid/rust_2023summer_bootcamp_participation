#![allow(dead_code)]
/// This is the only way to implement Sync, Send behaviour
/// without resorting to use of UNSAFE or NIGHTLY



use std::{
    marker::PhantomData,
    cell::RefCell,
    sync::{RwLock, MutexGuard},
    fmt::Debug,
};



#[derive(Debug)]
struct OnlySync<'a>(RwLock<i32>, PhantomData<MutexGuard<'a, i32>>);
impl OnlySync<'_> {
    fn new() -> Self { Self(RwLock::new(0), PhantomData) }
}


#[derive(Debug)]
struct OnlySend(RwLock<i32>, PhantomData<RefCell<i32>>);
impl OnlySend {
    fn new() -> Self { Self(RwLock::new(0), PhantomData) }
}


#[derive(Debug)]
struct SyncAndSend(RwLock<i32>, PhantomData<i32>);
impl SyncAndSend {
    fn new() -> Self { Self(RwLock::new(0), PhantomData) }
}


#[derive(Debug)]
struct NotSyncNotSend(RwLock<i32>, PhantomData<*mut i32>);
impl NotSyncNotSend {
    fn new() -> Self { Self(RwLock::new(0), PhantomData)}
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        thread,
    };


    #[test]
    // Please use:
    // `cargo test -p task_1_9 --lib -- --nocapture
    // To see output
    fn test_demonstration_markers() {
        // Only Sync
        let only_sync = OnlySync::new();
        let only_sync_ref = &only_sync;
        
        // Sync and Send
        let sync_send = SyncAndSend::new(); // to sync
        // Threads are "move", so we watch not to move original value but share it
        let sync_send_ref = &sync_send; 
                
        // Not Sync Not Send
        let _neither = NotSyncNotSend::new();
        let _not_sync_not_send_ref = &_neither;
        
        let max_threads = 100;

        thread::scope(|s| {
            // Demonstrate Sync + Send
            (1..max_threads).into_iter().for_each(|i| {
                let to_send_only = OnlySend::new();

                s.spawn(move || {
                    let onlysync = &only_sync_ref;
                    
                    // Test here is that it should not fall
                    // And that it can be successfully sent to the thread
                    let _syncsend = &sync_send_ref;
                    let _send_only = to_send_only.0.read().unwrap();


                    if let Ok(mut v) = onlysync.0.write() {
                        *v += i;
                    }

                    // Compile error: Cannot be shared safely
                    // 000000000000000000000000000000
                    // let os = &only_send_link;

                    // Compile error:
                    // - Cannot be shared
                    // - Cannot be sent
                    // 000000000000000000000000000000
                    // let os = _neither;
                    // let os = &_not_sync_not_send_ref;
                });
            });
        });

        let v = only_sync_ref.0.write().unwrap();

        assert_eq!(4950, *v);
    }
}