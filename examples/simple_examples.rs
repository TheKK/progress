extern crate progress;

use std::thread;

fn main() {
    let mut pbar = progress::Bar::new();

    // Add percentage each time
    {
        pbar.set_job_title("This...");
        pbar.add_percent(20);
        thread::sleep_ms(1000);

        pbar.set_job_title("is...");
        pbar.add_percent(20);
        thread::sleep_ms(1000);

        pbar.set_job_title("very...");
        pbar.add_percent(20);
        thread::sleep_ms(1000);

        pbar.set_job_title("slow...");
        pbar.add_percent(20);
        thread::sleep_ms(1000);
        
        pbar.set_job_title("job...");
        pbar.add_percent(20);
        thread::sleep_ms(1000);
    }
    pbar.jobs_done();

    // Or you can directly specify where to go
    pbar.set_job_title(&"Creating kitties...".to_string());
    for i in 0..101 { 
        thread::sleep_ms(30);
        pbar.reach_percent(i);
    }
    pbar.jobs_done();

    // It's okay to break the limit!!
    pbar.set_job_title(&"Creating rainbow kitties...".to_string());
    for i in 0..501 { 
        thread::sleep_ms(5);
        pbar.reach_percent(i);
    }
    pbar.jobs_done();

    println!("Now the world is filled with rainbow kitties!");
}
