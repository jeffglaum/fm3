#[doc = "Register `SPSR` reader"]
pub type R = crate::R<SPSR_SPEC>;
#[doc = "Register `SPSR` writer"]
pub type W = crate::W<SPSR_SPEC>;
#[doc = "Field `SUBXC` reader - Sub clock(oscillation) pin setting bit"]
pub type SUBXC_R = crate::BitReader;
#[doc = "Field `SUBXC` writer - Sub clock(oscillation) pin setting bit"]
pub type SUBXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAINXC` reader - Main clock(oscillation) pin setting bit"]
pub type MAINXC_R = crate::BitReader;
#[doc = "Field `MAINXC` writer - Main clock(oscillation) pin setting bit"]
pub type MAINXC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB0C` reader - USBch0 pin setting bit"]
pub type USB0C_R = crate::BitReader;
#[doc = "Field `USB0C` writer - USBch0 pin setting bit"]
pub type USB0C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1C` reader - USBch1 pin setting bit"]
pub type USB1C_R = crate::BitReader;
#[doc = "Field `USB1C` writer - USBch1 pin setting bit"]
pub type USB1C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sub clock(oscillation) pin setting bit"]
    #[inline(always)]
    pub fn subxc(&self) -> SUBXC_R {
        SUBXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Main clock(oscillation) pin setting bit"]
    #[inline(always)]
    pub fn mainxc(&self) -> MAINXC_R {
        MAINXC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - USBch0 pin setting bit"]
    #[inline(always)]
    pub fn usb0c(&self) -> USB0C_R {
        USB0C_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USBch1 pin setting bit"]
    #[inline(always)]
    pub fn usb1c(&self) -> USB1C_R {
        USB1C_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sub clock(oscillation) pin setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn subxc(&mut self) -> SUBXC_W<SPSR_SPEC> {
        SUBXC_W::new(self, 0)
    }
    #[doc = "Bit 2 - Main clock(oscillation) pin setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn mainxc(&mut self) -> MAINXC_W<SPSR_SPEC> {
        MAINXC_W::new(self, 2)
    }
    #[doc = "Bit 4 - USBch0 pin setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb0c(&mut self) -> USB0C_W<SPSR_SPEC> {
        USB0C_W::new(self, 4)
    }
    #[doc = "Bit 5 - USBch1 pin setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn usb1c(&mut self) -> USB1C_W<SPSR_SPEC> {
        USB1C_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Special port setting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPSR_SPEC;
impl crate::RegisterSpec for SPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spsr::R`](R) reader structure"]
impl crate::Readable for SPSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spsr::W`](W) writer structure"]
impl crate::Writable for SPSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPSR to value 0x05"]
impl crate::Resettable for SPSR_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
