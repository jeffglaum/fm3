#[doc = "Register `WDG_CTL` reader"]
pub type R = crate::R<WDG_CTL_SPEC>;
#[doc = "Register `WDG_CTL` writer"]
pub type W = crate::W<WDG_CTL_SPEC>;
#[doc = "Field `INTEN` reader - Hardware watchdog interrupt and counter enable bit"]
pub type INTEN_R = crate::BitReader;
#[doc = "Field `INTEN` writer - Hardware watchdog interrupt and counter enable bit"]
pub type INTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEN` reader - Hardware watchdog reset enable bit"]
pub type RESEN_R = crate::BitReader;
#[doc = "Field `RESEN` writer - Hardware watchdog reset enable bit"]
pub type RESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hardware watchdog interrupt and counter enable bit"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hardware watchdog reset enable bit"]
    #[inline(always)]
    pub fn resen(&self) -> RESEN_R {
        RESEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware watchdog interrupt and counter enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<WDG_CTL_SPEC> {
        INTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Hardware watchdog reset enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn resen(&mut self) -> RESEN_W<WDG_CTL_SPEC> {
        RESEN_W::new(self, 1)
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
#[doc = "Hardware Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDG_CTL_SPEC;
impl crate::RegisterSpec for WDG_CTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdg_ctl::R`](R) reader structure"]
impl crate::Readable for WDG_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdg_ctl::W`](W) writer structure"]
impl crate::Writable for WDG_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WDG_CTL to value 0x03"]
impl crate::Resettable for WDG_CTL_SPEC {
    const RESET_VALUE: u8 = 0x03;
}
