#[doc = "Register `ISR` reader"]
pub type R = crate::R<ISR_SPEC>;
#[doc = "Field `RGIS` reader - RGMII Interrupt Status"]
pub type RGIS_R = crate::BitReader;
#[doc = "Field `PIS` reader - PMT Interrupt Status"]
pub type PIS_R = crate::BitReader;
#[doc = "Field `MIS` reader - MMC Interrupt Status"]
pub type MIS_R = crate::BitReader;
#[doc = "Field `RIS` reader - MMC Receive Interrupt Status"]
pub type RIS_R = crate::BitReader;
#[doc = "Field `TIS` reader - MMC Transmit Interrupt Status"]
pub type TIS_R = crate::BitReader;
#[doc = "Field `COIS` reader - MMC Receive Checksum Offload Interrupt Status"]
pub type COIS_R = crate::BitReader;
#[doc = "Field `TSIS` reader - Time Stamp Interrupt Status"]
pub type TSIS_R = crate::BitReader;
#[doc = "Field `LPIIS` reader - LPI Interrupt Status"]
pub type LPIIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RGMII Interrupt Status"]
    #[inline(always)]
    pub fn rgis(&self) -> RGIS_R {
        RGIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - PMT Interrupt Status"]
    #[inline(always)]
    pub fn pis(&self) -> PIS_R {
        PIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MMC Interrupt Status"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MMC Receive Interrupt Status"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MMC Transmit Interrupt Status"]
    #[inline(always)]
    pub fn tis(&self) -> TIS_R {
        TIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC Receive Checksum Offload Interrupt Status"]
    #[inline(always)]
    pub fn cois(&self) -> COIS_R {
        COIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Time Stamp Interrupt Status"]
    #[inline(always)]
    pub fn tsis(&self) -> TSIS_R {
        TSIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPI Interrupt Status"]
    #[inline(always)]
    pub fn lpiis(&self) -> LPIIS_R {
        LPIIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for ISR_SPEC {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: u32 = 0;
}
