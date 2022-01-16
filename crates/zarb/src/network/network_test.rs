mod tests {
    use super::super::*;
    use async_std::task;
    use libp2p::PeerId;
    use std::{thread, time::Duration};
    //use simple_logger::SimpleLogger;

    fn initialize() -> Result<Network, Error> {
        let mut conf = Config::default();
        conf.listening_multiaddr = format!(
            "/ip4/0.0.0.0/tcp/{}",
            portpicker::pick_unused_port().unwrap()
        )
        .parse()
        .unwrap();

        Network::new(conf)
    }

    #[test]
    fn network_initialize() {
        let net = self::initialize();
        assert!(net.is_ok(), "Network initialization failed");
    }

    //#[async_std::test]
    async fn network_discovery() {
        //SimpleLogger::new().init();
        let net1 = self::initialize().unwrap();
        let mut net2 = self::initialize().unwrap();

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
        let entry1 = Message::new(PeerId::random(), p.to_vec());
        net1_sender.send(entry1.clone()).await.unwrap();

        let msg2 = net2_receiver.recv().await;
        println!("{:?}", msg2);
        assert!(msg2.is_ok(), "Receiver failed");
        assert_eq!(entry1.message, msg2.unwrap().message);
    }
}
