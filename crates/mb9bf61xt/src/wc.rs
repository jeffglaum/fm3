#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    wcrd: WCRD,
    wcrl: WCRL,
    wccr: WCCR,
    _reserved3: [u8; 0x0d],
    clk_sel: CLK_SEL,
    _reserved4: [u8; 0x02],
    clk_en: CLK_EN,
}
impl RegisterBlock {
    #[doc = "0x00 - Watch Counter Read Register"]
    #[inline(always)]
    pub const fn wcrd(&self) -> &WCRD {
        &self.wcrd
    }
    #[doc = "0x01 - Watch Counter Reload Register"]
    #[inline(always)]
    pub const fn wcrl(&self) -> &WCRL {
        &self.wcrl
    }
    #[doc = "0x02 - Watch Counter Control Register"]
    #[inline(always)]
    pub const fn wccr(&self) -> &WCCR {
        &self.wccr
    }
    #[doc = "0x10 - Clock Selection Register"]
    #[inline(always)]
    pub const fn clk_sel(&self) -> &CLK_SEL {
        &self.clk_sel
    }
    #[doc = "0x14 - Division Clock Enable Register"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
}
#[doc = "WCRD (r) register accessor: Watch Counter Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcrd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcrd`]
module"]
pub type WCRD = crate::Reg<wcrd::WCRD_SPEC>;
#[doc = "Watch Counter Read Register"]
pub mod wcrd;
#[doc = "WCRL (rw) register accessor: Watch Counter Reload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wcrl`]
module"]
pub type WCRL = crate::Reg<wcrl::WCRL_SPEC>;
#[doc = "Watch Counter Reload Register"]
pub mod wcrl;
#[doc = "WCCR (rw) register accessor: Watch Counter Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wccr`]
module"]
pub type WCCR = crate::Reg<wccr::WCCR_SPEC>;
#[doc = "Watch Counter Control Register"]
pub mod wccr;
#[doc = "CLK_SEL (rw) register accessor: Clock Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sel`]
module"]
pub type CLK_SEL = crate::Reg<clk_sel::CLK_SEL_SPEC>;
#[doc = "Clock Selection Register"]
pub mod clk_sel;
#[doc = "CLK_EN (rw) register accessor: Division Clock Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`]
module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "Division Clock Enable Register"]
pub mod clk_en;
