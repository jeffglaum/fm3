#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    btsel89ab: BTSEL89AB,
}
impl RegisterBlock {
    #[doc = "0x00 - I/O Select Register"]
    #[inline(always)]
    pub const fn btsel89ab(&self) -> &BTSEL89AB {
        &self.btsel89ab
    }
}
#[doc = "BTSEL89AB (rw) register accessor: I/O Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btsel89ab::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btsel89ab::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btsel89ab`]
module"]
pub type BTSEL89AB = crate::Reg<btsel89ab::BTSEL89AB_SPEC>;
#[doc = "I/O Select Register"]
pub mod btsel89ab;
