mod pool;

use std::thread::sleep;

fn main() {
    let mut pool = pool::Pool::new(4);
    pool.add(|| {
        sleep(std::time::Duration::from_secs(2));
        println!("Hello from job 1!");
    });
    pool.add(|| {
        sleep(std::time::Duration::from_secs(3));
        println!("Hello from job 2!");
    });
    pool.add(|| {
        sleep(std::time::Duration::from_secs(1));
        println!("Hello from job 3!");
    });
    pool.add(|| {
        sleep(std::time::Duration::from_secs(7));
        println!("Hello from job 4!");
    });
    pool.add(|| {
        sleep(std::time::Duration::from_secs(4));
        println!("Hello from job 5!");
    });
    pool.add(|| {
        sleep(std::time::Duration::from_secs(2));
        println!("Hello from job 6!");
    });

    // 3 1 2 6 5 4
    pool.start();
}
