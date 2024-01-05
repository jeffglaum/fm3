#[doc = "Register `HSTATE` reader"]
pub type R = crate::R<HSTATE_SPEC>;
#[doc = "Register `HSTATE` writer"]
pub type W = crate::W<HSTATE_SPEC>;
#[doc = "Field `CSTAT` reader - connection status flag"]
pub type CSTAT_R = crate::BitReader;
#[doc = "Field `TMODE` reader - transmission mode flag"]
pub type TMODE_R = crate::BitReader;
#[doc = "Field `SUSP` reader - suspend setting bit"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - suspend setting bit"]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFBUSY` reader - SOF busy flag"]
pub type SOFBUSY_R = crate::BitReader;
#[doc = "Field `SOFBUSY` writer - SOF busy flag"]
pub type SOFBUSY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKSEL` reader - USB operation clock selection bit"]
pub type CLKSEL_R = crate::BitReader;
#[doc = "Field `CLKSEL` writer - USB operation clock selection bit"]
pub type CLKSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIVE` reader - specify the keep-alive function in the low-speed mode"]
pub type ALIVE_R = crate::BitReader;
#[doc = "Field `ALIVE` writer - specify the keep-alive function in the low-speed mode"]
pub type ALIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - connection status flag"]
    #[inline(always)]
    pub fn cstat(&self) -> CSTAT_R {
        CSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transmission mode flag"]
    #[inline(always)]
    pub fn tmode(&self) -> TMODE_R {
        TMODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - suspend setting bit"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF busy flag"]
    #[inline(always)]
    pub fn sofbusy(&self) -> SOFBUSY_R {
        SOFBUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB operation clock selection bit"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - specify the keep-alive function in the low-speed mode"]
    #[inline(always)]
    pub fn alive(&self) -> ALIVE_R {
        ALIVE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - suspend setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<HSTATE_SPEC> {
        SUSP_W::new(self, 2)
    }
    #[doc = "Bit 3 - SOF busy flag"]
    #[inline(always)]
    #[must_use]
    pub fn sofbusy(&mut self) -> SOFBUSY_W<HSTATE_SPEC> {
        SOFBUSY_W::new(self, 3)
    }
    #[doc = "Bit 4 - USB operation clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<HSTATE_SPEC> {
        CLKSEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - specify the keep-alive function in the low-speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn alive(&mut self) -> ALIVE_W<HSTATE_SPEC> {
        ALIVE_W::new(self, 5)
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
#[doc = "Host Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTATE_SPEC;
impl crate::RegisterSpec for HSTATE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hstate::R`](R) reader structure"]
impl crate::Readable for HSTATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hstate::W`](W) writer structure"]
impl crate::Writable for HSTATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HSTATE to value 0x12"]
impl crate::Resettable for HSTATE_SPEC {
    const RESET_VALUE: u8 = 0x12;
}
