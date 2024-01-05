#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    scm_ctl: SCM_CTL,
    _reserved1: [u8; 0x03],
    scm_str: SCM_STR,
    _reserved2: [u8; 0x03],
    stb_ctl: STB_CTL,
    rst_str: RST_STR,
    _reserved4: [u8; 0x02],
    bsc_psr: BSC_PSR,
    _reserved5: [u8; 0x03],
    apbc0_psr: APBC0_PSR,
    _reserved6: [u8; 0x03],
    apbc1_psr: APBC1_PSR,
    _reserved7: [u8; 0x03],
    apbc2_psr: APBC2_PSR,
    _reserved8: [u8; 0x03],
    swc_psr: SWC_PSR,
    _reserved9: [u8; 0x07],
    ttc_psr: TTC_PSR,
    _reserved10: [u8; 0x07],
    csw_tmr: CSW_TMR,
    _reserved11: [u8; 0x03],
    psw_tmr: PSW_TMR,
    _reserved12: [u8; 0x03],
    pll_ctl1: PLL_CTL1,
    _reserved13: [u8; 0x03],
    pll_ctl2: PLL_CTL2,
    _reserved14: [u8; 0x03],
    csv_ctl: CSV_CTL,
    _reserved15: [u8; 0x02],
    csv_str: CSV_STR,
    _reserved16: [u8; 0x03],
    fcswh_ctl: FCSWH_CTL,
    _reserved17: [u8; 0x02],
    fcswl_ctl: FCSWL_CTL,
    _reserved18: [u8; 0x02],
    fcswd_ctl: FCSWD_CTL,
    _reserved19: [u8; 0x02],
    dbwdt_ctl: DBWDT_CTL,
    _reserved20: [u8; 0x0b],
    int_enr: INT_ENR,
    _reserved21: [u8; 0x03],
    int_str: INT_STR,
    _reserved22: [u8; 0x03],
    int_clr: INT_CLR,
}
impl RegisterBlock {
    #[doc = "0x00 - System Clock Mode Control Register"]
    #[inline(always)]
    pub const fn scm_ctl(&self) -> &SCM_CTL {
        &self.scm_ctl
    }
    #[doc = "0x04 - System Clock Mode Status Register"]
    #[inline(always)]
    pub const fn scm_str(&self) -> &SCM_STR {
        &self.scm_str
    }
    #[doc = "0x08 - Standby Mode Control Register"]
    #[inline(always)]
    pub const fn stb_ctl(&self) -> &STB_CTL {
        &self.stb_ctl
    }
    #[doc = "0x0c - Reset Cause Register"]
    #[inline(always)]
    pub const fn rst_str(&self) -> &RST_STR {
        &self.rst_str
    }
    #[doc = "0x10 - Base Clock Prescaler Register"]
    #[inline(always)]
    pub const fn bsc_psr(&self) -> &BSC_PSR {
        &self.bsc_psr
    }
    #[doc = "0x14 - APB0 Prescaler Register"]
    #[inline(always)]
    pub const fn apbc0_psr(&self) -> &APBC0_PSR {
        &self.apbc0_psr
    }
    #[doc = "0x18 - APB1 Prescaler Register"]
    #[inline(always)]
    pub const fn apbc1_psr(&self) -> &APBC1_PSR {
        &self.apbc1_psr
    }
    #[doc = "0x1c - APB2 Prescaler Register"]
    #[inline(always)]
    pub const fn apbc2_psr(&self) -> &APBC2_PSR {
        &self.apbc2_psr
    }
    #[doc = "0x20 - Software Watchdog Clock Prescaler Register"]
    #[inline(always)]
    pub const fn swc_psr(&self) -> &SWC_PSR {
        &self.swc_psr
    }
    #[doc = "0x28 - Trace Clock Prescaler Register"]
    #[inline(always)]
    pub const fn ttc_psr(&self) -> &TTC_PSR {
        &self.ttc_psr
    }
    #[doc = "0x30 - Clock Stabilization Wait Time Register"]
    #[inline(always)]
    pub const fn csw_tmr(&self) -> &CSW_TMR {
        &self.csw_tmr
    }
    #[doc = "0x34 - PLL Clock Stabilization Wait Time Setup Register"]
    #[inline(always)]
    pub const fn psw_tmr(&self) -> &PSW_TMR {
        &self.psw_tmr
    }
    #[doc = "0x38 - PLL Control Register 1"]
    #[inline(always)]
    pub const fn pll_ctl1(&self) -> &PLL_CTL1 {
        &self.pll_ctl1
    }
    #[doc = "0x3c - PLL Control Register 2"]
    #[inline(always)]
    pub const fn pll_ctl2(&self) -> &PLL_CTL2 {
        &self.pll_ctl2
    }
    #[doc = "0x40 - CSV control register"]
    #[inline(always)]
    pub const fn csv_ctl(&self) -> &CSV_CTL {
        &self.csv_ctl
    }
    #[doc = "0x44 - CSV status register"]
    #[inline(always)]
    pub const fn csv_str(&self) -> &CSV_STR {
        &self.csv_str
    }
    #[doc = "0x48 - Frequency detection window setting register"]
    #[inline(always)]
    pub const fn fcswh_ctl(&self) -> &FCSWH_CTL {
        &self.fcswh_ctl
    }
    #[doc = "0x4c - Frequency detection window setting register"]
    #[inline(always)]
    pub const fn fcswl_ctl(&self) -> &FCSWL_CTL {
        &self.fcswl_ctl
    }
    #[doc = "0x50 - Frequency detection counter register"]
    #[inline(always)]
    pub const fn fcswd_ctl(&self) -> &FCSWD_CTL {
        &self.fcswd_ctl
    }
    #[doc = "0x54 - Debug Break Watchdog Timer Control Register"]
    #[inline(always)]
    pub const fn dbwdt_ctl(&self) -> &DBWDT_CTL {
        &self.dbwdt_ctl
    }
    #[doc = "0x60 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_enr(&self) -> &INT_ENR {
        &self.int_enr
    }
    #[doc = "0x64 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn int_str(&self) -> &INT_STR {
        &self.int_str
    }
    #[doc = "0x68 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
}
#[doc = "SCM_CTL (rw) register accessor: System Clock Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scm_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scm_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scm_ctl`]
module"]
pub type SCM_CTL = crate::Reg<scm_ctl::SCM_CTL_SPEC>;
#[doc = "System Clock Mode Control Register"]
pub mod scm_ctl;
#[doc = "SCM_STR (r) register accessor: System Clock Mode Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scm_str::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scm_str`]
module"]
pub type SCM_STR = crate::Reg<scm_str::SCM_STR_SPEC>;
#[doc = "System Clock Mode Status Register"]
pub mod scm_str;
#[doc = "BSC_PSR (rw) register accessor: Base Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsc_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsc_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bsc_psr`]
module"]
pub type BSC_PSR = crate::Reg<bsc_psr::BSC_PSR_SPEC>;
#[doc = "Base Clock Prescaler Register"]
pub mod bsc_psr;
#[doc = "APBC0_PSR (rw) register accessor: APB0 Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbc0_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbc0_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbc0_psr`]
module"]
pub type APBC0_PSR = crate::Reg<apbc0_psr::APBC0_PSR_SPEC>;
#[doc = "APB0 Prescaler Register"]
pub mod apbc0_psr;
#[doc = "APBC1_PSR (rw) register accessor: APB1 Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbc1_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbc1_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbc1_psr`]
module"]
pub type APBC1_PSR = crate::Reg<apbc1_psr::APBC1_PSR_SPEC>;
#[doc = "APB1 Prescaler Register"]
pub mod apbc1_psr;
#[doc = "APBC2_PSR (rw) register accessor: APB2 Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbc2_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbc2_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apbc2_psr`]
module"]
pub type APBC2_PSR = crate::Reg<apbc2_psr::APBC2_PSR_SPEC>;
#[doc = "APB2 Prescaler Register"]
pub mod apbc2_psr;
#[doc = "SWC_PSR (rw) register accessor: Software Watchdog Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swc_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swc_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swc_psr`]
module"]
pub type SWC_PSR = crate::Reg<swc_psr::SWC_PSR_SPEC>;
#[doc = "Software Watchdog Clock Prescaler Register"]
pub mod swc_psr;
#[doc = "TTC_PSR (rw) register accessor: Trace Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttc_psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttc_psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttc_psr`]
module"]
pub type TTC_PSR = crate::Reg<ttc_psr::TTC_PSR_SPEC>;
#[doc = "Trace Clock Prescaler Register"]
pub mod ttc_psr;
#[doc = "CSW_TMR (rw) register accessor: Clock Stabilization Wait Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csw_tmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csw_tmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csw_tmr`]
module"]
pub type CSW_TMR = crate::Reg<csw_tmr::CSW_TMR_SPEC>;
#[doc = "Clock Stabilization Wait Time Register"]
pub mod csw_tmr;
#[doc = "PSW_TMR (rw) register accessor: PLL Clock Stabilization Wait Time Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psw_tmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psw_tmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psw_tmr`]
module"]
pub type PSW_TMR = crate::Reg<psw_tmr::PSW_TMR_SPEC>;
#[doc = "PLL Clock Stabilization Wait Time Setup Register"]
pub mod psw_tmr;
#[doc = "PLL_CTL1 (rw) register accessor: PLL Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ctl1`]
module"]
pub type PLL_CTL1 = crate::Reg<pll_ctl1::PLL_CTL1_SPEC>;
#[doc = "PLL Control Register 1"]
pub mod pll_ctl1;
#[doc = "PLL_CTL2 (rw) register accessor: PLL Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ctl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ctl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_ctl2`]
module"]
pub type PLL_CTL2 = crate::Reg<pll_ctl2::PLL_CTL2_SPEC>;
#[doc = "PLL Control Register 2"]
pub mod pll_ctl2;
#[doc = "DBWDT_CTL (rw) register accessor: Debug Break Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbwdt_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbwdt_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbwdt_ctl`]
module"]
pub type DBWDT_CTL = crate::Reg<dbwdt_ctl::DBWDT_CTL_SPEC>;
#[doc = "Debug Break Watchdog Timer Control Register"]
pub mod dbwdt_ctl;
#[doc = "INT_ENR (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_enr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_enr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_enr`]
module"]
pub type INT_ENR = crate::Reg<int_enr::INT_ENR_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod int_enr;
#[doc = "INT_STR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_str::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_str`]
module"]
pub type INT_STR = crate::Reg<int_str::INT_STR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod int_str;
#[doc = "INT_CLR (w) register accessor: Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`]
module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod int_clr;
#[doc = "RST_STR (r) register accessor: Reset Cause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_str::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_str`]
module"]
pub type RST_STR = crate::Reg<rst_str::RST_STR_SPEC>;
#[doc = "Reset Cause Register"]
pub mod rst_str;
#[doc = "STB_CTL (rw) register accessor: Standby Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stb_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stb_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stb_ctl`]
module"]
pub type STB_CTL = crate::Reg<stb_ctl::STB_CTL_SPEC>;
#[doc = "Standby Mode Control Register"]
pub mod stb_ctl;
#[doc = "CSV_CTL (rw) register accessor: CSV control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csv_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csv_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csv_ctl`]
module"]
pub type CSV_CTL = crate::Reg<csv_ctl::CSV_CTL_SPEC>;
#[doc = "CSV control register"]
pub mod csv_ctl;
#[doc = "CSV_STR (r) register accessor: CSV status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csv_str::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csv_str`]
module"]
pub type CSV_STR = crate::Reg<csv_str::CSV_STR_SPEC>;
#[doc = "CSV status register"]
pub mod csv_str;
#[doc = "FCSWH_CTL (rw) register accessor: Frequency detection window setting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcswh_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcswh_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcswh_ctl`]
module"]
pub type FCSWH_CTL = crate::Reg<fcswh_ctl::FCSWH_CTL_SPEC>;
#[doc = "Frequency detection window setting register"]
pub mod fcswh_ctl;
#[doc = "FCSWL_CTL (rw) register accessor: Frequency detection window setting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcswl_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcswl_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcswl_ctl`]
module"]
pub type FCSWL_CTL = crate::Reg<fcswl_ctl::FCSWL_CTL_SPEC>;
#[doc = "Frequency detection window setting register"]
pub mod fcswl_ctl;
#[doc = "FCSWD_CTL (r) register accessor: Frequency detection counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcswd_ctl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcswd_ctl`]
module"]
pub type FCSWD_CTL = crate::Reg<fcswd_ctl::FCSWD_CTL_SPEC>;
#[doc = "Frequency detection counter register"]
pub mod fcswd_ctl;
