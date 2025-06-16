use fuser::ReplyBacking;
use std::os::fd::AsFd;

pub type FusePassthroughInterfaceDyn<'a> = FusePassthroughInterface<FusePassthroughInterfaceKind<'a>>;
pub type FusePassthroughInterfaceCreate<'a> = FusePassthroughInterface<&'a fuser::ReplyCreate>;
pub type FusePassthroughInterfaceOpen<'a> = FusePassthroughInterface<&'a fuser::ReplyOpen>;

#[derive(Debug, Clone, Copy)]
pub struct FusePassthroughInterface<R>(pub(crate) R);
impl<R: PassthroughInterface> FusePassthroughInterface<R>
{
	/// See [`ReplyBacking::open_backing`].
    #[inline]
    pub fn open_backing(&self, fd: impl AsFd) -> std::io::Result<fuser::BackingId>
	{
        self.0.open_backing(fd.as_fd())
    }
}
impl<'a> FusePassthroughInterfaceCreate<'a> {
	#[inline]
	pub fn into_dyn(self) -> FusePassthroughInterfaceDyn<'a> {
		FusePassthroughInterface(FusePassthroughInterfaceKind::Create(self.0))
	}
}
impl<'a> FusePassthroughInterfaceOpen<'a> {
	#[inline]
	pub fn into_dyn(self) -> FusePassthroughInterfaceDyn<'a> {
		FusePassthroughInterface(FusePassthroughInterfaceKind::Open(self.0))
	}
}

mod private {
	use super::*;

	pub trait PassthroughInterface {
		fn open_backing(&self, fd: impl AsFd) -> std::io::Result<fuser::BackingId>;
	}
	impl PassthroughInterface for &fuser::ReplyCreate {
		#[inline]
		fn open_backing(&self, fd: impl AsFd) -> std::io::Result<fuser::BackingId> {
			fuser::ReplyCreate::open_backing(*self, fd)
		}
	}
	impl PassthroughInterface for &fuser::ReplyOpen {
		#[inline]
		fn open_backing(&self, fd: impl AsFd) -> std::io::Result<fuser::BackingId> {
			fuser::ReplyOpen::open_backing(*self, fd)
		}
	}
	impl PassthroughInterface for FusePassthroughInterfaceKind<'_> {
		#[inline]
		fn open_backing(&self, fd: impl AsFd) -> std::io::Result<fuser::BackingId> {
			match self {
				Self::Create(reply) => reply.open_backing(fd),
				Self::Open(reply) => reply.open_backing(fd),
			}
		}
	}

	#[derive(Debug, Clone, Copy)]
	pub enum FusePassthroughInterfaceKind<'a> {
		Create(&'a fuser::ReplyCreate),
		Open(&'a fuser::ReplyOpen),
	}
	impl fuser::ReplyBacking for FusePassthroughInterfaceKind<'_> {
		#[inline]
		fn open_backing(&self, fd: impl AsFd) -> std::io::Result<fuser::BackingId> {
			match self {
				Self::Create(reply) => reply.open_backing(fd),
				Self::Open(reply) => reply.open_backing(fd),
			}
		}
	}
}
use private::*;

#[allow(dead_code)]
#[allow(unreachable_code)]
#[allow(unused_variables)]
fn assert_open_backing_compiles() {
	let create: fuser::ReplyCreate = todo!();
	let fd: std::os::fd::OwnedFd = todo!();
	let _ = FusePassthroughInterface(&create).open_backing(fd);
}