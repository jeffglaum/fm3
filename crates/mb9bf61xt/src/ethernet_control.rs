#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    eth_mode: ETH_MODE,
    _reserved1: [u8; 0x04],
    eth_clkg: ETH_CLKG,
}
impl RegisterBlock {
    #[doc = "0x00 - Mode Select Register"]
    #[inline(always)]
    pub const fn eth_mode(&self) -> &ETH_MODE {
        &self.eth_mode
    }
    #[doc = "0x08 - Clock Gating Register"]
    #[inline(always)]
    pub const fn eth_clkg(&self) -> &ETH_CLKG {
        &self.eth_clkg
    }
}
#[doc = "ETH_MODE (rw) register accessor: Mode Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_mode`]
module"]
pub type ETH_MODE = crate::Reg<eth_mode::ETH_MODE_SPEC>;
#[doc = "Mode Select Register"]
pub mod eth_mode;
#[doc = "ETH_CLKG (rw) register accessor: Clock Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_clkg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_clkg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eth_clkg`]
module"]
pub type ETH_CLKG = crate::Reg<eth_clkg::ETH_CLKG_SPEC>;
#[doc = "Clock Gating Register"]
pub mod eth_clkg;
