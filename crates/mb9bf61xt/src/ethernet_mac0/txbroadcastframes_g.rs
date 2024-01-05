#[doc = "Register `txbroadcastframes_g` reader"]
pub type R = crate::R<TXBROADCASTFRAMES_G_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXBROADCASTFRAMES_G_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of good broadcast frames transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txbroadcastframes_g::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXBROADCASTFRAMES_G_SPEC;
impl crate::RegisterSpec for TXBROADCASTFRAMES_G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbroadcastframes_g::R`](R) reader structure"]
impl crate::Readable for TXBROADCASTFRAMES_G_SPEC {}
#[doc = "`reset()` method sets txbroadcastframes_g to value 0"]
impl crate::Resettable for TXBROADCASTFRAMES_G_SPEC {
    const RESET_VALUE: u32 = 0;
}
