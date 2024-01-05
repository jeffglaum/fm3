#[doc = "Register `PFR4` reader"]
pub type R = crate::R<PFR4_SPEC>;
#[doc = "Register `PFR4` writer"]
pub type W = crate::W<PFR4_SPEC>;
#[doc = "Field `P40` reader - Bit0 of PFR4"]
pub type P40_R = crate::BitReader;
#[doc = "Field `P40` writer - Bit0 of PFR4"]
pub type P40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P41` reader - Bit1 of PFR4"]
pub type P41_R = crate::BitReader;
#[doc = "Field `P41` writer - Bit1 of PFR4"]
pub type P41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P42` reader - Bit2 of PFR4"]
pub type P42_R = crate::BitReader;
#[doc = "Field `P42` writer - Bit2 of PFR4"]
pub type P42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P43` reader - Bit3 of PFR4"]
pub type P43_R = crate::BitReader;
#[doc = "Field `P43` writer - Bit3 of PFR4"]
pub type P43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P44` reader - Bit4 of PFR4"]
pub type P44_R = crate::BitReader;
#[doc = "Field `P44` writer - Bit4 of PFR4"]
pub type P44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P45` reader - Bit5 of PFR4"]
pub type P45_R = crate::BitReader;
#[doc = "Field `P45` writer - Bit5 of PFR4"]
pub type P45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P46` reader - Bit6 of PFR4"]
pub type P46_R = crate::BitReader;
#[doc = "Field `P46` writer - Bit6 of PFR4"]
pub type P46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P47` reader - Bit7 of PFR4"]
pub type P47_R = crate::BitReader;
#[doc = "Field `P47` writer - Bit7 of PFR4"]
pub type P47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P48` reader - Bit8 of PFR4"]
pub type P48_R = crate::BitReader;
#[doc = "Field `P48` writer - Bit8 of PFR4"]
pub type P48_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P49` reader - Bit9 of PFR4"]
pub type P49_R = crate::BitReader;
#[doc = "Field `P49` writer - Bit9 of PFR4"]
pub type P49_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4A` reader - Bit10 of PFR4"]
pub type P4A_R = crate::BitReader;
#[doc = "Field `P4A` writer - Bit10 of PFR4"]
pub type P4A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4B` reader - Bit11 of PFR4"]
pub type P4B_R = crate::BitReader;
#[doc = "Field `P4B` writer - Bit11 of PFR4"]
pub type P4B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4C` reader - Bit12 of PFR4"]
pub type P4C_R = crate::BitReader;
#[doc = "Field `P4C` writer - Bit12 of PFR4"]
pub type P4C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4D` reader - Bit13 of PFR4"]
pub type P4D_R = crate::BitReader;
#[doc = "Field `P4D` writer - Bit13 of PFR4"]
pub type P4D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P4E` reader - Bit14 of PFR4"]
pub type P4E_R = crate::BitReader;
#[doc = "Field `P4E` writer - Bit14 of PFR4"]
pub type P4E_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR4"]
    #[inline(always)]
    pub fn p40(&self) -> P40_R {
        P40_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR4"]
    #[inline(always)]
    pub fn p41(&self) -> P41_R {
        P41_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR4"]
    #[inline(always)]
    pub fn p42(&self) -> P42_R {
        P42_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR4"]
    #[inline(always)]
    pub fn p43(&self) -> P43_R {
        P43_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFR4"]
    #[inline(always)]
    pub fn p44(&self) -> P44_R {
        P44_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFR4"]
    #[inline(always)]
    pub fn p45(&self) -> P45_R {
        P45_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFR4"]
    #[inline(always)]
    pub fn p46(&self) -> P46_R {
        P46_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFR4"]
    #[inline(always)]
    pub fn p47(&self) -> P47_R {
        P47_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of PFR4"]
    #[inline(always)]
    pub fn p48(&self) -> P48_R {
        P48_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of PFR4"]
    #[inline(always)]
    pub fn p49(&self) -> P49_R {
        P49_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit10 of PFR4"]
    #[inline(always)]
    pub fn p4a(&self) -> P4A_R {
        P4A_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit11 of PFR4"]
    #[inline(always)]
    pub fn p4b(&self) -> P4B_R {
        P4B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bit12 of PFR4"]
    #[inline(always)]
    pub fn p4c(&self) -> P4C_R {
        P4C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit13 of PFR4"]
    #[inline(always)]
    pub fn p4d(&self) -> P4D_R {
        P4D_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit14 of PFR4"]
    #[inline(always)]
    pub fn p4e(&self) -> P4E_R {
        P4E_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p40(&mut self) -> P40_W<PFR4_SPEC> {
        P40_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p41(&mut self) -> P41_W<PFR4_SPEC> {
        P41_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p42(&mut self) -> P42_W<PFR4_SPEC> {
        P42_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p43(&mut self) -> P43_W<PFR4_SPEC> {
        P43_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p44(&mut self) -> P44_W<PFR4_SPEC> {
        P44_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p45(&mut self) -> P45_W<PFR4_SPEC> {
        P45_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p46(&mut self) -> P46_W<PFR4_SPEC> {
        P46_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p47(&mut self) -> P47_W<PFR4_SPEC> {
        P47_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p48(&mut self) -> P48_W<PFR4_SPEC> {
        P48_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p49(&mut self) -> P49_W<PFR4_SPEC> {
        P49_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bit10 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p4a(&mut self) -> P4A_W<PFR4_SPEC> {
        P4A_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit11 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p4b(&mut self) -> P4B_W<PFR4_SPEC> {
        P4B_W::new(self, 11)
    }
    #[doc = "Bit 12 - Bit12 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p4c(&mut self) -> P4C_W<PFR4_SPEC> {
        P4C_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bit13 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p4d(&mut self) -> P4D_W<PFR4_SPEC> {
        P4D_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bit14 of PFR4"]
    #[inline(always)]
    #[must_use]
    pub fn p4e(&mut self) -> P4E_W<PFR4_SPEC> {
        P4E_W::new(self, 14)
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
#[doc = "Port function setting register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR4_SPEC;
impl crate::RegisterSpec for PFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr4::R`](R) reader structure"]
impl crate::Readable for PFR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr4::W`](W) writer structure"]
impl crate::Writable for PFR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR4 to value 0"]
impl crate::Resettable for PFR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
