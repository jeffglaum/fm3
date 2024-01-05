#[doc = "Register `WFG_WFSA10` reader"]
pub type R = crate::R<WFG_WFSA10_SPEC>;
#[doc = "Register `WFG_WFSA10` writer"]
pub type W = crate::W<WFG_WFSA10_SPEC>;
#[doc = "Field `DCK` reader - clock cycle of the WFG timer"]
pub type DCK_R = crate::FieldReader;
#[doc = "Field `DCK` writer - clock cycle of the WFG timer"]
pub type DCK_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TMD` reader - WFG's operation mode"]
pub type TMD_R = crate::FieldReader;
#[doc = "Field `TMD` writer - WFG's operation mode"]
pub type TMD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GTEN` reader - the CH_GATE signal for each channel of WFG"]
pub type GTEN_R = crate::FieldReader;
#[doc = "Field `GTEN` writer - the CH_GATE signal for each channel of WFG"]
pub type GTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSEL` reader - the PPG timer unit to be used at each channel of WFG"]
pub type PSEL_R = crate::FieldReader;
#[doc = "Field `PSEL` writer - the PPG timer unit to be used at each channel of WFG"]
pub type PSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGEN` reader - specifies how to reflect the CH_PPG signal that is input to each channel of WFG on WFG output"]
pub type PGEN_R = crate::FieldReader;
#[doc = "Field `PGEN` writer - specifies how to reflect the CH_PPG signal that is input to each channel of WFG on WFG output"]
pub type PGEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMOD` reader - specifies which polarity will be used to output the non-overlap signal"]
pub type DMOD_R = crate::BitReader;
#[doc = "Field `DMOD` writer - specifies which polarity will be used to output the non-overlap signal"]
pub type DMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - clock cycle of the WFG timer"]
    #[inline(always)]
    pub fn dck(&self) -> DCK_R {
        DCK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - WFG's operation mode"]
    #[inline(always)]
    pub fn tmd(&self) -> TMD_R {
        TMD_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - the CH_GATE signal for each channel of WFG"]
    #[inline(always)]
    pub fn gten(&self) -> GTEN_R {
        GTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - the PPG timer unit to be used at each channel of WFG"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - specifies how to reflect the CH_PPG signal that is input to each channel of WFG on WFG output"]
    #[inline(always)]
    pub fn pgen(&self) -> PGEN_R {
        PGEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - specifies which polarity will be used to output the non-overlap signal"]
    #[inline(always)]
    pub fn dmod(&self) -> DMOD_R {
        DMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - clock cycle of the WFG timer"]
    #[inline(always)]
    #[must_use]
    pub fn dck(&mut self) -> DCK_W<WFG_WFSA10_SPEC> {
        DCK_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - WFG's operation mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmd(&mut self) -> TMD_W<WFG_WFSA10_SPEC> {
        TMD_W::new(self, 3)
    }
    #[doc = "Bits 6:7 - the CH_GATE signal for each channel of WFG"]
    #[inline(always)]
    #[must_use]
    pub fn gten(&mut self) -> GTEN_W<WFG_WFSA10_SPEC> {
        GTEN_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - the PPG timer unit to be used at each channel of WFG"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<WFG_WFSA10_SPEC> {
        PSEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - specifies how to reflect the CH_PPG signal that is input to each channel of WFG on WFG output"]
    #[inline(always)]
    #[must_use]
    pub fn pgen(&mut self) -> PGEN_W<WFG_WFSA10_SPEC> {
        PGEN_W::new(self, 10)
    }
    #[doc = "Bit 12 - specifies which polarity will be used to output the non-overlap signal"]
    #[inline(always)]
    #[must_use]
    pub fn dmod(&mut self) -> DMOD_W<WFG_WFSA10_SPEC> {
        DMOD_W::new(self, 12)
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
#[doc = "WFG ch.10 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wfg_wfsa10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wfg_wfsa10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WFG_WFSA10_SPEC;
impl crate::RegisterSpec for WFG_WFSA10_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wfg_wfsa10::R`](R) reader structure"]
impl crate::Readable for WFG_WFSA10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wfg_wfsa10::W`](W) writer structure"]
impl crate::Writable for WFG_WFSA10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WFG_WFSA10 to value 0"]
impl crate::Resettable for WFG_WFSA10_SPEC {
    const RESET_VALUE: u16 = 0;
}
