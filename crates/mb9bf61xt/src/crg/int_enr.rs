#[doc = "Register `INT_ENR` reader"]
pub type R = crate::R<INT_ENR_SPEC>;
#[doc = "Register `INT_ENR` writer"]
pub type W = crate::W<INT_ENR_SPEC>;
#[doc = "Field `MCSE` reader - Main oscillation stabilization completion interrupt enable bit"]
pub type MCSE_R = crate::BitReader;
#[doc = "Field `MCSE` writer - Main oscillation stabilization completion interrupt enable bit"]
pub type MCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCSE` reader - Sub oscillation stabilization completion interrupt enable bit"]
pub type SCSE_R = crate::BitReader;
#[doc = "Field `SCSE` writer - Sub oscillation stabilization completion interrupt enable bit"]
pub type SCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSE` reader - PLL oscillation stabilization completion interrupt enable bit"]
pub type PCSE_R = crate::BitReader;
#[doc = "Field `PCSE` writer - PLL oscillation stabilization completion interrupt enable bit"]
pub type PCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCSE` reader - Anomalous frequency detection interrupt enable bit"]
pub type FCSE_R = crate::BitReader;
#[doc = "Field `FCSE` writer - Anomalous frequency detection interrupt enable bit"]
pub type FCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main oscillation stabilization completion interrupt enable bit"]
    #[inline(always)]
    pub fn mcse(&self) -> MCSE_R {
        MCSE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sub oscillation stabilization completion interrupt enable bit"]
    #[inline(always)]
    pub fn scse(&self) -> SCSE_R {
        SCSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PLL oscillation stabilization completion interrupt enable bit"]
    #[inline(always)]
    pub fn pcse(&self) -> PCSE_R {
        PCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Anomalous frequency detection interrupt enable bit"]
    #[inline(always)]
    pub fn fcse(&self) -> FCSE_R {
        FCSE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main oscillation stabilization completion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn mcse(&mut self) -> MCSE_W<INT_ENR_SPEC> {
        MCSE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Sub oscillation stabilization completion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn scse(&mut self) -> SCSE_W<INT_ENR_SPEC> {
        SCSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - PLL oscillation stabilization completion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pcse(&mut self) -> PCSE_W<INT_ENR_SPEC> {
        PCSE_W::new(self, 2)
    }
    #[doc = "Bit 5 - Anomalous frequency detection interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fcse(&mut self) -> FCSE_W<INT_ENR_SPEC> {
        FCSE_W::new(self, 5)
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
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENR_SPEC;
impl crate::RegisterSpec for INT_ENR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`int_enr::R`](R) reader structure"]
impl crate::Readable for INT_ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_enr::W`](W) writer structure"]
impl crate::Writable for INT_ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INT_ENR to value 0"]
impl crate::Resettable for INT_ENR_SPEC {
    const RESET_VALUE: u8 = 0;
}
