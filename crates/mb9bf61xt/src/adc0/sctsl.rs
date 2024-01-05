#[doc = "Register `SCTSL` reader"]
pub type R = crate::R<SCTSL_SPEC>;
#[doc = "Register `SCTSL` writer"]
pub type W = crate::W<SCTSL_SPEC>;
#[doc = "Field `SCTSL` reader - Scan conversion timer trigger selection bit"]
pub type SCTSL_R = crate::FieldReader;
#[doc = "Field `SCTSL` writer - Scan conversion timer trigger selection bit"]
pub type SCTSL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Scan conversion timer trigger selection bit"]
    #[inline(always)]
    pub fn sctsl(&self) -> SCTSL_R {
        SCTSL_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Scan conversion timer trigger selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn sctsl(&mut self) -> SCTSL_W<SCTSL_SPEC> {
        SCTSL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Scan Conversion Timer Trigger Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sctsl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sctsl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCTSL_SPEC;
impl crate::RegisterSpec for SCTSL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sctsl::R`](R) reader structure"]
impl crate::Readable for SCTSL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sctsl::W`](W) writer structure"]
impl crate::Writable for SCTSL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCTSL to value 0"]
impl crate::Resettable for SCTSL_SPEC {
    const RESET_VALUE: u8 = 0;
}
