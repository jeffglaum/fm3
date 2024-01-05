#[doc = "Register `TMSP` reader"]
pub type R = crate::R<TMSP_SPEC>;
#[doc = "Field `TMSP` reader - Time Stamp bits"]
pub type TMSP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Time Stamp bits"]
    #[inline(always)]
    pub fn tmsp(&self) -> TMSP_R {
        TMSP_R::new(self.bits & 0x07ff)
    }
}
#[doc = "Time Stamp Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmsp::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMSP_SPEC;
impl crate::RegisterSpec for TMSP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tmsp::R`](R) reader structure"]
impl crate::Readable for TMSP_SPEC {}
#[doc = "`reset()` method sets TMSP to value 0"]
impl crate::Resettable for TMSP_SPEC {
    const RESET_VALUE: u16 = 0;
}
