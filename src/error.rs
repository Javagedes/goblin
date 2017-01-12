use scroll;
use core::result;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: ::std::io::Error) {
            from()
        }
        Scroll(err: scroll::Error) {
            from()
        }
        BadMagic(magic: u64) {
            description("Invalid magic number")
                display("Invalid magic number: 0x{:x}", magic)
        }
        // TODO: remove this, hack for archive
        BadFile {
            from(String)
        }
        Malformed(msg: String) {
            description("Binary is malformed in some way")
                display("Malformed binary: {}", msg)
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
