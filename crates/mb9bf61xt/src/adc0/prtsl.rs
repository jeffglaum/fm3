#[doc = "Register `PRTSL` reader"]
pub type R = crate::R<PRTSL_SPEC>;
#[doc = "Register `PRTSL` writer"]
pub type W = crate::W<PRTSL_SPEC>;
#[doc = "Field `PRTSL` reader - Priority conversion timer trigger selection bit"]
pub type PRTSL_R = crate::FieldReader;
#[doc = "Field `PRTSL` writer - Priority conversion timer trigger selection bit"]
pub type PRTSL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Priority conversion timer trigger selection bit"]
    #[inline(always)]
    pub fn prtsl(&self) -> PRTSL_R {
        PRTSL_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Priority conversion timer trigger selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn prtsl(&mut self) -> PRTSL_W<PRTSL_SPEC> {
        PRTSL_W::new(self, 0)
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
#[doc = "Priority Conversion Timer Trigger Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prtsl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prtsl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRTSL_SPEC;
impl crate::RegisterSpec for PRTSL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prtsl::R`](R) reader structure"]
impl crate::Readable for PRTSL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prtsl::W`](W) writer structure"]
impl crate::Writable for PRTSL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PRTSL to value 0"]
impl crate::Resettable for PRTSL_SPEC {
    const RESET_VALUE: u8 = 0;
}
