use std::ffi::{CStr, CString};
use std::ops::Drop;
use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
use ::rustc_serialize::{Encodable,Decodable,Encoder,Decoder};

use std::path::Path;
use std::fs::{File,PathExt};
use std::io::{Read,Write};

use std::convert::Into;
use std::ops::{Deref,DerefMut};
