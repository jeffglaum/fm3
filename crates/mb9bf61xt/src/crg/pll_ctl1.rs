#[doc = "Register `PLL_CTL1` reader"]
pub type R = crate::R<PLL_CTL1_SPEC>;
#[doc = "Register `PLL_CTL1` writer"]
pub type W = crate::W<PLL_CTL1_SPEC>;
#[doc = "Field `PLLM` reader - PLL VCO clock frequency division ratio setting bit"]
pub type PLLM_R = crate::FieldReader;
#[doc = "Field `PLLM` writer - PLL VCO clock frequency division ratio setting bit"]
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PLLK` reader - PLL input clock frequency division ratio setting bit"]
pub type PLLK_R = crate::FieldReader;
#[doc = "Field `PLLK` writer - PLL input clock frequency division ratio setting bit"]
pub type PLLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - PLL VCO clock frequency division ratio setting bit"]
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - PLL input clock frequency division ratio setting bit"]
    #[inline(always)]
    pub fn pllk(&self) -> PLLK_R {
        PLLK_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLL VCO clock frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn pllm(&mut self) -> PLLM_W<PLL_CTL1_SPEC> {
        PLLM_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PLL input clock frequency division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn pllk(&mut self) -> PLLK_W<PLL_CTL1_SPEC> {
        PLLK_W::new(self, 4)
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
#[doc = "PLL Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll_ctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_ctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL_CTL1_SPEC;
impl crate::RegisterSpec for PLL_CTL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pll_ctl1::R`](R) reader structure"]
impl crate::Readable for PLL_CTL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pll_ctl1::W`](W) writer structure"]
impl crate::Writable for PLL_CTL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PLL_CTL1 to value 0"]
impl crate::Resettable for PLL_CTL1_SPEC {
    const RESET_VALUE: u8 = 0;
}
