pub use std::{cmp, fmt, iter, mem, ptr, slice, str, unreachable};

pub use std::format;

pub mod vec {
    pub use std::vec::Vec;
    pub use std::vec;
}

pub mod string {
    pub use std::string::{String, ToString};
}

pub mod borrow {
    pub use std::borrow::ToOwned;
}

pub mod boxed {
    pub use std::boxed::Box;
}

pub mod convert {
    pub use std::convert::{From, Into, TryFrom};
}

pub mod ffi {
    pub use std::ffi::{CStr, CString, OsStr, OsString};
}

pub mod sync {
    pub use std::sync::{Mutex};
}

pub mod io {
    pub use std::io::{Error, ErrorKind, Result};

    pub mod prelude {
        pub use std::io::prelude::{Read};
    }
}

pub mod hash {
    pub use std::hash::{Hash, Hasher};
}

pub mod option {
    pub use std::option::Option;
}

pub mod marker {
    pub use std::marker::{Copy, PhantomData};
}

pub mod clone {
    pub use std::clone::Clone;
}

pub mod default {
    pub use std::default::Default;
}

pub mod ops {
    pub use std::ops::{Add, Sub, RangeInclusive, Deref, DerefMut};
}

pub mod os {
    pub mod unix {
        pub mod ffi {
            pub use std::os::unix::ffi::OsStrExt;
        }
        pub mod fs {
            pub use std::os::unix::fs::{MetadataExt, symlink};
        }
        pub mod io {
            pub use std::os::unix::io::FromRawFd;
        }
    }
}

pub mod path {
    pub use std::path::{Path, PathBuf, MAIN_SEPARATOR};
}

pub mod fs {
    pub use std::fs::{read_dir, read_link, remove_file, File, metadata};
}

pub mod process {
    pub use std::process::id;
}
