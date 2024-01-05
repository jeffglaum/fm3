#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `MCSC` writer - Main oscillation stabilization completion interrupt cause clear bit"]
pub type MCSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCSC` writer - Sub oscillation stabilization completion interrupt cause clear bit"]
pub type SCSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSC` writer - PLL oscillation stabilization completion interrupt cause clear bit"]
pub type PCSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSC` writer - Anomalous frequency detection interrupt cause clear bit"]
pub type FCSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Main oscillation stabilization completion interrupt cause clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn mcsc(&mut self) -> MCSC_W<INT_CLR_SPEC> {
        MCSC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sub oscillation stabilization completion interrupt cause clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn scsc(&mut self) -> SCSC_W<INT_CLR_SPEC> {
        SCSC_W::new(self, 1)
    }
    #[doc = "Bit 2 - PLL oscillation stabilization completion interrupt cause clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcsc(&mut self) -> PCSC_W<INT_CLR_SPEC> {
        PCSC_W::new(self, 2)
    }
    #[doc = "Bit 5 - Anomalous frequency detection interrupt cause clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn fcsc(&mut self) -> FCSC_W<INT_CLR_SPEC> {
        FCSC_W::new(self, 5)
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
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u8 = 0;
}
