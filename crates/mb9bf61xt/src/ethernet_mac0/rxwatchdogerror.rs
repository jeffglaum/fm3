#[doc = "Register `rxwatchdogerror` reader"]
pub type R = crate::R<RXWATCHDOGERROR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXWATCHDOGERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames received with error due to watchdog timeout error (frames with a data load larger than 2048 bytes).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxwatchdogerror::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXWATCHDOGERROR_SPEC;
impl crate::RegisterSpec for RXWATCHDOGERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxwatchdogerror::R`](R) reader structure"]
impl crate::Readable for RXWATCHDOGERROR_SPEC {}
#[doc = "`reset()` method sets rxwatchdogerror to value 0"]
impl crate::Resettable for RXWATCHDOGERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
