#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xfc],
    btsssr: BTSSSR,
}
impl RegisterBlock {
    #[doc = "0xfc - Software-based Simultaneous Startup Register"]
    #[inline(always)]
    pub const fn btsssr(&self) -> &BTSSSR {
        &self.btsssr
    }
}
#[doc = "BTSSSR (w) register accessor: Software-based Simultaneous Startup Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btsssr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btsssr`]
module"]
pub type BTSSSR = crate::Reg<btsssr::BTSSSR_SPEC>;
#[doc = "Software-based Simultaneous Startup Register"]
pub mod btsssr;
