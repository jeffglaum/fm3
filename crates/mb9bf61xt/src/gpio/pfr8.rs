#[doc = "Register `PFR8` reader"]
pub type R = crate::R<PFR8_SPEC>;
#[doc = "Register `PFR8` writer"]
pub type W = crate::W<PFR8_SPEC>;
#[doc = "Field `P80` reader - Bit0 of PFR8"]
pub type P80_R = crate::BitReader;
#[doc = "Field `P80` writer - Bit0 of PFR8"]
pub type P80_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P81` reader - Bit1 of PFR8"]
pub type P81_R = crate::BitReader;
#[doc = "Field `P81` writer - Bit1 of PFR8"]
pub type P81_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P82` reader - Bit2 of PFR8"]
pub type P82_R = crate::BitReader;
#[doc = "Field `P82` writer - Bit2 of PFR8"]
pub type P82_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P83` reader - Bit3 of PFR8"]
pub type P83_R = crate::BitReader;
#[doc = "Field `P83` writer - Bit3 of PFR8"]
pub type P83_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR8"]
    #[inline(always)]
    pub fn p80(&self) -> P80_R {
        P80_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR8"]
    #[inline(always)]
    pub fn p81(&self) -> P81_R {
        P81_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR8"]
    #[inline(always)]
    pub fn p82(&self) -> P82_R {
        P82_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR8"]
    #[inline(always)]
    pub fn p83(&self) -> P83_R {
        P83_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR8"]
    #[inline(always)]
    #[must_use]
    pub fn p80(&mut self) -> P80_W<PFR8_SPEC> {
        P80_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR8"]
    #[inline(always)]
    #[must_use]
    pub fn p81(&mut self) -> P81_W<PFR8_SPEC> {
        P81_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR8"]
    #[inline(always)]
    #[must_use]
    pub fn p82(&mut self) -> P82_W<PFR8_SPEC> {
        P82_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR8"]
    #[inline(always)]
    #[must_use]
    pub fn p83(&mut self) -> P83_W<PFR8_SPEC> {
        P83_W::new(self, 3)
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
#[doc = "Port function setting register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR8_SPEC;
impl crate::RegisterSpec for PFR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr8::R`](R) reader structure"]
impl crate::Readable for PFR8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr8::W`](W) writer structure"]
impl crate::Writable for PFR8_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR8 to value 0"]
impl crate::Resettable for PFR8_SPEC {
    const RESET_VALUE: u32 = 0;
}
