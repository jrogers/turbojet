//! The Simple Open Framing Header (SOFH) before each message: six bytes, big-endian, the message
//! length as a u32 (the header included), then the encoding type, 0x5BE0 for SBE 1.0
//! little-endian.

use crate::sbe::{Encode, SbeError};

/// The framing header's length.
pub(crate) const HEADER: usize = 6;

/// The encoding type of SBE 1.0 little-endian, as registered for SOFH.
pub(crate) const ENCODING_TYPE: u16 = 0x5BE0;

/// The longest message accepted, framing included. SOFH allows 4 GiB; order entry messages are a
/// few hundred bytes, so 64 KiB bounds what a counterparty can make us buffer with room to spare.
pub(crate) const MAX_MESSAGE: usize = 64 * 1024;
const _: () = assert!(MAX_MESSAGE <= u32::MAX as usize);
// The smallest frame is a header and an SBE message header.
const _: () = assert!(HEADER + 8 <= MAX_MESSAGE);

/// What's at the start of a buffer.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Framed {
    /// A complete frame of `len` bytes, its SBE message from `HEADER` on.
    Message(usize),
    /// Not all of the next frame has arrived.
    Incomplete,
    /// The framing is wrong: the stream can't be read on from here.
    Invalid(&'static str),
}

/// The frame at the start of `buf`.
pub(crate) fn frame(buf: &[u8]) -> Framed {
    let Some(header) = buf.get(..HEADER) else { return Framed::Incomplete };
    let len = u32::from_be_bytes([header[0], header[1], header[2], header[3]]);
    if u16::from_be_bytes([header[4], header[5]]) != ENCODING_TYPE {
        return Framed::Invalid("the framing header's encoding type isn't SBE 1.0 little-endian (0x5BE0)");
    }
    // Over the limit, or more than the platform can address: either way, too long.
    let len = usize::try_from(len).unwrap_or(usize::MAX);
    // Too short to hold an SBE message header.
    if len < HEADER + 8 {
        return Framed::Invalid("the framing header's message length is too short for an SBE header");
    }
    if len > MAX_MESSAGE {
        return Framed::Invalid("the framing header's message length is over 64 KiB");
    }
    if buf.len() < len { Framed::Incomplete } else { Framed::Message(len) }
}

/// Appends `msg`, framed, to `out`.
///
/// # Errors
///
/// As [`Encode::encode_into`], or [`SbeError::TooLong`] if the frame is over [`MAX_MESSAGE`]; `out`
/// is then as it was.
pub(crate) fn push(out: &mut Vec<u8>, msg: &impl Encode) -> Result<(), SbeError> {
    let start = out.len();
    out.extend_from_slice(&[0; HEADER]);
    if let Err(e) = msg.encode_into(out) {
        out.truncate(start);
        return Err(e);
    }
    finish(out, start)
}

/// Appends `sbe`, an encoded SBE message, framed, to `out`.
///
/// # Errors
///
/// [`SbeError::TooLong`] if the frame is over [`MAX_MESSAGE`]; `out` is then as it was.
pub(crate) fn push_bytes(out: &mut Vec<u8>, sbe: &[u8]) -> Result<(), SbeError> {
    let start = out.len();
    out.extend_from_slice(&[0; HEADER]);
    out.extend_from_slice(sbe);
    finish(out, start)
}

/// Writes the header of the frame that starts at `start` and runs to the end of `out`.
fn finish(out: &mut Vec<u8>, start: usize) -> Result<(), SbeError> {
    let len = out.len() - start;
    let length = u32::try_from(len).ok().filter(|_| len <= MAX_MESSAGE);
    let Some(length) = length else {
        out.truncate(start);
        return Err(SbeError::TooLong { name: "message", len, max: MAX_MESSAGE });
    };
    out[start..start + 4].copy_from_slice(&length.to_be_bytes());
    out[start + 4..start + HEADER].copy_from_slice(&ENCODING_TYPE.to_be_bytes());
    // Paired with `frame`: what's written reads back as the same frame.
    debug_assert_eq!(frame(&out[start..]), Framed::Message(len));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_frame_reads_back_as_written() {
        let mut out = Vec::new();
        push_bytes(&mut out, &[1, 0, 2, 0, 1, 0, 5, 0, 9]).unwrap();
        assert_eq!(out[..HEADER], [0, 0, 0, 15, 0x5B, 0xE0]);
        assert_eq!(frame(&out), Framed::Message(15));
        assert_eq!(frame(&out[..14]), Framed::Incomplete);
        assert_eq!(frame(&out[..5]), Framed::Incomplete);
    }

    #[test]
    fn bad_framing_is_invalid() {
        assert!(matches!(frame(&[0, 0, 0, 15, 0xCA, 0xFE]), Framed::Invalid(_)));
        assert!(matches!(frame(&[0, 0, 0, 13, 0x5B, 0xE0]), Framed::Invalid(_)));
        assert!(matches!(frame(&[0xFF, 0xFF, 0xFF, 0xFF, 0x5B, 0xE0]), Framed::Invalid(_)));
    }

    #[test]
    fn a_message_too_long_to_frame_is_refused() {
        let mut out = vec![7];
        let error = push_bytes(&mut out, &vec![0; MAX_MESSAGE]).unwrap_err();
        assert!(matches!(error, SbeError::TooLong { .. }));
        assert_eq!(out, [7]);
    }
}
