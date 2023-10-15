
use monoio::fs::File;


pub fn monoio_example() -> Result<(), Box<dyn std::error::Error>>{
    monoio::start::<monoio::LegacyDriver, _>(async {
        println!("monoio_example: Hello world!");

        // Open a file
        let file = File::open("LICENSE").await?;

        let buf = vec![0; 4096];
        // Read some data, the buffer is passed by ownership and
        // submitted to the kernel. When the operation completes,
        // we get the buffer back.
        let (res, buf) = file.read_at(buf, 0).await;
        let n = res?;

        // Display the contents
        println!("monoio_example: {:?}", &buf[..n]);

        Ok(())
    })
}
  
