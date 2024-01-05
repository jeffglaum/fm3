#[doc = "Register `TTC_PSR` reader"]
pub type R = crate::R<TTC_PSR_SPEC>;
#[doc = "Register `TTC_PSR` writer"]
pub type W = crate::W<TTC_PSR_SPEC>;
#[doc = "Field `TTC` reader - Trace clock frequency division ratio setting bit"]
pub type TTC_R = crate::FieldReader;
#[doc = "Field `TTC` writer - Trace clock frequency division ratio setting bit"]
pub type TTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Trace clock frequency division ratio setting bit"]
    #[inline(always)]
    pub fn ttc(&self) -> TTC_R {
        TTC_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trace clock frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TTC_W<TTC_PSR_SPEC> {
        TTC_W::new(self, 0)
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
#[doc = "Trace Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttc_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttc_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TTC_PSR_SPEC;
impl crate::RegisterSpec for TTC_PSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ttc_psr::R`](R) reader structure"]
impl crate::Readable for TTC_PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ttc_psr::W`](W) writer structure"]
impl crate::Writable for TTC_PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TTC_PSR to value 0"]
impl crate::Resettable for TTC_PSR_SPEC {
    const RESET_VALUE: u8 = 0;
}
