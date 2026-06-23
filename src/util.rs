//use std::{thread, time};
//use clearscreen;

////Sleep


pub mod utility {
    use std::{thread, time};
    pub fn sec_sleep(seconds: f64) {
        const THOUSAND_MS :f64 = 1000.0;
        let final_time :u64 = (seconds * THOUSAND_MS).round() as u64;

        
        let interval = time::Duration::from_millis(final_time);
        let now = time::Instant::now();
        println!("Begin wait");    
        thread::sleep(interval);

        assert!(now.elapsed() >= interval);

        println!("End wait");
    }
    
    pub fn clear_screen() {clearscreen::clear().expect("Failed to clear screen");}

    pub fn greet() {println!("Hello");}
}
