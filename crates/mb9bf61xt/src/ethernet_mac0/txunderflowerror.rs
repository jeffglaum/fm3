#[doc = "Register `txunderflowerror` reader"]
pub type R = crate::R<TXUNDERFLOWERROR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXUNDERFLOWERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames aborted due to frame underflow error\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txunderflowerror::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXUNDERFLOWERROR_SPEC;
impl crate::RegisterSpec for TXUNDERFLOWERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txunderflowerror::R`](R) reader structure"]
impl crate::Readable for TXUNDERFLOWERROR_SPEC {}
#[doc = "`reset()` method sets txunderflowerror to value 0"]
impl crate::Resettable for TXUNDERFLOWERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
