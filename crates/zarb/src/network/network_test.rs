mod tests {
    use super::super::*;
    use async_std::task;
    use std::{thread, time::Duration};
    //use simple_logger::SimpleLogger;

    fn initialize(node_id: NodeId) -> Result<Network, Error> {
        let conf = Config::default();
        Network::new(conf, node_id)
    }

    #[test]
    fn network_initialize() {
        let net = self::initialize(0);
        assert!(net.is_ok(), "Network initialization failed");
    }

    #[tokio::test(threaded_scheduler)]
    async fn network_discovery() {
        //SimpleLogger::new().init();
        let net1 = self::initialize(0).unwrap();
        let mut net2 = self::initialize(1).unwrap();

        let net1_sender = net1.sender();
        let net2_receiver = net2.register_topic("test".to_owned());

        task::spawn(async {
            net1.run().await;
        });

        task::spawn(async {
            net2.run().await;
        });

        let delay = Duration::from_millis(100);
        thread::sleep(delay);

        let p: [u8; 4] = [1, 2, 3, 4];
        let entry1 = Message::new("test".to_owned(), 0, p.to_vec());
        net1_sender.send(entry1.clone()).await;

        let msg2 = net2_receiver.recv().await;
        assert!(msg2.is_ok(), "Receiver failed");
        assert_eq!(entry1.message, msg2.unwrap().message);
    }
}
