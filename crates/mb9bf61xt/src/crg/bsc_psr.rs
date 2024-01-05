#[doc = "Register `BSC_PSR` reader"]
pub type R = crate::R<BSC_PSR_SPEC>;
#[doc = "Register `BSC_PSR` writer"]
pub type W = crate::W<BSC_PSR_SPEC>;
#[doc = "Field `BSR` reader - Base clock frequency division ratio setting bit"]
pub type BSR_R = crate::FieldReader;
#[doc = "Field `BSR` writer - Base clock frequency division ratio setting bit"]
pub type BSR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Base clock frequency division ratio setting bit"]
    #[inline(always)]
    pub fn bsr(&self) -> BSR_R {
        BSR_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Base clock frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn bsr(&mut self) -> BSR_W<BSC_PSR_SPEC> {
        BSR_W::new(self, 0)
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
#[doc = "Base Clock Prescaler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bsc_psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bsc_psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BSC_PSR_SPEC;
impl crate::RegisterSpec for BSC_PSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bsc_psr::R`](R) reader structure"]
impl crate::Readable for BSC_PSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bsc_psr::W`](W) writer structure"]
impl crate::Writable for BSC_PSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BSC_PSR to value 0"]
impl crate::Resettable for BSC_PSR_SPEC {
    const RESET_VALUE: u8 = 0;
}
