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

// fn start_gossip_node(
//     identity_keypair: Arc<Keypair>,
//     cluster_entrypoints: &[ContactInfo],
//     ledger_path: &Path,
//     gossip_addr: &SocketAddr,
//     gossip_socket: UdpSocket,
//     expected_shred_version: Option<u16>,
//     gossip_validators: Option<HashSet<Pubkey>>,
//     should_check_duplicate_instance: bool,
//     socket_addr_space: SocketAddrSpace,
// ) -> (Arc<ClusterInfo>, Arc<AtomicBool>, GossipService) {
//     let contact_info = ClusterInfo::gossip_contact_info(
//         identity_keypair.pubkey(),
//         *gossip_addr,
//         expected_shred_version.unwrap_or(0),
//     );
//     let mut cluster_info = ClusterInfo::new(contact_info, identity_keypair, socket_addr_space);
//     cluster_info.set_entrypoints(cluster_entrypoints.to_vec());
//     let cluster_info = Arc::new(cluster_info);

//     let gossip_exit_flag = Arc::new(AtomicBool::new(false));
//     let gossip_service = GossipService::new(
//         &cluster_info,
//         None,
//         gossip_socket,
//         gossip_validators,
//         should_check_duplicate_instance,
//         None,
//         gossip_exit_flag.clone(),
//     );
//     (cluster_info, gossip_exit_flag, gossip_service)
// }

// pub struct GossipService {
//     thread_hdls: Vec<JoinHandle<()>>,
// }

// impl GossipService {
//     pub fn new(
//         cluster_info: &Arc<ClusterInfo>,
//         bank_forks: Option<Arc<RwLock<BankForks>>>,
//         gossip_socket: UdpSocket,
//         gossip_validators: Option<HashSet<Pubkey>>,
//         should_check_duplicate_instance: bool,
//         stats_reporter_sender: Option<Sender<Box<dyn FnOnce() + Send>>>,
//         exit: Arc<AtomicBool>,
//     ) -> Self {
//         let (request_sender, request_receiver) = unbounded();
//         let gossip_socket = Arc::new(gossip_socket);
//         trace!(
//             "GossipService: id: {}, listening on: {:?}",
//             &cluster_info.id(),
//             gossip_socket.local_addr().unwrap()
//         );
//         let socket_addr_space = *cluster_info.socket_addr_space();
//         let t_receiver = streamer::receiver(
//             "solRcvrGossip".to_string(),
//             gossip_socket.clone(),
//             exit.clone(),
//             request_sender,
//             Recycler::default(),
//             Arc::new(StreamerReceiveStats::new("gossip_receiver")),
//             Duration::from_millis(1), // coalesce
//             false,
//             None,
//             false,
//         );
//         let (consume_sender, listen_receiver) = unbounded();
//         let t_socket_consume = cluster_info.clone().start_socket_consume_thread(
//             request_receiver,
//             consume_sender,
//             exit.clone(),
//         );
//         let (response_sender, response_receiver) = unbounded();
//         let t_listen = cluster_info.clone().listen(
//             bank_forks.clone(),
//             listen_receiver,
//             response_sender.clone(),
//             should_check_duplicate_instance,
//             exit.clone(),
//         );
//         let t_gossip =
//             cluster_info
//                 .clone()
//                 .gossip(bank_forks, response_sender, gossip_validators, exit);
//         let t_responder = streamer::responder(
//             "Gossip",
//             gossip_socket,
//             response_receiver,
//             socket_addr_space,
//             stats_reporter_sender,
//         );
//         let thread_hdls = vec![
//             t_receiver,
//             t_responder,
//             t_socket_consume,
//             t_listen,
//             t_gossip,
//         ];
//         Self { thread_hdls }
//     }

//     pub fn join(self) -> thread::Result<()> {
//         for thread_hdl in self.thread_hdls {
//             thread_hdl.join()?;
//         }
//         Ok(())
//     }
// }
