use fuser::ReplyBacking;
use std::os::fd::AsFd;

#[derive(Debug, Clone, Copy)]
pub struct FusePassthroughInterface<R>(pub(crate) R);
impl<R: ReplyBacking> FusePassthroughInterface<&R> {
    #[inline]
    pub fn open_backing(&self, fd: impl AsFd) -> std::io::Result<fuser::BackingId> {
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

	#[derive(Debug, Clone, Copy)]
	pub enum FusePassthroughInterfaceKind<'a> {
		Create(&'a fuser::ReplyCreate),
		Open(&'a fuser::ReplyOpen),
	}
	impl fuser::ReplyBacking for FusePassthroughInterfaceKind<'_> {
		fn open_backing(&self, fd: impl AsFd) -> std::io::Result<fuser::BackingId> {
			match self {
				Self::Create(reply) => reply.open_backing(fd),
				Self::Open(reply) => reply.open_backing(fd),
			}
		}
	}
}
use private::*;

pub type FusePassthroughInterfaceDyn<'a> = FusePassthroughInterface<FusePassthroughInterfaceKind<'a>>;
pub type FusePassthroughInterfaceCreate<'a> = FusePassthroughInterface<&'a fuser::ReplyCreate>;
pub type FusePassthroughInterfaceOpen<'a> = FusePassthroughInterface<&'a fuser::ReplyOpen>;
