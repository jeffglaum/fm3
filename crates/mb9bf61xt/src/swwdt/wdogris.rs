#[doc = "Register `WDOGRIS` reader"]
pub type R = crate::R<WDOGRIS_SPEC>;
#[doc = "Field `RIS` reader - Software watchdog interrupt status bit"]
pub type RIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software watchdog interrupt status bit"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Software Watchdog Timer Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOGRIS_SPEC;
impl crate::RegisterSpec for WDOGRIS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdogris::R`](R) reader structure"]
impl crate::Readable for WDOGRIS_SPEC {}
#[doc = "`reset()` method sets WDOGRIS to value 0"]
impl crate::Resettable for WDOGRIS_SPEC {
    const RESET_VALUE: u8 = 0;
}
