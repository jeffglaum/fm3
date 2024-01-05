#[doc = "Register `rxoctetcount_g` reader"]
pub type R = crate::R<RXOCTETCOUNT_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXOCTETCOUNT_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of bytes received, exclusive of preamble, only in good frames. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxoctetcount_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXOCTETCOUNT_G_SPEC;
impl crate::RegisterSpec for RXOCTETCOUNT_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoctetcount_g::R`](R) reader structure"]
impl crate::Readable for RXOCTETCOUNT_G_SPEC {}
#[doc = "`reset()` method sets rxoctetcount_g to value 0"]
impl crate::Resettable for RXOCTETCOUNT_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
