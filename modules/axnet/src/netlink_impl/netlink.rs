
use alloc::vec;
use super::{NETLINK_SOCKET_SET, NetlinkSockSetWrapper};
use axerrno::AxResult;
use netlink_packet_core::{DoneMessage, FakeNetlinkInnerMessage, NetlinkHeader, NetlinkMessage, NetlinkPayload};

// use axprocess::current_process;

/// A netlink socket.
pub struct NetlinkSocket {
    handle: usize,
    seq: u32,
    pid: u64,
}

impl Drop for NetlinkSocket {
    fn drop(&mut self) {
        NETLINK_SOCKET_SET.remove(self.handle);
    }
}


impl NetlinkSocket {
    /// Create a new netlink socket.
    pub fn new(pid: u64) -> Self {
        let sock = NetlinkSockSetWrapper::new_netlink_socket();
        let handle = NETLINK_SOCKET_SET.add(sock);
        NetlinkSocket {
            handle,
            seq: 0,
            pid: pid
        }
    }

    /// Transmits data in the given buffer.
    pub fn send(&self, buf: &[u8]) -> AxResult<usize> {
        let header = NetlinkHeader::parseheader(buf)?;
        match &header.message_type {
            RTM_GETLINK => {
                self.send_getlink(&header)
            }
            RTM_GETADDR => {
                self.send_getaddr()
            }
            _ => {
                unimplemented!("NetlinkSocket::send: unsupported message type")
            }
        }
    }


    /// Receives data into the given buffer.
    pub fn recv(&self, buf: &mut [u8]) -> AxResult<usize> {
        NETLINK_SOCKET_SET.with_socket_mut(self.handle, |socket| {
            socket.recv(buf)
        })
    }

    
    fn send_getlink(&self, header: &NetlinkHeader) -> AxResult<usize> {
        // return done message
        error!("NetlinkSocket::send_getlink: {:?}", header);
        let extended_ack = vec![0u8; 0];
        let done_msg = DoneMessage::new(0, extended_ack);

        let mut done = NetlinkMessage::new(header.clone(), NetlinkPayload::<FakeNetlinkInnerMessage>::Done(done_msg.clone()),);
        done.header.message_type = 3; // NLMSG_DONE
        done.header.flags = 2; // NLM_F_MULTI

        NETLINK_SOCKET_SET.with_socket_mut(self.handle, |socket| {
            let mut buf = vec![0u8; done.buffer_len()];
            done.serialize(&mut buf);
            error!("NetlinkSocket::send_getlink: sending done message: {:?}", buf);
            socket.send(&buf)
        })
    }
    
    fn send_getaddr(&self) -> AxResult<usize> {
        todo!("NetlinkSocket::send_getaddr")
    } 
}