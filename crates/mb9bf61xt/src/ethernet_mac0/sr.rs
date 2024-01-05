#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TI_R = crate::BitReader;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TPS_R = crate::BitReader;
#[doc = "Field `TU` reader - Transmit Buffer Unavailable"]
pub type TU_R = crate::BitReader;
#[doc = "Field `TJT` reader - Transmit Jabber Timeout"]
pub type TJT_R = crate::BitReader;
#[doc = "Field `OVF` reader - Receive Overflow"]
pub type OVF_R = crate::BitReader;
#[doc = "Field `UNF` reader - Transmit underflow"]
pub type UNF_R = crate::BitReader;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RI_R = crate::BitReader;
#[doc = "Field `RU` reader - Receive Buffer Unavailable"]
pub type RU_R = crate::BitReader;
#[doc = "Field `RPS` reader - Receive process Stopped"]
pub type RPS_R = crate::BitReader;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RWT_R = crate::BitReader;
#[doc = "Field `ETI` reader - Early Transmit Interrupt"]
pub type ETI_R = crate::BitReader;
#[doc = "Field `FBI` reader - Fatal Bus Error Interrupt"]
pub type FBI_R = crate::BitReader;
#[doc = "Field `ERI` reader - Early Receive Interrupt"]
pub type ERI_R = crate::BitReader;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AIS_R = crate::BitReader;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NIS_R = crate::BitReader;
#[doc = "Field `RS` reader - Receive Process State"]
pub type RS_R = crate::FieldReader;
#[doc = "Field `TS` reader - Transmit Process State"]
pub type TS_R = crate::FieldReader;
#[doc = "Field `EB` reader - Error Bits"]
pub type EB_R = crate::FieldReader;
#[doc = "Field `GLI` reader - GMAC Line interface Interrupt"]
pub type GLI_R = crate::BitReader;
#[doc = "Field `GMI` reader - GMAC MMC Interrupt"]
pub type GMI_R = crate::BitReader;
#[doc = "Field `GPI` reader - GMAC PMT Interrupt"]
pub type GPI_R = crate::BitReader;
#[doc = "Field `TTI` reader - Time-Stamp Trigger Interrupt"]
pub type TTI_R = crate::BitReader;
#[doc = "Field `GLPII` reader - GMAC LPI Interrupt"]
pub type GLPII_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tu(&self) -> TU_R {
        TU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Jabber Timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TJT_R {
        TJT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UNF_R {
        UNF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RWT_R {
        RWT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> ETI_R {
        ETI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal Bus Error Interrupt"]
    #[inline(always)]
    pub fn fbi(&self) -> FBI_R {
        FBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> ERI_R {
        ERI_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive Process State"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit Process State"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error Bits"]
    #[inline(always)]
    pub fn eb(&self) -> EB_R {
        EB_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - GMAC Line interface Interrupt"]
    #[inline(always)]
    pub fn gli(&self) -> GLI_R {
        GLI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GMAC MMC Interrupt"]
    #[inline(always)]
    pub fn gmi(&self) -> GMI_R {
        GMI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GMAC PMT Interrupt"]
    #[inline(always)]
    pub fn gpi(&self) -> GPI_R {
        GPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Time-Stamp Trigger Interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TTI_R {
        TTI_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GMAC LPI Interrupt"]
    #[inline(always)]
    pub fn glpii(&self) -> GLPII_R {
        GLPII_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0;
}
