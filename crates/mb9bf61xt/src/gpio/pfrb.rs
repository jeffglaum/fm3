#[doc = "Register `PFRB` reader"]
pub type R = crate::R<PFRB_SPEC>;
#[doc = "Register `PFRB` writer"]
pub type W = crate::W<PFRB_SPEC>;
#[doc = "Field `PB0` reader - Bit0 of PFRB"]
pub type PB0_R = crate::BitReader;
#[doc = "Field `PB0` writer - Bit0 of PFRB"]
pub type PB0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB1` reader - Bit1 of PFRB"]
pub type PB1_R = crate::BitReader;
#[doc = "Field `PB1` writer - Bit1 of PFRB"]
pub type PB1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB2` reader - Bit2 of PFRB"]
pub type PB2_R = crate::BitReader;
#[doc = "Field `PB2` writer - Bit2 of PFRB"]
pub type PB2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB3` reader - Bit3 of PFRB"]
pub type PB3_R = crate::BitReader;
#[doc = "Field `PB3` writer - Bit3 of PFRB"]
pub type PB3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB4` reader - Bit4 of PFRB"]
pub type PB4_R = crate::BitReader;
#[doc = "Field `PB4` writer - Bit4 of PFRB"]
pub type PB4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB5` reader - Bit5 of PFRB"]
pub type PB5_R = crate::BitReader;
#[doc = "Field `PB5` writer - Bit5 of PFRB"]
pub type PB5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB6` reader - Bit6 of PFRB"]
pub type PB6_R = crate::BitReader;
#[doc = "Field `PB6` writer - Bit6 of PFRB"]
pub type PB6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB7` reader - Bit7 of PFRB"]
pub type PB7_R = crate::BitReader;
#[doc = "Field `PB7` writer - Bit7 of PFRB"]
pub type PB7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFRB"]
    #[inline(always)]
    pub fn pb0(&self) -> PB0_R {
        PB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRB"]
    #[inline(always)]
    pub fn pb1(&self) -> PB1_R {
        PB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFRB"]
    #[inline(always)]
    pub fn pb2(&self) -> PB2_R {
        PB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFRB"]
    #[inline(always)]
    pub fn pb3(&self) -> PB3_R {
        PB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFRB"]
    #[inline(always)]
    pub fn pb4(&self) -> PB4_R {
        PB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFRB"]
    #[inline(always)]
    pub fn pb5(&self) -> PB5_R {
        PB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFRB"]
    #[inline(always)]
    pub fn pb6(&self) -> PB6_R {
        PB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFRB"]
    #[inline(always)]
    pub fn pb7(&self) -> PB7_R {
        PB7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFRB"]
    #[inline(always)]
    #[must_use]
    pub fn pb0(&mut self) -> PB0_W<PFRB_SPEC> {
        PB0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRB"]
    #[inline(always)]
    #[must_use]
    pub fn pb1(&mut self) -> PB1_W<PFRB_SPEC> {
        PB1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFRB"]
    #[inline(always)]
    #[must_use]
    pub fn pb2(&mut self) -> PB2_W<PFRB_SPEC> {
        PB2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFRB"]
    #[inline(always)]
    #[must_use]
    pub fn pb3(&mut self) -> PB3_W<PFRB_SPEC> {
        PB3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFRB"]
    #[inline(always)]
    #[must_use]
    pub fn pb4(&mut self) -> PB4_W<PFRB_SPEC> {
        PB4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFRB"]
    #[inline(always)]
    #[must_use]
    pub fn pb5(&mut self) -> PB5_W<PFRB_SPEC> {
        PB5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFRB"]
    #[inline(always)]
    #[must_use]
    pub fn pb6(&mut self) -> PB6_W<PFRB_SPEC> {
        PB6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFRB"]
    #[inline(always)]
    #[must_use]
    pub fn pb7(&mut self) -> PB7_W<PFRB_SPEC> {
        PB7_W::new(self, 7)
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
#[doc = "Port function setting register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfrb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfrb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFRB_SPEC;
impl crate::RegisterSpec for PFRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfrb::R`](R) reader structure"]
impl crate::Readable for PFRB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfrb::W`](W) writer structure"]
impl crate::Writable for PFRB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFRB to value 0"]
impl crate::Resettable for PFRB_SPEC {
    const RESET_VALUE: u32 = 0;
}
