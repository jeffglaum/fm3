#[doc = "Register `rxoversize_g` reader"]
pub type R = crate::R<RXOVERSIZE_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXOVERSIZE_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames received with length greater than the maxsize without error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxoversize_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXOVERSIZE_G_SPEC;
impl crate::RegisterSpec for RXOVERSIZE_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoversize_g::R`](R) reader structure"]
impl crate::Readable for RXOVERSIZE_G_SPEC {}
#[doc = "`reset()` method sets rxoversize_g to value 0"]
impl crate::Resettable for RXOVERSIZE_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
