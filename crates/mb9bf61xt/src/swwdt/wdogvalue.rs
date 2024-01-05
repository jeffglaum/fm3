#[doc = "Register `WDOGVALUE` reader"]
pub type R = crate::R<WDOGVALUE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<WDOGVALUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Software Watchdog Timer Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogvalue::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOGVALUE_SPEC;
impl crate::RegisterSpec for WDOGVALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdogvalue::R`](R) reader structure"]
impl crate::Readable for WDOGVALUE_SPEC {}
#[doc = "`reset()` method sets WDOGVALUE to value 0xffff_ffff"]
impl crate::Resettable for WDOGVALUE_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
