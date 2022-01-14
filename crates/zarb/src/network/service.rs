use super::{behaviour::ZarbBehaviour, config::Config, Network};
use crate::error::Result;
use async_trait::async_trait;
use futures::StreamExt;
use libp2p::{
    identity,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};

pub struct Service {
    config: Config,
    swarm: Swarm<ZarbBehaviour>,
}

#[async_trait]
impl Network for Service {
    fn start(&self) -> Result<()> {
        Ok(())
    }

    fn stop(&self) {}
    fn close_connection(&self, pid: PeerId) {}
    fn self_id(&self) -> PeerId {
        todo!()
    }
}

impl Service {
    pub fn new(config: Config) -> Result<Self> {
        // Create a random PeerId.
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());
        println!("Local peer id: {:?}", peer_id);

        // Create a transport.
        let transport = super::transport::build_transport(keypair.clone());

        // Create a network behaviour.
        let behaviour = super::behaviour::ZarbBehaviour::new(&keypair, &config);

        // Create a Swarm that establishes connections through the given transport.
        // Note that the MDNS behaviour itself will not actually inititiate any connections,
        // as it only uses UDP.
        let mut swarm = Swarm::new(transport, behaviour, peer_id);
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        Ok(Service { config, swarm })
    }
}
