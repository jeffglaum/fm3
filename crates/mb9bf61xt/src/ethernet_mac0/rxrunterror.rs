#[doc = "Register `rxrunterror` reader"]
pub type R = crate::R<RXRUNTERROR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXRUNTERROR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames received with runt (64 bytes and CRC error) error.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxrunterror::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXRUNTERROR_SPEC;
impl crate::RegisterSpec for RXRUNTERROR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxrunterror::R`](R) reader structure"]
impl crate::Readable for RXRUNTERROR_SPEC {}
#[doc = "`reset()` method sets rxrunterror to value 0"]
impl crate::Resettable for RXRUNTERROR_SPEC {
    const RESET_VALUE: u32 = 0;
}
