#[doc = "Register `TIMER1VALUE` reader"]
pub type R = crate::R<TIMER1VALUE_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIMER1VALUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1value::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1VALUE_SPEC;
impl crate::RegisterSpec for TIMER1VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1value::R`](R) reader structure"]
impl crate::Readable for TIMER1VALUE_SPEC {}
#[doc = "`reset()` method sets TIMER1VALUE to value 0xffff_ffff"]
impl crate::Resettable for TIMER1VALUE_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
