#[doc = "Register `rxpauseframes` reader"]
pub type R = crate::R<RXPAUSEFRAMES_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXPAUSEFRAMES_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of good and valid PAUSE frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxpauseframes::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXPAUSEFRAMES_SPEC;
impl crate::RegisterSpec for RXPAUSEFRAMES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxpauseframes::R`](R) reader structure"]
impl crate::Readable for RXPAUSEFRAMES_SPEC {}
#[doc = "`reset()` method sets rxpauseframes to value 0"]
impl crate::Resettable for RXPAUSEFRAMES_SPEC {
    const RESET_VALUE: u32 = 0;
}
