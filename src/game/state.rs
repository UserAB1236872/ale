use ::libc::c_int;
use ::rustc_serialize::{Encodable,Encoder,Decodable,Decoder};
use ::ffi::*;

pub struct AleState {
    s: *mut CAleState,
}

impl Drop for AleState {
    fn drop(&mut self) {
        unsafe {
            deleteState(self.s);
        }
    }
}

impl Encodable for AleState {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(),S::Error> {
        let serial = encode_state(self.s);
        serial.encode(s)
    }
}

impl Decodable for AleState {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self,D::Error> {
        let serial = try!(Vec::decode(d));

        Ok(AleState{
            s: decode_state(&serial),
        })
    }
}

pub struct AleSystemState {
    s: *mut CAleState,
}

impl Drop for AleSystemState {
    fn drop(&mut self) {
        unsafe {
            deleteState(self.s)
        }
    }
}

impl Encodable for AleSystemState {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(),S::Error> {
        let serial = encode_state(self.s);
        serial.encode(s)
    }
}

impl Decodable for AleSystemState {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self,D::Error> {
        let serial = try!(Vec::decode(d));

        Ok(AleSystemState{
            s: decode_state(&serial),
        })
    }
}

fn encode_state(s: *mut CAleState) -> Vec<i8> {
    unsafe {
        let len = encodeStateLen(s) as usize;
        let mut buf = Vec::<i8>::with_capacity(len);
        buf.set_len(len);
        encodeState(s, buf.as_mut_ptr());

        buf
    }
}

fn decode_state(serialized: &Vec<i8>) -> *mut CAleState {
    unsafe {
        decodeState(serialized.as_ptr(), serialized.len() as c_int)
    }
}

pub mod protected {
    use ::ffi::CAleState;
    use super::{AleState,AleSystemState};

    pub trait Protected {
        #[inline]
        fn s(&self) -> *mut CAleState;
        fn new(s: *mut CAleState) -> Self;
    }

    impl Protected for AleState {
        fn s(&self) -> *mut CAleState {
            self.s
        }

        fn new(s: *mut CAleState) -> AleState {
            AleState{s: s}
        }
    }

    impl Protected for AleSystemState {
        fn s(&self) -> *mut CAleState {
            self.s
        }

        fn new(s: *mut CAleState) -> AleSystemState {
            AleSystemState{s: s}
        }
    }
}