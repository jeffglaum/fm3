#[doc = "Register `TSR` reader"]
pub type R = crate::R<TSR_SPEC>;
#[doc = "Field `TSSOVF` reader - Time Stamp Seconds Overflow"]
pub type TSSOVF_R = crate::BitReader;
#[doc = "Field `TSTART` reader - Time Stamp Target Time Reached"]
pub type TSTART_R = crate::BitReader;
#[doc = "Field `ATSTS` reader - Auxiliary Time Stamp Trigger Snapshot"]
pub type ATSTS_R = crate::BitReader;
#[doc = "Field `TRGTER` reader - Timestamp Target Time Error"]
pub type TRGTER_R = crate::BitReader;
#[doc = "Field `ATSSTM` reader - Auxiliary Time Stamp Snapshot Trigger Missed"]
pub type ATSSTM_R = crate::BitReader;
#[doc = "Field `ATSNS` reader - Auxiliary Time Stamp Number of Snapshots"]
pub type ATSNS_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Time Stamp Seconds Overflow"]
    #[inline(always)]
    pub fn tssovf(&self) -> TSSOVF_R {
        TSSOVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Stamp Target Time Reached"]
    #[inline(always)]
    pub fn tstart(&self) -> TSTART_R {
        TSTART_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auxiliary Time Stamp Trigger Snapshot"]
    #[inline(always)]
    pub fn atsts(&self) -> ATSTS_R {
        ATSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp Target Time Error"]
    #[inline(always)]
    pub fn trgter(&self) -> TRGTER_R {
        TRGTER_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 24 - Auxiliary Time Stamp Snapshot Trigger Missed"]
    #[inline(always)]
    pub fn atsstm(&self) -> ATSSTM_R {
        ATSSTM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Auxiliary Time Stamp Number of Snapshots"]
    #[inline(always)]
    pub fn atsns(&self) -> ATSNS_R {
        ATSNS_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[doc = "Time Stamp Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSR_SPEC;
impl crate::RegisterSpec for TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsr::R`](R) reader structure"]
impl crate::Readable for TSR_SPEC {}
#[doc = "`reset()` method sets TSR to value 0"]
impl crate::Resettable for TSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
