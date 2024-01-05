#[doc = "Register `PFR9` reader"]
pub type R = crate::R<PFR9_SPEC>;
#[doc = "Register `PFR9` writer"]
pub type W = crate::W<PFR9_SPEC>;
#[doc = "Field `P90` reader - Bit0 of PFR9"]
pub type P90_R = crate::BitReader;
#[doc = "Field `P90` writer - Bit0 of PFR9"]
pub type P90_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P91` reader - Bit1 of PFR9"]
pub type P91_R = crate::BitReader;
#[doc = "Field `P91` writer - Bit1 of PFR9"]
pub type P91_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P92` reader - Bit2 of PFR9"]
pub type P92_R = crate::BitReader;
#[doc = "Field `P92` writer - Bit2 of PFR9"]
pub type P92_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P93` reader - Bit3 of PFR9"]
pub type P93_R = crate::BitReader;
#[doc = "Field `P93` writer - Bit3 of PFR9"]
pub type P93_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P94` reader - Bit4 of PFR9"]
pub type P94_R = crate::BitReader;
#[doc = "Field `P94` writer - Bit4 of PFR9"]
pub type P94_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P95` reader - Bit5 of PFR9"]
pub type P95_R = crate::BitReader;
#[doc = "Field `P95` writer - Bit5 of PFR9"]
pub type P95_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR9"]
    #[inline(always)]
    pub fn p90(&self) -> P90_R {
        P90_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR9"]
    #[inline(always)]
    pub fn p91(&self) -> P91_R {
        P91_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR9"]
    #[inline(always)]
    pub fn p92(&self) -> P92_R {
        P92_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR9"]
    #[inline(always)]
    pub fn p93(&self) -> P93_R {
        P93_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFR9"]
    #[inline(always)]
    pub fn p94(&self) -> P94_R {
        P94_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFR9"]
    #[inline(always)]
    pub fn p95(&self) -> P95_R {
        P95_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR9"]
    #[inline(always)]
    #[must_use]
    pub fn p90(&mut self) -> P90_W<PFR9_SPEC> {
        P90_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR9"]
    #[inline(always)]
    #[must_use]
    pub fn p91(&mut self) -> P91_W<PFR9_SPEC> {
        P91_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR9"]
    #[inline(always)]
    #[must_use]
    pub fn p92(&mut self) -> P92_W<PFR9_SPEC> {
        P92_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR9"]
    #[inline(always)]
    #[must_use]
    pub fn p93(&mut self) -> P93_W<PFR9_SPEC> {
        P93_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFR9"]
    #[inline(always)]
    #[must_use]
    pub fn p94(&mut self) -> P94_W<PFR9_SPEC> {
        P94_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFR9"]
    #[inline(always)]
    #[must_use]
    pub fn p95(&mut self) -> P95_W<PFR9_SPEC> {
        P95_W::new(self, 5)
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
#[doc = "Port function setting register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR9_SPEC;
impl crate::RegisterSpec for PFR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr9::R`](R) reader structure"]
impl crate::Readable for PFR9_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr9::W`](W) writer structure"]
impl crate::Writable for PFR9_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR9 to value 0"]
impl crate::Resettable for PFR9_SPEC {
    const RESET_VALUE: u32 = 0;
}
