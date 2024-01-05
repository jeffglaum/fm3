#[doc = "Register `rxlengtherror` reader"]
pub type R = crate::R<RXLENGTHERROR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXLENGTHERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\"Number of frames received with length error (Length type field is not the frame size), for all frames with valid length field. \"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxlengtherror::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXLENGTHERROR_SPEC;
impl crate::RegisterSpec for RXLENGTHERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxlengtherror::R`](R) reader structure"]
impl crate::Readable for RXLENGTHERROR_SPEC {}
#[doc = "`reset()` method sets rxlengtherror to value 0"]
impl crate::Resettable for RXLENGTHERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
