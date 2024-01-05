#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_rt_rt_pcsr: [u8; 0x02],
    _reserved1: [u8; 0x02],
    _reserved_1_ppg_ppg_prlh: [u8; 0x02],
    _reserved2: [u8; 0x02],
    _reserved_2_rt_rt_tmr: [u8; 0x02],
    _reserved3: [u8; 0x02],
    _reserved_3_rt_rt_tmcr: [u8; 0x02],
    _reserved4: [u8; 0x02],
    _reserved_4_rt_rt_stc: [u8; 0x01],
    _reserved_5_rt_rt_tmcr2: [u8; 0x01],
}
impl RegisterBlock {
    #[doc = "0x00 - PWM Cycle Set Register"]
    #[inline(always)]
    pub const fn rt_rt_pcsr(&self) -> &RT_RT_PCSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - LOW Width Reload Register"]
    #[inline(always)]
    pub const fn ppg_ppg_prll(&self) -> &PPG_PPG_PRLL {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - PWM Cycle Set Register"]
    #[inline(always)]
    pub const fn pwm_pwm_pcsr(&self) -> &PWM_PWM_PCSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Data Buffer Register"]
    #[inline(always)]
    pub const fn pwc_pwc_dtbf(&self) -> &PWC_PWC_DTBF {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - HIGH Width Reload Register"]
    #[inline(always)]
    pub const fn ppg_ppg_prlh(&self) -> &PPG_PPG_PRLH {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - PWM Duty Set Register"]
    #[inline(always)]
    pub const fn pwm_pwm_pdut(&self) -> &PWM_PWM_PDUT {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Timer Register"]
    #[inline(always)]
    pub const fn rt_rt_tmr(&self) -> &RT_RT_TMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Timer Register"]
    #[inline(always)]
    pub const fn ppg_ppg_tmr(&self) -> &PPG_PPG_TMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Timer Register"]
    #[inline(always)]
    pub const fn pwm_pwm_tmr(&self) -> &PWM_PWM_TMR {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Timer Control Register"]
    #[inline(always)]
    pub const fn pwc_pwc_tmcr(&self) -> &PWC_PWC_TMCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Timer Control Register"]
    #[inline(always)]
    pub const fn rt_rt_tmcr(&self) -> &RT_RT_TMCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Timer Control Register"]
    #[inline(always)]
    pub const fn ppg_ppg_tmcr(&self) -> &PPG_PPG_TMCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x0c - Timer Control Register"]
    #[inline(always)]
    pub const fn pwm_pwm_tmcr(&self) -> &PWM_PWM_TMCR {
        unsafe { &*(self as *const Self).cast::<u8>().add(12).cast() }
    }
    #[doc = "0x10 - Status Control Register"]
    #[inline(always)]
    pub const fn pwc_pwc_stc(&self) -> &PWC_PWC_STC {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Status Control Register"]
    #[inline(always)]
    pub const fn rt_rt_stc(&self) -> &RT_RT_STC {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Status Control Register"]
    #[inline(always)]
    pub const fn ppg_ppg_stc(&self) -> &PPG_PPG_STC {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Status Control Register"]
    #[inline(always)]
    pub const fn pwm_pwm_stc(&self) -> &PWM_PWM_STC {
        unsafe { &*(self as *const Self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x11 - Timer Control Register 2"]
    #[inline(always)]
    pub const fn pwc_pwc_tmcr2(&self) -> &PWC_PWC_TMCR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(17).cast() }
    }
    #[doc = "0x11 - Timer Control Register 2"]
    #[inline(always)]
    pub const fn rt_rt_tmcr2(&self) -> &RT_RT_TMCR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(17).cast() }
    }
    #[doc = "0x11 - Timer Control Register 2"]
    #[inline(always)]
    pub const fn ppg_ppg_tmcr2(&self) -> &PPG_PPG_TMCR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(17).cast() }
    }
    #[doc = "0x11 - Timer Control Register 2"]
    #[inline(always)]
    pub const fn pwm_pwm_tmcr2(&self) -> &PWM_PWM_TMCR2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(17).cast() }
    }
}
#[doc = "PWM_PWM_TMCR (rw) register accessor: Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_tmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_tmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_tmcr`]
module"]
pub type PWM_PWM_TMCR = crate::Reg<pwm_pwm_tmcr::PWM_PWM_TMCR_SPEC>;
#[doc = "Timer Control Register"]
pub mod pwm_pwm_tmcr;
#[doc = "PWM_PWM_TMCR2 (rw) register accessor: Timer Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_tmcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_tmcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_tmcr2`]
module"]
pub type PWM_PWM_TMCR2 = crate::Reg<pwm_pwm_tmcr2::PWM_PWM_TMCR2_SPEC>;
#[doc = "Timer Control Register 2"]
pub mod pwm_pwm_tmcr2;
#[doc = "PWM_PWM_STC (rw) register accessor: Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_stc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_stc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_stc`]
module"]
pub type PWM_PWM_STC = crate::Reg<pwm_pwm_stc::PWM_PWM_STC_SPEC>;
#[doc = "Status Control Register"]
pub mod pwm_pwm_stc;
#[doc = "PWM_PWM_PCSR (rw) register accessor: PWM Cycle Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_pcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_pcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_pcsr`]
module"]
pub type PWM_PWM_PCSR = crate::Reg<pwm_pwm_pcsr::PWM_PWM_PCSR_SPEC>;
#[doc = "PWM Cycle Set Register"]
pub mod pwm_pwm_pcsr;
#[doc = "PWM_PWM_PDUT (rw) register accessor: PWM Duty Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_pdut::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_pwm_pdut::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_pdut`]
module"]
pub type PWM_PWM_PDUT = crate::Reg<pwm_pwm_pdut::PWM_PWM_PDUT_SPEC>;
#[doc = "PWM Duty Set Register"]
pub mod pwm_pwm_pdut;
#[doc = "PWM_PWM_TMR (r) register accessor: Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwm_pwm_tmr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwm_pwm_tmr`]
module"]
pub type PWM_PWM_TMR = crate::Reg<pwm_pwm_tmr::PWM_PWM_TMR_SPEC>;
#[doc = "Timer Register"]
pub mod pwm_pwm_tmr;
#[doc = "PPG_PPG_TMCR (rw) register accessor: Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_tmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppg_ppg_tmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppg_ppg_tmcr`]
module"]
pub type PPG_PPG_TMCR = crate::Reg<ppg_ppg_tmcr::PPG_PPG_TMCR_SPEC>;
#[doc = "Timer Control Register"]
pub mod ppg_ppg_tmcr;
#[doc = "PPG_PPG_TMCR2 (rw) register accessor: Timer Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_tmcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppg_ppg_tmcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppg_ppg_tmcr2`]
module"]
pub type PPG_PPG_TMCR2 = crate::Reg<ppg_ppg_tmcr2::PPG_PPG_TMCR2_SPEC>;
#[doc = "Timer Control Register 2"]
pub mod ppg_ppg_tmcr2;
#[doc = "PPG_PPG_STC (rw) register accessor: Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_stc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppg_ppg_stc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppg_ppg_stc`]
module"]
pub type PPG_PPG_STC = crate::Reg<ppg_ppg_stc::PPG_PPG_STC_SPEC>;
#[doc = "Status Control Register"]
pub mod ppg_ppg_stc;
#[doc = "PPG_PPG_PRLL (rw) register accessor: LOW Width Reload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_prll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppg_ppg_prll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppg_ppg_prll`]
module"]
pub type PPG_PPG_PRLL = crate::Reg<ppg_ppg_prll::PPG_PPG_PRLL_SPEC>;
#[doc = "LOW Width Reload Register"]
pub mod ppg_ppg_prll;
#[doc = "PPG_PPG_PRLH (rw) register accessor: HIGH Width Reload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_prlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppg_ppg_prlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppg_ppg_prlh`]
module"]
pub type PPG_PPG_PRLH = crate::Reg<ppg_ppg_prlh::PPG_PPG_PRLH_SPEC>;
#[doc = "HIGH Width Reload Register"]
pub mod ppg_ppg_prlh;
#[doc = "PPG_PPG_TMR (r) register accessor: Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_tmr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppg_ppg_tmr`]
module"]
pub type PPG_PPG_TMR = crate::Reg<ppg_ppg_tmr::PPG_PPG_TMR_SPEC>;
#[doc = "Timer Register"]
pub mod ppg_ppg_tmr;
#[doc = "RT_RT_TMCR (rw) register accessor: Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt_rt_tmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt_rt_tmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rt_rt_tmcr`]
module"]
pub type RT_RT_TMCR = crate::Reg<rt_rt_tmcr::RT_RT_TMCR_SPEC>;
#[doc = "Timer Control Register"]
pub mod rt_rt_tmcr;
#[doc = "RT_RT_TMCR2 (rw) register accessor: Timer Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt_rt_tmcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt_rt_tmcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rt_rt_tmcr2`]
module"]
pub type RT_RT_TMCR2 = crate::Reg<rt_rt_tmcr2::RT_RT_TMCR2_SPEC>;
#[doc = "Timer Control Register 2"]
pub mod rt_rt_tmcr2;
#[doc = "RT_RT_STC (rw) register accessor: Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt_rt_stc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt_rt_stc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rt_rt_stc`]
module"]
pub type RT_RT_STC = crate::Reg<rt_rt_stc::RT_RT_STC_SPEC>;
#[doc = "Status Control Register"]
pub mod rt_rt_stc;
#[doc = "RT_RT_PCSR (rw) register accessor: PWM Cycle Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt_rt_pcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rt_rt_pcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rt_rt_pcsr`]
module"]
pub type RT_RT_PCSR = crate::Reg<rt_rt_pcsr::RT_RT_PCSR_SPEC>;
#[doc = "PWM Cycle Set Register"]
pub mod rt_rt_pcsr;
#[doc = "RT_RT_TMR (r) register accessor: Timer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rt_rt_tmr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rt_rt_tmr`]
module"]
pub type RT_RT_TMR = crate::Reg<rt_rt_tmr::RT_RT_TMR_SPEC>;
#[doc = "Timer Register"]
pub mod rt_rt_tmr;
#[doc = "PWC_PWC_TMCR (rw) register accessor: Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc_pwc_tmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc_pwc_tmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwc_pwc_tmcr`]
module"]
pub type PWC_PWC_TMCR = crate::Reg<pwc_pwc_tmcr::PWC_PWC_TMCR_SPEC>;
#[doc = "Timer Control Register"]
pub mod pwc_pwc_tmcr;
#[doc = "PWC_PWC_TMCR2 (rw) register accessor: Timer Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc_pwc_tmcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc_pwc_tmcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwc_pwc_tmcr2`]
module"]
pub type PWC_PWC_TMCR2 = crate::Reg<pwc_pwc_tmcr2::PWC_PWC_TMCR2_SPEC>;
#[doc = "Timer Control Register 2"]
pub mod pwc_pwc_tmcr2;
#[doc = "PWC_PWC_STC (rw) register accessor: Status Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc_pwc_stc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc_pwc_stc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwc_pwc_stc`]
module"]
pub type PWC_PWC_STC = crate::Reg<pwc_pwc_stc::PWC_PWC_STC_SPEC>;
#[doc = "Status Control Register"]
pub mod pwc_pwc_stc;
#[doc = "PWC_PWC_DTBF (r) register accessor: Data Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwc_pwc_dtbf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwc_pwc_dtbf`]
module"]
pub type PWC_PWC_DTBF = crate::Reg<pwc_pwc_dtbf::PWC_PWC_DTBF_SPEC>;
#[doc = "Data Buffer Register"]
pub mod pwc_pwc_dtbf;
