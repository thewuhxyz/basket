use {
    solana_gossip::{
        gossip_service::discover,
        contact_info::ContactInfo,
    },
    solana_sdk::signature::Keypair,
    solana_streamer::socket::SocketAddrSpace,
    std::{
        net::{SocketAddr, TcpStream, ToSocketAddrs},
        time::Duration,
        io,
    },
};

fn test_connectivity(addr: &str) -> io::Result<()> {
    println!("Testing connectivity to {}", addr);
    
    let socket_addrs: Vec<SocketAddr> = addr.to_socket_addrs()?.collect();
    println!("Resolved addresses: {:?}", socket_addrs);
    
    for socket_addr in socket_addrs {
        match TcpStream::connect_timeout(&socket_addr, Duration::from_secs(5)) {
            Ok(_) => println!("Successfully connected to {}", socket_addr),
            Err(e) => println!("Failed to connect to {}: {}", socket_addr, e),
        }
    }
    Ok(())
}

fn try_discover(entrypoint: &SocketAddr, shred_version: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("Attempting discover with shred_version: {}", shred_version);
    match discover(
        None,
        Some(entrypoint),
        Some(1),
        Duration::from_secs(15),
        None,
        None,
        None,
        shred_version,
        SocketAddrSpace::new(true),
    ) {
        Ok((peers, validators)) => {
            println!("Success! Found {} peers, {} validators", 
                peers.len(), validators.len());
            
            for peer in peers.iter().take(3) {
                println!("Peer: {} Gossip: {:?}", peer.pubkey(), peer.gossip());
            }
            
            if !peers.is_empty() {
                return Ok(());
            }
        }
        Err(e) => println!("Discovery failed: {}", e),
    }
    Err("No peers found".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    solana_logger::setup_with_default("solana_gossip=debug");

    let entrypoints = [
        ("mainnet", "34.83.231.147:8001"),
        ("testnet", "35.203.170.30:8001"),
    ];

    for (name, addr) in entrypoints.iter() {
        println!("\nTesting {} ({})", name, addr);
        
        if let Err(e) = test_connectivity(addr) {
            println!("Connectivity test failed: {}", e);
            continue;
        }
        println!("Basic connectivity test passed");

        let entrypoint = addr.parse::<SocketAddr>()?;

        // Test UDP socket
        println!("Testing UDP socket...");
        let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(&entrypoint)?;
        println!("Successfully connected UDP socket to {}", entrypoint);

        // Get the cluster's shred version
        match solana_net_utils::get_cluster_shred_version(&entrypoint) {
            Ok(cluster_version) => {
                println!("Cluster shred version: {}", cluster_version);
                
                // Try with cluster's shred version
                if try_discover(&entrypoint, cluster_version).is_ok() {
                    return Ok(());
                }

                // Try with spy mode (shred_version = 0)
                if try_discover(&entrypoint, 0).is_ok() {
                    return Ok(());
                }
            }
            Err(e) => println!("Failed to get cluster shred version: {}", e),
        }
    }

    Err("Failed to connect to any entrypoint".into())
}