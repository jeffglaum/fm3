#[doc = "Register `PFR5` reader"]
pub type R = crate::R<PFR5_SPEC>;
#[doc = "Register `PFR5` writer"]
pub type W = crate::W<PFR5_SPEC>;
#[doc = "Field `P50` reader - Bit0 of PFR5"]
pub type P50_R = crate::BitReader;
#[doc = "Field `P50` writer - Bit0 of PFR5"]
pub type P50_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P51` reader - Bit1 of PFR5"]
pub type P51_R = crate::BitReader;
#[doc = "Field `P51` writer - Bit1 of PFR5"]
pub type P51_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P52` reader - Bit2 of PFR5"]
pub type P52_R = crate::BitReader;
#[doc = "Field `P52` writer - Bit2 of PFR5"]
pub type P52_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P53` reader - Bit3 of PFR5"]
pub type P53_R = crate::BitReader;
#[doc = "Field `P53` writer - Bit3 of PFR5"]
pub type P53_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P54` reader - Bit4 of PFR5"]
pub type P54_R = crate::BitReader;
#[doc = "Field `P54` writer - Bit4 of PFR5"]
pub type P54_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P55` reader - Bit5 of PFR5"]
pub type P55_R = crate::BitReader;
#[doc = "Field `P55` writer - Bit5 of PFR5"]
pub type P55_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P56` reader - Bit6 of PFR5"]
pub type P56_R = crate::BitReader;
#[doc = "Field `P56` writer - Bit6 of PFR5"]
pub type P56_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P57` reader - Bit7 of PFR5"]
pub type P57_R = crate::BitReader;
#[doc = "Field `P57` writer - Bit7 of PFR5"]
pub type P57_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P58` reader - Bit8 of PFR5"]
pub type P58_R = crate::BitReader;
#[doc = "Field `P58` writer - Bit8 of PFR5"]
pub type P58_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P59` reader - Bit9 of PFR5"]
pub type P59_R = crate::BitReader;
#[doc = "Field `P59` writer - Bit9 of PFR5"]
pub type P59_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5A` reader - Bit10 of PFR5"]
pub type P5A_R = crate::BitReader;
#[doc = "Field `P5A` writer - Bit10 of PFR5"]
pub type P5A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5B` reader - Bit11 of PFR5"]
pub type P5B_R = crate::BitReader;
#[doc = "Field `P5B` writer - Bit11 of PFR5"]
pub type P5B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5C` reader - Bit12 of PFR5"]
pub type P5C_R = crate::BitReader;
#[doc = "Field `P5C` writer - Bit12 of PFR5"]
pub type P5C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P5D` reader - Bit13 of PFR5"]
pub type P5D_R = crate::BitReader;
#[doc = "Field `P5D` writer - Bit13 of PFR5"]
pub type P5D_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR5"]
    #[inline(always)]
    pub fn p50(&self) -> P50_R {
        P50_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR5"]
    #[inline(always)]
    pub fn p51(&self) -> P51_R {
        P51_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR5"]
    #[inline(always)]
    pub fn p52(&self) -> P52_R {
        P52_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR5"]
    #[inline(always)]
    pub fn p53(&self) -> P53_R {
        P53_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFR5"]
    #[inline(always)]
    pub fn p54(&self) -> P54_R {
        P54_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFR5"]
    #[inline(always)]
    pub fn p55(&self) -> P55_R {
        P55_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFR5"]
    #[inline(always)]
    pub fn p56(&self) -> P56_R {
        P56_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFR5"]
    #[inline(always)]
    pub fn p57(&self) -> P57_R {
        P57_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of PFR5"]
    #[inline(always)]
    pub fn p58(&self) -> P58_R {
        P58_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of PFR5"]
    #[inline(always)]
    pub fn p59(&self) -> P59_R {
        P59_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit10 of PFR5"]
    #[inline(always)]
    pub fn p5a(&self) -> P5A_R {
        P5A_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit11 of PFR5"]
    #[inline(always)]
    pub fn p5b(&self) -> P5B_R {
        P5B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bit12 of PFR5"]
    #[inline(always)]
    pub fn p5c(&self) -> P5C_R {
        P5C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit13 of PFR5"]
    #[inline(always)]
    pub fn p5d(&self) -> P5D_R {
        P5D_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p50(&mut self) -> P50_W<PFR5_SPEC> {
        P50_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p51(&mut self) -> P51_W<PFR5_SPEC> {
        P51_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p52(&mut self) -> P52_W<PFR5_SPEC> {
        P52_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p53(&mut self) -> P53_W<PFR5_SPEC> {
        P53_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p54(&mut self) -> P54_W<PFR5_SPEC> {
        P54_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p55(&mut self) -> P55_W<PFR5_SPEC> {
        P55_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p56(&mut self) -> P56_W<PFR5_SPEC> {
        P56_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p57(&mut self) -> P57_W<PFR5_SPEC> {
        P57_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p58(&mut self) -> P58_W<PFR5_SPEC> {
        P58_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p59(&mut self) -> P59_W<PFR5_SPEC> {
        P59_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bit10 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p5a(&mut self) -> P5A_W<PFR5_SPEC> {
        P5A_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit11 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p5b(&mut self) -> P5B_W<PFR5_SPEC> {
        P5B_W::new(self, 11)
    }
    #[doc = "Bit 12 - Bit12 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p5c(&mut self) -> P5C_W<PFR5_SPEC> {
        P5C_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bit13 of PFR5"]
    #[inline(always)]
    #[must_use]
    pub fn p5d(&mut self) -> P5D_W<PFR5_SPEC> {
        P5D_W::new(self, 13)
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
#[doc = "Port function setting register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR5_SPEC;
impl crate::RegisterSpec for PFR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr5::R`](R) reader structure"]
impl crate::Readable for PFR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr5::W`](W) writer structure"]
impl crate::Writable for PFR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR5 to value 0"]
impl crate::Resettable for PFR5_SPEC {
    const RESET_VALUE: u32 = 0;
}
