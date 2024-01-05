#[doc = "Register `rxjabbererror` reader"]
pub type R = crate::R<RXJABBERERROR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXJABBERERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames received with length greater than 1518 bytes with CRC error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxjabbererror::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXJABBERERROR_SPEC;
impl crate::RegisterSpec for RXJABBERERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxjabbererror::R`](R) reader structure"]
impl crate::Readable for RXJABBERERROR_SPEC {}
#[doc = "`reset()` method sets rxjabbererror to value 0"]
impl crate::Resettable for RXJABBERERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
