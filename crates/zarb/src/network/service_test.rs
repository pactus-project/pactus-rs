mod tests {
    use crate::network::{
        config::Config,
        service::ZarbNetwork,
        {NetworkEvent, NetworkMessage, NetworkService},
    };
    use crate::Service;
    use async_std::task;
    use simple_logger::SimpleLogger;
    use std::{thread, time::Duration};

    #[test]
    fn network_initialize() {
        let conf = Config::default();
        let net = ZarbNetwork::new(conf);
        assert!(net.is_ok(), "Network initialization failed");
    }

    #[async_std::test]
    async fn network_discovery() {
        SimpleLogger::new().with_utc_timestamps().init().unwrap();

        let conf1 = Config::default();
        let net1 = ZarbNetwork::new(conf1).unwrap();

        let mut conf2 = Config::default();
        conf2.listening_addr = format!(
            "/ip4/0.0.0.0/tcp/{}",
            portpicker::pick_unused_port().unwrap()
        )
        .parse()
        .unwrap();
        let net2 = ZarbNetwork::new(conf2).unwrap();

        let net1_sender = net1.message_sender();
        let net2_receiver = net2.event_receiver();

        task::spawn(async {
            net1.start().await;
        });

        task::spawn(async {
            net2.start().await;
        });

        let delay = Duration::from_millis(2000);
        thread::sleep(delay);

        let data1 = [1, 2, 3, 4].to_vec();
        let msg1 = NetworkMessage::GeneralMessage {
            data: data1.clone(),
        };

        net1_sender.send(msg1).await.unwrap();

        loop {
            let msg2 = net2_receiver.recv().await;
            match msg2.unwrap() {
                NetworkEvent::MessageReceived {
                    source: _,
                    data,
                } => {
                    assert_eq!(data, data1);
                    break;
                }
                _ => {}
            }
        }
    }
}
