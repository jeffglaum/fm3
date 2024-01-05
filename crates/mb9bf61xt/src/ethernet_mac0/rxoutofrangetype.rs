#[doc = "Register `rxoutofrangetype` reader"]
pub type R = crate::R<RXOUTOFRANGETYPE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RXOUTOFRANGETYPE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Number of frames received with length/type field not equal to the valid frame size (>1500)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxoutofrangetype::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXOUTOFRANGETYPE_SPEC;
impl crate::RegisterSpec for RXOUTOFRANGETYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxoutofrangetype::R`](R) reader structure"]
impl crate::Readable for RXOUTOFRANGETYPE_SPEC {}
#[doc = "`reset()` method sets rxoutofrangetype to value 0"]
impl crate::Resettable for RXOUTOFRANGETYPE_SPEC {
    const RESET_VALUE: u32 = 0;
}
