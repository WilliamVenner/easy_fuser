#[cfg(feature = "serial")]
mod serial {
    include!(concat!(env!("OUT_DIR"), "/serial/fuse_driver.rs"));
}

#[cfg(feature = "parallel")]
mod parallel {
    include!(concat!(env!("OUT_DIR"), "/parallel/fuse_driver.rs"));
}

#[cfg(feature = "async")]
mod async_task {
    include!(concat!(env!("OUT_DIR"), "/async/fuse_driver.rs"));
}

// `FuseDriver` is the only item these mode modules export to the rest of the crate, and
// it is re-exported publicly through the crate prelude (see `lib.rs`), so re-export it
// `pub` here rather than pulling the whole module in with a glob.
#[cfg(feature = "serial")]
pub use serial::FuseDriver;

#[cfg(feature = "parallel")]
pub use parallel::FuseDriver;

#[cfg(feature = "async")]
pub use async_task::FuseDriver;
