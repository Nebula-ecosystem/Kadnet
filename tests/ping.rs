use cadentis::join;
use cadentis::time::sleep;
use kadnet::node::NodeBuilder;

use std::time::Duration;

#[cadentis::test]
async fn ping_test() {
    let mut n1 = NodeBuilder::new(5734).build().unwrap();
    let mut n2 = NodeBuilder::new(5735).build().unwrap();

    let t1 = async move {
        let _ = n1.start().await;
    };
    let t2 = async move {
        sleep(Duration::from_secs(1)).await;
        let _ = n2.start().await;
    };

    let _ = join!(t1, t2);
}
