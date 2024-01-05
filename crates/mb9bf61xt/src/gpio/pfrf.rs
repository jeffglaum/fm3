#[doc = "Register `PFRF` reader"]
pub type R = crate::R<PFRF_SPEC>;
#[doc = "Register `PFRF` writer"]
pub type W = crate::W<PFRF_SPEC>;
#[doc = "Field `PF0` reader - Bit0 of PFRF"]
pub type PF0_R = crate::BitReader;
#[doc = "Field `PF0` writer - Bit0 of PFRF"]
pub type PF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF1` reader - Bit1 of PFRF"]
pub type PF1_R = crate::BitReader;
#[doc = "Field `PF1` writer - Bit1 of PFRF"]
pub type PF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF2` reader - Bit2 of PFRF"]
pub type PF2_R = crate::BitReader;
#[doc = "Field `PF2` writer - Bit2 of PFRF"]
pub type PF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF3` reader - Bit3 of PFRF"]
pub type PF3_R = crate::BitReader;
#[doc = "Field `PF3` writer - Bit3 of PFRF"]
pub type PF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF4` reader - Bit4 of PFRF"]
pub type PF4_R = crate::BitReader;
#[doc = "Field `PF4` writer - Bit4 of PFRF"]
pub type PF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF5` reader - Bit5 of PFRF"]
pub type PF5_R = crate::BitReader;
#[doc = "Field `PF5` writer - Bit5 of PFRF"]
pub type PF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF6` reader - Bit6 of PFRF"]
pub type PF6_R = crate::BitReader;
#[doc = "Field `PF6` writer - Bit6 of PFRF"]
pub type PF6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFRF"]
    #[inline(always)]
    pub fn pf0(&self) -> PF0_R {
        PF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRF"]
    #[inline(always)]
    pub fn pf1(&self) -> PF1_R {
        PF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFRF"]
    #[inline(always)]
    pub fn pf2(&self) -> PF2_R {
        PF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFRF"]
    #[inline(always)]
    pub fn pf3(&self) -> PF3_R {
        PF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFRF"]
    #[inline(always)]
    pub fn pf4(&self) -> PF4_R {
        PF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFRF"]
    #[inline(always)]
    pub fn pf5(&self) -> PF5_R {
        PF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFRF"]
    #[inline(always)]
    pub fn pf6(&self) -> PF6_R {
        PF6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFRF"]
    #[inline(always)]
    #[must_use]
    pub fn pf0(&mut self) -> PF0_W<PFRF_SPEC> {
        PF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRF"]
    #[inline(always)]
    #[must_use]
    pub fn pf1(&mut self) -> PF1_W<PFRF_SPEC> {
        PF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFRF"]
    #[inline(always)]
    #[must_use]
    pub fn pf2(&mut self) -> PF2_W<PFRF_SPEC> {
        PF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFRF"]
    #[inline(always)]
    #[must_use]
    pub fn pf3(&mut self) -> PF3_W<PFRF_SPEC> {
        PF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFRF"]
    #[inline(always)]
    #[must_use]
    pub fn pf4(&mut self) -> PF4_W<PFRF_SPEC> {
        PF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFRF"]
    #[inline(always)]
    #[must_use]
    pub fn pf5(&mut self) -> PF5_W<PFRF_SPEC> {
        PF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFRF"]
    #[inline(always)]
    #[must_use]
    pub fn pf6(&mut self) -> PF6_W<PFRF_SPEC> {
        PF6_W::new(self, 6)
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
#[doc = "Port function setting register F\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfrf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfrf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFRF_SPEC;
impl crate::RegisterSpec for PFRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfrf::R`](R) reader structure"]
impl crate::Readable for PFRF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfrf::W`](W) writer structure"]
impl crate::Writable for PFRF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFRF to value 0"]
impl crate::Resettable for PFRF_SPEC {
    const RESET_VALUE: u32 = 0;
}
