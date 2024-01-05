#[doc = "Register `WDG_VLR` reader"]
pub type R = crate::R<WDG_VLR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<WDG_VLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Hardware Watchdog Timer Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_vlr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDG_VLR_SPEC;
impl crate::RegisterSpec for WDG_VLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdg_vlr::R`](R) reader structure"]
impl crate::Readable for WDG_VLR_SPEC {}
#[doc = "`reset()` method sets WDG_VLR to value 0"]
impl crate::Resettable for WDG_VLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
