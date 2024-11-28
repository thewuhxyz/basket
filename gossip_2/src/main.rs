pub fn main() {

    // our config
    // 1. identity keypair: Default: Keypair::default()
    // 2. Cluster entrypoints
    // 3. Our socket addressess/ports: I think this should be automatic from the gossip
    // 4. Gossip validators
    //

    // step 1: node config, we use all our socket addresses here
    //
    // let node_config = NodeConfig {
    //         gossip_addr,
    //         port_range: dynamic_port_range,
    //         bind_ip_addr: bind_address,
    //         public_tpu_addr,
    //         public_tpu_forwards_addr,
    //         num_tvu_sockets: tvu_receive_threads,
    //         num_quic_endpoints,
    //     };

    // step 2 : convert the entrypoint to contact_info
    //
    // let cluster_entrypoints = entrypoint_addrs
    //         .iter()
    //         .map(ContactInfo::new_gossip_entry_point)
    //         .collect::<Vec<_>>();

    // step 3: create a node
    //
    // let mut node = Node::new_with_external_ip(&identity_keypair.pubkey(), node_config);

    // we don't want to be an entrypoint for other nodes
    //
    // node.info.remove_tpu();
    // node.info.remove_tpu_forwards();
    // node.info.remove_tvu();
    // node.info.remove_serve_repair();
    // node.sockets.ip_echo = None;

    // private rpc bind address: we don't want to public our rpc port for use
    // let rpc_bind_address = solana_net_utils::parse_host("127.0.0.1").unwrap()

    // let identity_keypair = Arc::new(identity_keypair);
}
