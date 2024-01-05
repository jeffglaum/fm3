#[doc = "Register `txdeferred` reader"]
pub type R = crate::R<TXDEFERRED_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXDEFERRED_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of successfully transmitted frames after a deferral in Half-duplex mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdeferred::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDEFERRED_SPEC;
impl crate::RegisterSpec for TXDEFERRED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdeferred::R`](R) reader structure"]
impl crate::Readable for TXDEFERRED_SPEC {}
#[doc = "`reset()` method sets txdeferred to value 0"]
impl crate::Resettable for TXDEFERRED_SPEC {
    const RESET_VALUE: u32 = 0;
}
