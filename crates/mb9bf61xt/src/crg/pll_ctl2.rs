#[doc = "Register `PLL_CTL2` reader"]
pub type R = crate::R<PLL_CTL2_SPEC>;
#[doc = "Register `PLL_CTL2` writer"]
pub type W = crate::W<PLL_CTL2_SPEC>;
#[doc = "Field `PLLN` reader - PLL feedback frequency division ratio setting bit"]
pub type PLLN_R = crate::FieldReader;
#[doc = "Field `PLLN` writer - PLL feedback frequency division ratio setting bit"]
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PLL feedback frequency division ratio setting bit"]
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL feedback frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn plln(&mut self) -> PLLN_W<PLL_CTL2_SPEC> {
        PLLN_W::new(self, 0)
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
#[doc = "PLL Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_CTL2_SPEC;
impl crate::RegisterSpec for PLL_CTL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pll_ctl2::R`](R) reader structure"]
impl crate::Readable for PLL_CTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_ctl2::W`](W) writer structure"]
impl crate::Writable for PLL_CTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PLL_CTL2 to value 0"]
impl crate::Resettable for PLL_CTL2_SPEC {
    const RESET_VALUE: u8 = 0;
}
