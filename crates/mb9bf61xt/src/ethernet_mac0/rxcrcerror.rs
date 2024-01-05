#[doc = "Register `rxcrcerror` reader"]
pub type R = crate::R<RXCRCERROR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXCRCERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames received with CRC error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrcerror::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXCRCERROR_SPEC;
impl crate::RegisterSpec for RXCRCERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrcerror::R`](R) reader structure"]
impl crate::Readable for RXCRCERROR_SPEC {}
#[doc = "`reset()` method sets rxcrcerror to value 0"]
impl crate::Resettable for RXCRCERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
