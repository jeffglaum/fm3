#[doc = "Register `PFR6` reader"]
pub type R = crate::R<PFR6_SPEC>;
#[doc = "Register `PFR6` writer"]
pub type W = crate::W<PFR6_SPEC>;
#[doc = "Field `P60` reader - Bit0 of PFR6"]
pub type P60_R = crate::BitReader;
#[doc = "Field `P60` writer - Bit0 of PFR6"]
pub type P60_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P61` reader - Bit1 of PFR6"]
pub type P61_R = crate::BitReader;
#[doc = "Field `P61` writer - Bit1 of PFR6"]
pub type P61_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P62` reader - Bit2 of PFR6"]
pub type P62_R = crate::BitReader;
#[doc = "Field `P62` writer - Bit2 of PFR6"]
pub type P62_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR6"]
    #[inline(always)]
    pub fn p60(&self) -> P60_R {
        P60_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR6"]
    #[inline(always)]
    pub fn p61(&self) -> P61_R {
        P61_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR6"]
    #[inline(always)]
    pub fn p62(&self) -> P62_R {
        P62_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR6"]
    #[inline(always)]
    #[must_use]
    pub fn p60(&mut self) -> P60_W<PFR6_SPEC> {
        P60_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR6"]
    #[inline(always)]
    #[must_use]
    pub fn p61(&mut self) -> P61_W<PFR6_SPEC> {
        P61_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR6"]
    #[inline(always)]
    #[must_use]
    pub fn p62(&mut self) -> P62_W<PFR6_SPEC> {
        P62_W::new(self, 2)
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
#[doc = "Port function setting register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR6_SPEC;
impl crate::RegisterSpec for PFR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr6::R`](R) reader structure"]
impl crate::Readable for PFR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr6::W`](W) writer structure"]
impl crate::Writable for PFR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR6 to value 0"]
impl crate::Resettable for PFR6_SPEC {
    const RESET_VALUE: u32 = 0;
}
