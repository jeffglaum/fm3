#[doc = "Register `CMPD` reader"]
pub type R = crate::R<CMPD_SPEC>;
#[doc = "Register `CMPD` writer"]
pub type W = crate::W<CMPD_SPEC>;
#[doc = "Field `CMAD` reader - A/D conversion result value setting bits"]
pub type CMAD_R = crate::FieldReader<u16>;
#[doc = "Field `CMAD` writer - A/D conversion result value setting bits"]
pub type CMAD_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 6:15 - A/D conversion result value setting bits"]
    #[inline(always)]
    pub fn cmad(&self) -> CMAD_R {
        CMAD_R::new((self.bits >> 6) & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 6:15 - A/D conversion result value setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn cmad(&mut self) -> CMAD_W<CMPD_SPEC> {
        CMAD_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "A/D Comparison Value Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPD_SPEC;
impl crate::RegisterSpec for CMPD_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cmpd::R`](R) reader structure"]
impl crate::Readable for CMPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpd::W`](W) writer structure"]
impl crate::Writable for CMPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CMPD to value 0"]
impl crate::Resettable for CMPD_SPEC {
    const RESET_VALUE: u16 = 0;
}
