use async_trait::*; // the official feature is not stable yet

// https://blog.theincredibleholk.org/blog/2022/04/18/how-async-functions-in-traits-could-work-in-rustc/

#[async_trait]
trait AsyncTrait {
    async fn get_string(&self) -> String;
}

#[async_trait]
impl AsyncTrait for i32 {
    async fn get_string(&self) -> String {
        self.to_string()
    }
}

pub fn async_trait_example() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let x = 10;
        let y = x.get_string().await;
        println!("y={}", y);
    });
}
