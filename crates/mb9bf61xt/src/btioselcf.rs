#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    btselcdef: BTSELCDEF,
}
impl RegisterBlock {
    #[doc = "0x00 - I/O Select Register"]
    #[inline(always)]
    pub const fn btselcdef(&self) -> &BTSELCDEF {
        &self.btselcdef
    }
}
#[doc = "BTSELCDEF (rw) register accessor: I/O Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btselcdef::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btselcdef::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btselcdef`]
module"]
pub type BTSELCDEF = crate::Reg<btselcdef::BTSELCDEF_SPEC>;
#[doc = "I/O Select Register"]
pub mod btselcdef;
