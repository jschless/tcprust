pub struct TcpState {}

impl Default for TcpState {
    fn default() -> Self {
        TcpState {}
    }
}

impl TcpState {
    pub fn on_packet<'a>(
        &mut self,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) {
        eprintln!(
            "{}:{} -> {}:{}    {} bytes of tcp to port {}",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len(),
            tcph.destination_port()
        );
    }
}
