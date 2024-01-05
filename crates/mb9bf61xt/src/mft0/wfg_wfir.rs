#[doc = "Register `WFG_WFIR` reader"]
pub type R = crate::R<WFG_WFIR_SPEC>;
#[doc = "Register `WFG_WFIR` writer"]
pub type W = crate::W<WFG_WFIR_SPEC>;
#[doc = "Field `DTIF` reader - Indicates that DTIF interrupt has been generated."]
pub type DTIF_R = crate::BitReader;
#[doc = "Field `DTIC` writer - Clears WFIR.DTIF and deasserts the DTIF interrupt signal."]
pub type DTIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIF10` reader - Indicates that WFG10 timer interrupt has been generated."]
pub type TMIF10_R = crate::BitReader;
#[doc = "Field `TMIC10` writer - Clears WFIR.TMIF10 and deasserts the WFG10 timer interrupt signal."]
pub type TMIC10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIE10` reader - Starts the WFG10 timer"]
pub type TMIE10_R = crate::BitReader;
#[doc = "Field `TMIE10` writer - Starts the WFG10 timer"]
pub type TMIE10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIS10` writer - Stops the WFG10 timer"]
pub type TMIS10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIF32` reader - Indicates that WFG32 timer interrupt has been generated."]
pub type TMIF32_R = crate::BitReader;
#[doc = "Field `TMIC32` writer - Clears WFIR.TMIF32 and deasserts the WFG32 timer interrupt signal."]
pub type TMIC32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIE32` reader - Starts the WFG32 timer"]
pub type TMIE32_R = crate::BitReader;
#[doc = "Field `TMIE32` writer - Starts the WFG32 timer"]
pub type TMIE32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIS32` writer - Stops the WFG32 timer"]
pub type TMIS32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIF54` reader - Indicates that WFG54 timer interrupt has been generated."]
pub type TMIF54_R = crate::BitReader;
#[doc = "Field `TMIC54` writer - Clears WFIR.TMIF54 and deasserts the WFG54 timer interrupt signal."]
pub type TMIC54_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIE54` reader - Starts the WFG54 timer"]
pub type TMIE54_R = crate::BitReader;
#[doc = "Field `TMIE54` writer - Starts the WFG54 timer"]
pub type TMIE54_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIS54` writer - Stops the WFG54 timer"]
pub type TMIS54_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that DTIF interrupt has been generated."]
    #[inline(always)]
    pub fn dtif(&self) -> DTIF_R {
        DTIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that WFG10 timer interrupt has been generated."]
    #[inline(always)]
    pub fn tmif10(&self) -> TMIF10_R {
        TMIF10_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Starts the WFG10 timer"]
    #[inline(always)]
    pub fn tmie10(&self) -> TMIE10_R {
        TMIE10_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that WFG32 timer interrupt has been generated."]
    #[inline(always)]
    pub fn tmif32(&self) -> TMIF32_R {
        TMIF32_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Starts the WFG32 timer"]
    #[inline(always)]
    pub fn tmie32(&self) -> TMIE32_R {
        TMIE32_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that WFG54 timer interrupt has been generated."]
    #[inline(always)]
    pub fn tmif54(&self) -> TMIF54_R {
        TMIF54_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Starts the WFG54 timer"]
    #[inline(always)]
    pub fn tmie54(&self) -> TMIE54_R {
        TMIE54_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clears WFIR.DTIF and deasserts the DTIF interrupt signal."]
    #[inline(always)]
    #[must_use]
    pub fn dtic(&mut self) -> DTIC_W<WFG_WFIR_SPEC> {
        DTIC_W::new(self, 1)
    }
    #[doc = "Bit 5 - Clears WFIR.TMIF10 and deasserts the WFG10 timer interrupt signal."]
    #[inline(always)]
    #[must_use]
    pub fn tmic10(&mut self) -> TMIC10_W<WFG_WFIR_SPEC> {
        TMIC10_W::new(self, 5)
    }
    #[doc = "Bit 6 - Starts the WFG10 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tmie10(&mut self) -> TMIE10_W<WFG_WFIR_SPEC> {
        TMIE10_W::new(self, 6)
    }
    #[doc = "Bit 7 - Stops the WFG10 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tmis10(&mut self) -> TMIS10_W<WFG_WFIR_SPEC> {
        TMIS10_W::new(self, 7)
    }
    #[doc = "Bit 9 - Clears WFIR.TMIF32 and deasserts the WFG32 timer interrupt signal."]
    #[inline(always)]
    #[must_use]
    pub fn tmic32(&mut self) -> TMIC32_W<WFG_WFIR_SPEC> {
        TMIC32_W::new(self, 9)
    }
    #[doc = "Bit 10 - Starts the WFG32 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tmie32(&mut self) -> TMIE32_W<WFG_WFIR_SPEC> {
        TMIE32_W::new(self, 10)
    }
    #[doc = "Bit 11 - Stops the WFG32 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tmis32(&mut self) -> TMIS32_W<WFG_WFIR_SPEC> {
        TMIS32_W::new(self, 11)
    }
    #[doc = "Bit 13 - Clears WFIR.TMIF54 and deasserts the WFG54 timer interrupt signal."]
    #[inline(always)]
    #[must_use]
    pub fn tmic54(&mut self) -> TMIC54_W<WFG_WFIR_SPEC> {
        TMIC54_W::new(self, 13)
    }
    #[doc = "Bit 14 - Starts the WFG54 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tmie54(&mut self) -> TMIE54_W<WFG_WFIR_SPEC> {
        TMIE54_W::new(self, 14)
    }
    #[doc = "Bit 15 - Stops the WFG54 timer"]
    #[inline(always)]
    #[must_use]
    pub fn tmis54(&mut self) -> TMIS54_W<WFG_WFIR_SPEC> {
        TMIS54_W::new(self, 15)
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
#[doc = "WFG Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wfg_wfir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wfg_wfir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WFG_WFIR_SPEC;
impl crate::RegisterSpec for WFG_WFIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wfg_wfir::R`](R) reader structure"]
impl crate::Readable for WFG_WFIR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wfg_wfir::W`](W) writer structure"]
impl crate::Writable for WFG_WFIR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WFG_WFIR to value 0"]
impl crate::Resettable for WFG_WFIR_SPEC {
    const RESET_VALUE: u16 = 0;
}
