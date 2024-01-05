#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x2100],
    hcnt: HCNT,
    _reserved1: [u8; 0x02],
    hirq: HIRQ,
    herr: HERR,
    _reserved3: [u8; 0x02],
    hstate: HSTATE,
    hfcomp: HFCOMP,
    _reserved5: [u8; 0x02],
    hrtimer: HRTIMER,
    _reserved6: [u8; 0x02],
    hrtimer2: HRTIMER2,
    hadr: HADR,
    _reserved8: [u8; 0x02],
    heof: HEOF,
    _reserved9: [u8; 0x02],
    hframe: HFRAME,
    _reserved10: [u8; 0x02],
    htoken: HTOKEN,
    _reserved11: [u8; 0x03],
    udcc: UDCC,
    _reserved12: [u8; 0x02],
    ep0c: EP0C,
    _reserved13: [u8; 0x02],
    ep1c: EP1C,
    _reserved14: [u8; 0x02],
    ep2c: EP2C,
    _reserved15: [u8; 0x02],
    ep3c: EP3C,
    _reserved16: [u8; 0x02],
    ep4c: EP4C,
    _reserved17: [u8; 0x02],
    ep5c: EP5C,
    _reserved18: [u8; 0x02],
    tmsp: TMSP,
    _reserved19: [u8; 0x02],
    udcs: UDCS,
    udcie: UDCIE,
    _reserved21: [u8; 0x02],
    ep0is: EP0IS,
    _reserved22: [u8; 0x02],
    ep0os: EP0OS,
    _reserved23: [u8; 0x02],
    ep1s: EP1S,
    _reserved24: [u8; 0x02],
    ep2s: EP2S,
    _reserved25: [u8; 0x02],
    ep3s: EP3S,
    _reserved26: [u8; 0x02],
    ep4s: EP4S,
    _reserved27: [u8; 0x02],
    ep5s: EP5S,
    _reserved28: [u8; 0x02],
    ep0dt: EP0DT,
    _reserved29: [u8; 0x02],
    ep1dt: EP1DT,
    _reserved30: [u8; 0x02],
    ep2dt: EP2DT,
    _reserved31: [u8; 0x02],
    ep3dt: EP3DT,
    _reserved32: [u8; 0x02],
    ep4dt: EP4DT,
    _reserved33: [u8; 0x02],
    ep5dt: EP5DT,
}
impl RegisterBlock {
    #[doc = "0x2100 - Host Control Register"]
    #[inline(always)]
    pub const fn hcnt(&self) -> &HCNT {
        &self.hcnt
    }
    #[doc = "0x2104 - Host Interrupt Register"]
    #[inline(always)]
    pub const fn hirq(&self) -> &HIRQ {
        &self.hirq
    }
    #[doc = "0x2105 - Host Error Status Register"]
    #[inline(always)]
    pub const fn herr(&self) -> &HERR {
        &self.herr
    }
    #[doc = "0x2108 - Host Status Register"]
    #[inline(always)]
    pub const fn hstate(&self) -> &HSTATE {
        &self.hstate
    }
    #[doc = "0x2109 - SOF Interrupt Frame Compare Register"]
    #[inline(always)]
    pub const fn hfcomp(&self) -> &HFCOMP {
        &self.hfcomp
    }
    #[doc = "0x210c - Retry Timer Setup Register"]
    #[inline(always)]
    pub const fn hrtimer(&self) -> &HRTIMER {
        &self.hrtimer
    }
    #[doc = "0x2110 - Retry Timer Setup Register 2"]
    #[inline(always)]
    pub const fn hrtimer2(&self) -> &HRTIMER2 {
        &self.hrtimer2
    }
    #[doc = "0x2111 - Host Address Register"]
    #[inline(always)]
    pub const fn hadr(&self) -> &HADR {
        &self.hadr
    }
    #[doc = "0x2114 - EOF Setup Register"]
    #[inline(always)]
    pub const fn heof(&self) -> &HEOF {
        &self.heof
    }
    #[doc = "0x2118 - Frame Setup Register"]
    #[inline(always)]
    pub const fn hframe(&self) -> &HFRAME {
        &self.hframe
    }
    #[doc = "0x211c - Host Token Endpoint Register"]
    #[inline(always)]
    pub const fn htoken(&self) -> &HTOKEN {
        &self.htoken
    }
    #[doc = "0x2120 - UDC Control Register"]
    #[inline(always)]
    pub const fn udcc(&self) -> &UDCC {
        &self.udcc
    }
    #[doc = "0x2124 - EP0 Control Register"]
    #[inline(always)]
    pub const fn ep0c(&self) -> &EP0C {
        &self.ep0c
    }
    #[doc = "0x2128 - EP1 Control Register"]
    #[inline(always)]
    pub const fn ep1c(&self) -> &EP1C {
        &self.ep1c
    }
    #[doc = "0x212c - EP2 Control Register"]
    #[inline(always)]
    pub const fn ep2c(&self) -> &EP2C {
        &self.ep2c
    }
    #[doc = "0x2130 - EP3 Control Register"]
    #[inline(always)]
    pub const fn ep3c(&self) -> &EP3C {
        &self.ep3c
    }
    #[doc = "0x2134 - EP4 Control Register"]
    #[inline(always)]
    pub const fn ep4c(&self) -> &EP4C {
        &self.ep4c
    }
    #[doc = "0x2138 - EP5 Control Register"]
    #[inline(always)]
    pub const fn ep5c(&self) -> &EP5C {
        &self.ep5c
    }
    #[doc = "0x213c - Time Stamp Register"]
    #[inline(always)]
    pub const fn tmsp(&self) -> &TMSP {
        &self.tmsp
    }
    #[doc = "0x2140 - UDC Status Register"]
    #[inline(always)]
    pub const fn udcs(&self) -> &UDCS {
        &self.udcs
    }
    #[doc = "0x2141 - UDC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn udcie(&self) -> &UDCIE {
        &self.udcie
    }
    #[doc = "0x2144 - EP0I Status Register"]
    #[inline(always)]
    pub const fn ep0is(&self) -> &EP0IS {
        &self.ep0is
    }
    #[doc = "0x2148 - EP0O Status Register"]
    #[inline(always)]
    pub const fn ep0os(&self) -> &EP0OS {
        &self.ep0os
    }
    #[doc = "0x214c - EP1 Status Register"]
    #[inline(always)]
    pub const fn ep1s(&self) -> &EP1S {
        &self.ep1s
    }
    #[doc = "0x2150 - EP2 Status Register"]
    #[inline(always)]
    pub const fn ep2s(&self) -> &EP2S {
        &self.ep2s
    }
    #[doc = "0x2154 - EP3 Status Register"]
    #[inline(always)]
    pub const fn ep3s(&self) -> &EP3S {
        &self.ep3s
    }
    #[doc = "0x2158 - EP4 Status Register"]
    #[inline(always)]
    pub const fn ep4s(&self) -> &EP4S {
        &self.ep4s
    }
    #[doc = "0x215c - EP5 Status Register"]
    #[inline(always)]
    pub const fn ep5s(&self) -> &EP5S {
        &self.ep5s
    }
    #[doc = "0x2160 - EP0 Data Register"]
    #[inline(always)]
    pub const fn ep0dt(&self) -> &EP0DT {
        &self.ep0dt
    }
    #[doc = "0x2164 - EP1 Data Register"]
    #[inline(always)]
    pub const fn ep1dt(&self) -> &EP1DT {
        &self.ep1dt
    }
    #[doc = "0x2168 - EP2 Data Register"]
    #[inline(always)]
    pub const fn ep2dt(&self) -> &EP2DT {
        &self.ep2dt
    }
    #[doc = "0x216c - EP3 Data Register"]
    #[inline(always)]
    pub const fn ep3dt(&self) -> &EP3DT {
        &self.ep3dt
    }
    #[doc = "0x2170 - EP4 Data Register"]
    #[inline(always)]
    pub const fn ep4dt(&self) -> &EP4DT {
        &self.ep4dt
    }
    #[doc = "0x2174 - EP5 Data Register"]
    #[inline(always)]
    pub const fn ep5dt(&self) -> &EP5DT {
        &self.ep5dt
    }
}
#[doc = "UDCC (rw) register accessor: UDC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udcc`]
module"]
pub type UDCC = crate::Reg<udcc::UDCC_SPEC>;
#[doc = "UDC Control Register"]
pub mod udcc;
#[doc = "EP0C (rw) register accessor: EP0 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0c`]
module"]
pub type EP0C = crate::Reg<ep0c::EP0C_SPEC>;
#[doc = "EP0 Control Register"]
pub mod ep0c;
#[doc = "EP1C (rw) register accessor: EP1 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1c`]
module"]
pub type EP1C = crate::Reg<ep1c::EP1C_SPEC>;
#[doc = "EP1 Control Register"]
pub mod ep1c;
#[doc = "EP2C (rw) register accessor: EP2 Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2c`]
module"]
pub type EP2C = crate::Reg<ep2c::EP2C_SPEC>;
#[doc = "EP2 Control Register"]
pub mod ep2c;
pub use ep2c as ep3c;
pub use ep2c as ep4c;
pub use ep2c as ep5c;
pub use EP2C as EP3C;
pub use EP2C as EP4C;
pub use EP2C as EP5C;
#[doc = "TMSP (r) register accessor: Time Stamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmsp::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmsp`]
module"]
pub type TMSP = crate::Reg<tmsp::TMSP_SPEC>;
#[doc = "Time Stamp Register"]
pub mod tmsp;
#[doc = "UDCS (rw) register accessor: UDC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udcs`]
module"]
pub type UDCS = crate::Reg<udcs::UDCS_SPEC>;
#[doc = "UDC Status Register"]
pub mod udcs;
#[doc = "UDCIE (rw) register accessor: UDC Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udcie`]
module"]
pub type UDCIE = crate::Reg<udcie::UDCIE_SPEC>;
#[doc = "UDC Interrupt Enable Register"]
pub mod udcie;
#[doc = "EP0IS (rw) register accessor: EP0I Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0is`]
module"]
pub type EP0IS = crate::Reg<ep0is::EP0IS_SPEC>;
#[doc = "EP0I Status Register"]
pub mod ep0is;
#[doc = "EP0OS (rw) register accessor: EP0O Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0os::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0os::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0os`]
module"]
pub type EP0OS = crate::Reg<ep0os::EP0OS_SPEC>;
#[doc = "EP0O Status Register"]
pub mod ep0os;
#[doc = "EP1S (rw) register accessor: EP1 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep1s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep1s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep1s`]
module"]
pub type EP1S = crate::Reg<ep1s::EP1S_SPEC>;
#[doc = "EP1 Status Register"]
pub mod ep1s;
#[doc = "EP2S (rw) register accessor: EP2 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep2s`]
module"]
pub type EP2S = crate::Reg<ep2s::EP2S_SPEC>;
#[doc = "EP2 Status Register"]
pub mod ep2s;
pub use ep2s as ep3s;
pub use ep2s as ep4s;
pub use ep2s as ep5s;
pub use EP2S as EP3S;
pub use EP2S as EP4S;
pub use EP2S as EP5S;
#[doc = "EP0DT (rw) register accessor: EP0 Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ep0dt`]
module"]
pub type EP0DT = crate::Reg<ep0dt::EP0DT_SPEC>;
#[doc = "EP0 Data Register"]
pub mod ep0dt;
pub use ep0dt as ep1dt;
pub use ep0dt as ep2dt;
pub use ep0dt as ep3dt;
pub use ep0dt as ep4dt;
pub use ep0dt as ep5dt;
pub use EP0DT as EP1DT;
pub use EP0DT as EP2DT;
pub use EP0DT as EP3DT;
pub use EP0DT as EP4DT;
pub use EP0DT as EP5DT;
#[doc = "HCNT (rw) register accessor: Host Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcnt`]
module"]
pub type HCNT = crate::Reg<hcnt::HCNT_SPEC>;
#[doc = "Host Control Register"]
pub mod hcnt;
#[doc = "HIRQ (rw) register accessor: Host Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hirq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hirq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hirq`]
module"]
pub type HIRQ = crate::Reg<hirq::HIRQ_SPEC>;
#[doc = "Host Interrupt Register"]
pub mod hirq;
#[doc = "HERR (rw) register accessor: Host Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`herr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`herr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@herr`]
module"]
pub type HERR = crate::Reg<herr::HERR_SPEC>;
#[doc = "Host Error Status Register"]
pub mod herr;
#[doc = "HSTATE (rw) register accessor: Host Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstate`]
module"]
pub type HSTATE = crate::Reg<hstate::HSTATE_SPEC>;
#[doc = "Host Status Register"]
pub mod hstate;
#[doc = "HFCOMP (rw) register accessor: SOF Interrupt Frame Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfcomp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfcomp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcomp`]
module"]
pub type HFCOMP = crate::Reg<hfcomp::HFCOMP_SPEC>;
#[doc = "SOF Interrupt Frame Compare Register"]
pub mod hfcomp;
#[doc = "HRTIMER (rw) register accessor: Retry Timer Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrtimer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrtimer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrtimer`]
module"]
pub type HRTIMER = crate::Reg<hrtimer::HRTIMER_SPEC>;
#[doc = "Retry Timer Setup Register"]
pub mod hrtimer;
#[doc = "HRTIMER2 (rw) register accessor: Retry Timer Setup Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrtimer2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrtimer2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hrtimer2`]
module"]
pub type HRTIMER2 = crate::Reg<hrtimer2::HRTIMER2_SPEC>;
#[doc = "Retry Timer Setup Register 2"]
pub mod hrtimer2;
#[doc = "HADR (rw) register accessor: Host Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hadr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hadr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hadr`]
module"]
pub type HADR = crate::Reg<hadr::HADR_SPEC>;
#[doc = "Host Address Register"]
pub mod hadr;
#[doc = "HEOF (rw) register accessor: EOF Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`heof::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`heof::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@heof`]
module"]
pub type HEOF = crate::Reg<heof::HEOF_SPEC>;
#[doc = "EOF Setup Register"]
pub mod heof;
#[doc = "HFRAME (rw) register accessor: Frame Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hframe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hframe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hframe`]
module"]
pub type HFRAME = crate::Reg<hframe::HFRAME_SPEC>;
#[doc = "Frame Setup Register"]
pub mod hframe;
#[doc = "HTOKEN (rw) register accessor: Host Token Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htoken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htoken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htoken`]
module"]
pub type HTOKEN = crate::Reg<htoken::HTOKEN_SPEC>;
#[doc = "Host Token Endpoint Register"]
pub mod htoken;
