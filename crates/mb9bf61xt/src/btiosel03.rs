#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    btsel0123: BTSEL0123,
}
impl RegisterBlock {
    #[doc = "0x00 - I/O Select Register"]
    #[inline(always)]
    pub const fn btsel0123(&self) -> &BTSEL0123 {
        &self.btsel0123
    }
}
#[doc = "BTSEL0123 (rw) register accessor: I/O Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btsel0123::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btsel0123::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btsel0123`]
module"]
pub type BTSEL0123 = crate::Reg<btsel0123::BTSEL0123_SPEC>;
#[doc = "I/O Select Register"]
pub mod btsel0123;
