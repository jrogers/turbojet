//! Receive timestamps on a spinning connection's socket: how long the bytes of each read were in
//! the host before Turbojet read them.
//!
//! On Linux, [`SpinningStream::with_receive_timestamps`](crate::connection::SpinningStream::with_receive_timestamps)
//! asks the kernel to stamp each packet it receives, with the network card's clock if the card
//! stamps packets (`SOF_TIMESTAMPING_RX_HARDWARE`), else with its own on receiving it
//! (`SOF_TIMESTAMPING_RX_SOFTWARE`), and reads with `recvmsg` to get the stamp of the latest
//! packet in each read. The wall clock at the read, less that stamp, is the read's wire-to-read
//! time.
//!
//! The stream reaches the driver only as an `AsyncRead`, so it leaves that time here, for the
//! read just done on this thread: the spinning driver runs a connection on one thread and takes
//! it after each read ([`take_wire_to_read`]), recording it if the session has latency metrics.

use std::cell::Cell;
use std::time::Duration;

thread_local! {
    /// The wire-to-read time of the latest read by a stream with receive timestamps on this
    /// thread, or `None` if it had no stamp.
    static WIRE_TO_READ: Cell<Option<Duration>> = const { Cell::new(None) };
}

/// The wire-to-read time of the latest read on this thread, if it had one: taken, so a read
/// without one leaves nothing behind.
pub(crate) fn take_wire_to_read() -> Option<Duration> {
    WIRE_TO_READ.with(Cell::take)
}

#[cfg(target_os = "linux")]
pub(crate) use linux::{control_buffer, enable, read};

#[cfg(target_os = "linux")]
mod linux {
    use std::io::{self, IoSliceMut};
    use std::net::TcpStream;
    use std::os::fd::AsRawFd;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    use nix::sys::socket::{self, ControlMessageOwned, MsgFlags, TimestampingFlag, Timestamps, sockopt};
    use nix::sys::time::TimeSpec;

    /// Asks the kernel to stamp what `socket` receives, by the network card if it can, and to
    /// report the stamps with each read.
    pub(crate) fn enable(socket: &TcpStream) -> io::Result<()> {
        let flags = TimestampingFlag::SOF_TIMESTAMPING_RX_HARDWARE
            | TimestampingFlag::SOF_TIMESTAMPING_RAW_HARDWARE
            | TimestampingFlag::SOF_TIMESTAMPING_RX_SOFTWARE
            | TimestampingFlag::SOF_TIMESTAMPING_SOFTWARE;
        socket::setsockopt(socket, sockopt::Timestamping, &flags)?;
        Ok(())
    }

    /// A buffer for [`read`]'s control message, made once per stream.
    pub(crate) fn control_buffer() -> Box<[u8]> {
        nix::cmsg_space!(Timestamps).into_boxed_slice()
    }

    /// Reads into `buf`, as `read` would, leaving the read's wire-to-read time for
    /// [`take_wire_to_read`](super::take_wire_to_read).
    pub(crate) fn read(socket: &TcpStream, buf: &mut [u8], control: &mut [u8]) -> io::Result<usize> {
        let mut iov = [IoSliceMut::new(buf)];
        let msg = socket::recvmsg::<()>(socket.as_raw_fd(), &mut iov, Some(control), MsgFlags::empty())?;
        // A truncated control message loses the stamp, not the bytes read.
        let stamps = msg.cmsgs().ok().and_then(|mut cmsgs| {
            cmsgs.find_map(|cmsg| match cmsg {
                ControlMessageOwned::ScmTimestampsns(stamps) => Some(stamps),
                _ => None,
            })
        });
        let wire_to_read = stamps.and_then(|stamps| since(&stamps, SystemTime::now()));
        super::WIRE_TO_READ.with(|cell| cell.set(wire_to_read));
        Ok(msg.bytes)
    }

    /// How long before `now` the packet was stamped: by the card if it did, else by the kernel.
    /// `None` without a stamp, or with one after `now`, as when the system clock isn't synced to
    /// the card's.
    fn since(stamps: &Timestamps, now: SystemTime) -> Option<Duration> {
        let stamp = [stamps.hw_raw, stamps.system].into_iter().find(|stamp| *stamp != TimeSpec::new(0, 0))?;
        now.duration_since(UNIX_EPOCH + Duration::from(stamp)).ok()
    }

    #[cfg(test)]
    mod tests {
        use std::io::Write;

        use super::*;

        #[test]
        fn the_cards_stamp_is_preferred_and_a_future_one_ignored() {
            let at = |secs| TimeSpec::new(secs, 0);
            let now = UNIX_EPOCH + Duration::from_secs(100);
            let stamps = |system, hw_raw| Timestamps { system, hw_trans: at(0), hw_raw };
            assert_eq!(since(&stamps(at(90), at(99)), now), Some(Duration::from_secs(1)));
            assert_eq!(since(&stamps(at(90), at(0)), now), Some(Duration::from_secs(10)));
            assert_eq!(since(&stamps(at(0), at(0)), now), None);
            assert_eq!(since(&stamps(at(0), at(101)), now), None);
        }

        /// Loopback has the kernel's stamps, if not a card's.
        #[test]
        fn a_read_over_loopback_has_a_stamp() {
            let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
            let mut sender = TcpStream::connect(listener.local_addr().unwrap()).unwrap();
            let (receiver, _) = listener.accept().unwrap();
            enable(&receiver).unwrap();
            sender.write_all(b"8=FIX.4.4\x01").unwrap();
            let (mut buf, mut control) = ([0; 64], control_buffer());
            assert_eq!(read(&receiver, &mut buf, &mut control).unwrap(), 10);
            let took = super::super::take_wire_to_read().expect("a stamp");
            assert!(took < Duration::from_secs(10), "{took:?}");
            assert_eq!(super::super::take_wire_to_read(), None, "taken");
        }
    }
}
