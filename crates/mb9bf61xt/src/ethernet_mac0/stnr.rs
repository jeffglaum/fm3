#[doc = "Register `STNR` reader"]
pub type R = crate::R<STNR_SPEC>;
#[doc = "Field `TSSS` reader - Time Stamp Sub-Seconds"]
pub type TSSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Time Stamp Sub-Seconds"]
    #[inline(always)]
    pub fn tsss(&self) -> TSSS_R {
        TSSS_R::new(self.bits & 0x7fff_ffff)
    }
}
#[doc = "System Time - Nanoseconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stnr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STNR_SPEC;
impl crate::RegisterSpec for STNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stnr::R`](R) reader structure"]
impl crate::Readable for STNR_SPEC {}
#[doc = "`reset()` method sets STNR to value 0"]
impl crate::Resettable for STNR_SPEC {
    const RESET_VALUE: u32 = 0;
}
