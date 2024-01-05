#[doc = "Register `rxmulticastframes_g` reader"]
pub type R = crate::R<RXMULTICASTFRAMES_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXMULTICASTFRAMES_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of good multicast frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmulticastframes_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMULTICASTFRAMES_G_SPEC;
impl crate::RegisterSpec for RXMULTICASTFRAMES_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmulticastframes_g::R`](R) reader structure"]
impl crate::Readable for RXMULTICASTFRAMES_G_SPEC {}
#[doc = "`reset()` method sets rxmulticastframes_g to value 0"]
impl crate::Resettable for RXMULTICASTFRAMES_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
