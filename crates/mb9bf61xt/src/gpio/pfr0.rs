#[doc = "Register `PFR0` reader"]
pub type R = crate::R<PFR0_SPEC>;
#[doc = "Register `PFR0` writer"]
pub type W = crate::W<PFR0_SPEC>;
#[doc = "Field `P00` reader - Bit0 of PFR0"]
pub type P00_R = crate::BitReader;
#[doc = "Field `P00` writer - Bit0 of PFR0"]
pub type P00_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P01` reader - Bit1 of PFR0"]
pub type P01_R = crate::BitReader;
#[doc = "Field `P01` writer - Bit1 of PFR0"]
pub type P01_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P02` reader - Bit2 of PFR0"]
pub type P02_R = crate::BitReader;
#[doc = "Field `P02` writer - Bit2 of PFR0"]
pub type P02_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P03` reader - Bit3 of PFR0"]
pub type P03_R = crate::BitReader;
#[doc = "Field `P03` writer - Bit3 of PFR0"]
pub type P03_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P04` reader - Bit4 of PFR0"]
pub type P04_R = crate::BitReader;
#[doc = "Field `P04` writer - Bit4 of PFR0"]
pub type P04_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P05` reader - Bit5 of PFR0"]
pub type P05_R = crate::BitReader;
#[doc = "Field `P05` writer - Bit5 of PFR0"]
pub type P05_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P06` reader - Bit6 of PFR0"]
pub type P06_R = crate::BitReader;
#[doc = "Field `P06` writer - Bit6 of PFR0"]
pub type P06_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P07` reader - Bit7 of PFR0"]
pub type P07_R = crate::BitReader;
#[doc = "Field `P07` writer - Bit7 of PFR0"]
pub type P07_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P08` reader - Bit8 of PFR0"]
pub type P08_R = crate::BitReader;
#[doc = "Field `P08` writer - Bit8 of PFR0"]
pub type P08_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P09` reader - Bit9 of PFR0"]
pub type P09_R = crate::BitReader;
#[doc = "Field `P09` writer - Bit9 of PFR0"]
pub type P09_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR0"]
    #[inline(always)]
    pub fn p00(&self) -> P00_R {
        P00_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR0"]
    #[inline(always)]
    pub fn p01(&self) -> P01_R {
        P01_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR0"]
    #[inline(always)]
    pub fn p02(&self) -> P02_R {
        P02_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR0"]
    #[inline(always)]
    pub fn p03(&self) -> P03_R {
        P03_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFR0"]
    #[inline(always)]
    pub fn p04(&self) -> P04_R {
        P04_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFR0"]
    #[inline(always)]
    pub fn p05(&self) -> P05_R {
        P05_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFR0"]
    #[inline(always)]
    pub fn p06(&self) -> P06_R {
        P06_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFR0"]
    #[inline(always)]
    pub fn p07(&self) -> P07_R {
        P07_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of PFR0"]
    #[inline(always)]
    pub fn p08(&self) -> P08_R {
        P08_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of PFR0"]
    #[inline(always)]
    pub fn p09(&self) -> P09_R {
        P09_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p00(&mut self) -> P00_W<PFR0_SPEC> {
        P00_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p01(&mut self) -> P01_W<PFR0_SPEC> {
        P01_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p02(&mut self) -> P02_W<PFR0_SPEC> {
        P02_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p03(&mut self) -> P03_W<PFR0_SPEC> {
        P03_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p04(&mut self) -> P04_W<PFR0_SPEC> {
        P04_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p05(&mut self) -> P05_W<PFR0_SPEC> {
        P05_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p06(&mut self) -> P06_W<PFR0_SPEC> {
        P06_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p07(&mut self) -> P07_W<PFR0_SPEC> {
        P07_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p08(&mut self) -> P08_W<PFR0_SPEC> {
        P08_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of PFR0"]
    #[inline(always)]
    #[must_use]
    pub fn p09(&mut self) -> P09_W<PFR0_SPEC> {
        P09_W::new(self, 9)
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
#[doc = "Port function setting register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR0_SPEC;
impl crate::RegisterSpec for PFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr0::R`](R) reader structure"]
impl crate::Readable for PFR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr0::W`](W) writer structure"]
impl crate::Writable for PFR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR0 to value 0x1f"]
impl crate::Resettable for PFR0_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
