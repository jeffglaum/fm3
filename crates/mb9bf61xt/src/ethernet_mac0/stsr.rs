#[doc = "Register `STSR` reader"]
pub type R = crate::R<STSR_SPEC>;
#[doc = "Field `TSS` reader - Time Stamp Second"]
pub type TSS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Time Stamp Second"]
    #[inline(always)]
    pub fn tss(&self) -> TSS_R {
        TSS_R::new(self.bits)
    }
}
#[doc = "System Time - Seconds Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STSR_SPEC;
impl crate::RegisterSpec for STSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stsr::R`](R) reader structure"]
impl crate::Readable for STSR_SPEC {}
#[doc = "`reset()` method sets STSR to value 0"]
impl crate::Resettable for STSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
