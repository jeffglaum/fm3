#[doc = "Register `PFR1` reader"]
pub type R = crate::R<PFR1_SPEC>;
#[doc = "Register `PFR1` writer"]
pub type W = crate::W<PFR1_SPEC>;
#[doc = "Field `P10` reader - Bit0 of PFR1"]
pub type P10_R = crate::BitReader;
#[doc = "Field `P10` writer - Bit0 of PFR1"]
pub type P10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P11` reader - Bit1 of PFR1"]
pub type P11_R = crate::BitReader;
#[doc = "Field `P11` writer - Bit1 of PFR1"]
pub type P11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P12` reader - Bit2 of PFR1"]
pub type P12_R = crate::BitReader;
#[doc = "Field `P12` writer - Bit2 of PFR1"]
pub type P12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P13` reader - Bit3 of PFR1"]
pub type P13_R = crate::BitReader;
#[doc = "Field `P13` writer - Bit3 of PFR1"]
pub type P13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P14` reader - Bit4 of PFR1"]
pub type P14_R = crate::BitReader;
#[doc = "Field `P14` writer - Bit4 of PFR1"]
pub type P14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P15` reader - Bit5 of PFR1"]
pub type P15_R = crate::BitReader;
#[doc = "Field `P15` writer - Bit5 of PFR1"]
pub type P15_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P16` reader - Bit6 of PFR1"]
pub type P16_R = crate::BitReader;
#[doc = "Field `P16` writer - Bit6 of PFR1"]
pub type P16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P17` reader - Bit7 of PFR1"]
pub type P17_R = crate::BitReader;
#[doc = "Field `P17` writer - Bit7 of PFR1"]
pub type P17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P18` reader - Bit8 of PFR1"]
pub type P18_R = crate::BitReader;
#[doc = "Field `P18` writer - Bit8 of PFR1"]
pub type P18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P19` reader - Bit9 of PFR1"]
pub type P19_R = crate::BitReader;
#[doc = "Field `P19` writer - Bit9 of PFR1"]
pub type P19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1A` reader - Bit10 of PFR1"]
pub type P1A_R = crate::BitReader;
#[doc = "Field `P1A` writer - Bit10 of PFR1"]
pub type P1A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1B` reader - Bit11 of PFR1"]
pub type P1B_R = crate::BitReader;
#[doc = "Field `P1B` writer - Bit11 of PFR1"]
pub type P1B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1C` reader - Bit12 of PFR1"]
pub type P1C_R = crate::BitReader;
#[doc = "Field `P1C` writer - Bit12 of PFR1"]
pub type P1C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1D` reader - Bit13 of PFR1"]
pub type P1D_R = crate::BitReader;
#[doc = "Field `P1D` writer - Bit13 of PFR1"]
pub type P1D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1E` reader - Bit14 of PFR1"]
pub type P1E_R = crate::BitReader;
#[doc = "Field `P1E` writer - Bit14 of PFR1"]
pub type P1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P1F` reader - Bit15 of PFR1"]
pub type P1F_R = crate::BitReader;
#[doc = "Field `P1F` writer - Bit15 of PFR1"]
pub type P1F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR1"]
    #[inline(always)]
    pub fn p10(&self) -> P10_R {
        P10_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR1"]
    #[inline(always)]
    pub fn p11(&self) -> P11_R {
        P11_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR1"]
    #[inline(always)]
    pub fn p12(&self) -> P12_R {
        P12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR1"]
    #[inline(always)]
    pub fn p13(&self) -> P13_R {
        P13_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFR1"]
    #[inline(always)]
    pub fn p14(&self) -> P14_R {
        P14_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFR1"]
    #[inline(always)]
    pub fn p15(&self) -> P15_R {
        P15_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFR1"]
    #[inline(always)]
    pub fn p16(&self) -> P16_R {
        P16_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFR1"]
    #[inline(always)]
    pub fn p17(&self) -> P17_R {
        P17_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of PFR1"]
    #[inline(always)]
    pub fn p18(&self) -> P18_R {
        P18_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of PFR1"]
    #[inline(always)]
    pub fn p19(&self) -> P19_R {
        P19_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit10 of PFR1"]
    #[inline(always)]
    pub fn p1a(&self) -> P1A_R {
        P1A_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit11 of PFR1"]
    #[inline(always)]
    pub fn p1b(&self) -> P1B_R {
        P1B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bit12 of PFR1"]
    #[inline(always)]
    pub fn p1c(&self) -> P1C_R {
        P1C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit13 of PFR1"]
    #[inline(always)]
    pub fn p1d(&self) -> P1D_R {
        P1D_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit14 of PFR1"]
    #[inline(always)]
    pub fn p1e(&self) -> P1E_R {
        P1E_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bit15 of PFR1"]
    #[inline(always)]
    pub fn p1f(&self) -> P1F_R {
        P1F_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<PFR1_SPEC> {
        P10_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<PFR1_SPEC> {
        P11_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<PFR1_SPEC> {
        P12_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<PFR1_SPEC> {
        P13_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<PFR1_SPEC> {
        P14_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<PFR1_SPEC> {
        P15_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<PFR1_SPEC> {
        P16_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<PFR1_SPEC> {
        P17_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<PFR1_SPEC> {
        P18_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<PFR1_SPEC> {
        P19_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bit10 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p1a(&mut self) -> P1A_W<PFR1_SPEC> {
        P1A_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit11 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p1b(&mut self) -> P1B_W<PFR1_SPEC> {
        P1B_W::new(self, 11)
    }
    #[doc = "Bit 12 - Bit12 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p1c(&mut self) -> P1C_W<PFR1_SPEC> {
        P1C_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bit13 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p1d(&mut self) -> P1D_W<PFR1_SPEC> {
        P1D_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bit14 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p1e(&mut self) -> P1E_W<PFR1_SPEC> {
        P1E_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bit15 of PFR1"]
    #[inline(always)]
    #[must_use]
    pub fn p1f(&mut self) -> P1F_W<PFR1_SPEC> {
        P1F_W::new(self, 15)
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
#[doc = "Port function setting register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR1_SPEC;
impl crate::RegisterSpec for PFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr1::R`](R) reader structure"]
impl crate::Readable for PFR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr1::W`](W) writer structure"]
impl crate::Writable for PFR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR1 to value 0"]
impl crate::Resettable for PFR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
