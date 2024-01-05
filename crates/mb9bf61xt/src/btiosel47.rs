#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    btsel4567: BTSEL4567,
}
impl RegisterBlock {
    #[doc = "0x00 - I/O Select Register"]
    #[inline(always)]
    pub const fn btsel4567(&self) -> &BTSEL4567 {
        &self.btsel4567
    }
}
#[doc = "BTSEL4567 (rw) register accessor: I/O Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btsel4567::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btsel4567::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btsel4567`]
module"]
pub type BTSEL4567 = crate::Reg<btsel4567::BTSEL4567_SPEC>;
#[doc = "I/O Select Register"]
pub mod btsel4567;
