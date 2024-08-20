use libp2p::{
    core, core::muxing::StreamMuxerBox, core::transport::Boxed, identity::Keypair,  noise,
    yamux, PeerId, Transport,
};
use libp2p_mplex::Multiplex;
use std::time::Duration;

/// Builds the transport stack that LibP2P will communicate over.
pub fn build_transport(local_key: &Keypair) -> Boxed<(PeerId, StreamMuxerBox)> {
    let transport = libp2p::tcp::Config::new().nodelay(true);
    let transport = async_std::task::block_on(libp2p::dns::ResolverConfig::system(transport)).unwrap();
    let auth_config = {
        let dh_keys = Keypair::generate_ed25519();
        noise::Config::xx(dh_keys).into_authenticated()
    };

    let mplex_config = {
        let mut mplex_config = libp2p_mplex::MplexConfig::new();
        mplex_config.set_max_buffer_size(usize::MAX);

        let mut yamux_config = libp2p::yamux::Config::default();
        yamux_config.set_max_buffer_size(16 * 1024 * 1024);
        yamux_config.set_receive_window_size(16 * 1024 * 1024);
        // yamux_config.set_window_update_mode(WindowUpdateMode::OnRead);
        core::upgrade::SelectUpgrade::new(yamux_config, mplex_config)
    };

    transport
        .upgrade(core::upgrade::Version::V1)
        .authenticate(auth_config)
        .multiplex(mplex_config)
        .timeout(Duration::from_secs(20))
        .boxed()
}
