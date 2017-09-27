extern crate gin;

use gin::futures::prelude::*;
use gin::futures_cpupool::CpuPool;

struct Hello<'s>(u64, &'s str);

impl<'s> Future for Hello<'s> {
    type Item = u64;
    type Error = String;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use std::thread::sleep;
        use std::time::Duration;
        sleep(Duration::from_secs(self.0));
        println!("{}", self.1);
        Ok(Async::Ready(self.0))
    }
}

fn main() {
    let pool = CpuPool::new(8);
    let hello = Hello(3, "first")
        .join(Hello(1, "second"))
        .join(Hello(2, "third"))
        .map(|((a, b), c)| a + b + c);
    if let Ok(_res) = pool.spawn(hello).wait() {
        println!("res: {}", _res)
    }
}