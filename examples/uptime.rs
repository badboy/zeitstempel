use std::{thread, time::Duration};
fn main() {
    let start_inc = zeitstempel::Instant::now_including_suspend();
    let start_exc = zeitstempel::Instant::now_excluding_suspend();
    println!("Now with suspend: {}", start_inc.as_timestamp());
    println!("Now w/o  suspend: {}", start_exc.as_timestamp());

    thread::sleep(Duration::from_secs(2));

    println!("Diff with suspend: {} ms", start_inc.elapsed().as_millis());
    println!("Diff w/o  suspend: {} ms", start_exc.elapsed().as_millis());
}
