#[doc = "Register `WDOGCONTROL` reader"]
pub type R = crate::R<WDOGCONTROL_SPEC>;
#[doc = "Register `WDOGCONTROL` writer"]
pub type W = crate::W<WDOGCONTROL_SPEC>;
#[doc = "Field `INTEN` reader - Interrupt and counter enable bit of the software watchdog"]
pub type INTEN_R = crate::BitReader;
#[doc = "Field `INTEN` writer - Interrupt and counter enable bit of the software watchdog"]
pub type INTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESEN` reader - Reset enable bit of the software watchdog"]
pub type RESEN_R = crate::BitReader;
#[doc = "Field `RESEN` writer - Reset enable bit of the software watchdog"]
pub type RESEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt and counter enable bit of the software watchdog"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset enable bit of the software watchdog"]
    #[inline(always)]
    pub fn resen(&self) -> RESEN_R {
        RESEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt and counter enable bit of the software watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<WDOGCONTROL_SPEC> {
        INTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Reset enable bit of the software watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn resen(&mut self) -> RESEN_W<WDOGCONTROL_SPEC> {
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
#[doc = "Software Watchdog Timer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdogcontrol::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdogcontrol::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDOGCONTROL_SPEC;
impl crate::RegisterSpec for WDOGCONTROL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdogcontrol::R`](R) reader structure"]
impl crate::Readable for WDOGCONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdogcontrol::W`](W) writer structure"]
impl crate::Writable for WDOGCONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WDOGCONTROL to value 0"]
impl crate::Resettable for WDOGCONTROL_SPEC {
    const RESET_VALUE: u8 = 0;
}
