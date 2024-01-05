#[doc = "Register `rxunicastframes_g` reader"]
pub type R = crate::R<RXUNICASTFRAMES_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXUNICASTFRAMES_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of good unicast frames received.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxunicastframes_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXUNICASTFRAMES_G_SPEC;
impl crate::RegisterSpec for RXUNICASTFRAMES_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxunicastframes_g::R`](R) reader structure"]
impl crate::Readable for RXUNICASTFRAMES_G_SPEC {}
#[doc = "`reset()` method sets rxunicastframes_g to value 0"]
impl crate::Resettable for RXUNICASTFRAMES_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
