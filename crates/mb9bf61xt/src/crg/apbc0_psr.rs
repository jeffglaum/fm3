#[doc = "Register `APBC0_PSR` reader"]
pub type R = crate::R<APBC0_PSR_SPEC>;
#[doc = "Register `APBC0_PSR` writer"]
pub type W = crate::W<APBC0_PSR_SPEC>;
#[doc = "Field `APBC0` reader - APB0 bus clock frequency division ratio setting bit"]
pub type APBC0_R = crate::FieldReader;
#[doc = "Field `APBC0` writer - APB0 bus clock frequency division ratio setting bit"]
pub type APBC0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - APB0 bus clock frequency division ratio setting bit"]
    #[inline(always)]
    pub fn apbc0(&self) -> APBC0_R {
        APBC0_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - APB0 bus clock frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn apbc0(&mut self) -> APBC0_W<APBC0_PSR_SPEC> {
        APBC0_W::new(self, 0)
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
#[doc = "APB0 Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apbc0_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apbc0_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBC0_PSR_SPEC;
impl crate::RegisterSpec for APBC0_PSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`apbc0_psr::R`](R) reader structure"]
impl crate::Readable for APBC0_PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbc0_psr::W`](W) writer structure"]
impl crate::Writable for APBC0_PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets APBC0_PSR to value 0"]
impl crate::Resettable for APBC0_PSR_SPEC {
    const RESET_VALUE: u8 = 0;
}
