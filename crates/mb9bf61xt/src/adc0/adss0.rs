#[doc = "Register `ADSS0` reader"]
pub type R = crate::R<ADSS0_SPEC>;
#[doc = "Register `ADSS0` writer"]
pub type W = crate::W<ADSS0_SPEC>;
#[doc = "Field `TS0` reader - Bit0 of ADSS0"]
pub type TS0_R = crate::BitReader;
#[doc = "Field `TS0` writer - Bit0 of ADSS0"]
pub type TS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1` reader - Bit1 of ADSS0"]
pub type TS1_R = crate::BitReader;
#[doc = "Field `TS1` writer - Bit1 of ADSS0"]
pub type TS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2` reader - Bit2 of ADSS0"]
pub type TS2_R = crate::BitReader;
#[doc = "Field `TS2` writer - Bit2 of ADSS0"]
pub type TS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS3` reader - Bit3 of ADSS0"]
pub type TS3_R = crate::BitReader;
#[doc = "Field `TS3` writer - Bit3 of ADSS0"]
pub type TS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS4` reader - Bit4 of ADSS0"]
pub type TS4_R = crate::BitReader;
#[doc = "Field `TS4` writer - Bit4 of ADSS0"]
pub type TS4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS5` reader - Bit5 of ADSS0"]
pub type TS5_R = crate::BitReader;
#[doc = "Field `TS5` writer - Bit5 of ADSS0"]
pub type TS5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS6` reader - Bit6 of ADSS0"]
pub type TS6_R = crate::BitReader;
#[doc = "Field `TS6` writer - Bit6 of ADSS0"]
pub type TS6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS7` reader - Bit7 of ADSS0"]
pub type TS7_R = crate::BitReader;
#[doc = "Field `TS7` writer - Bit7 of ADSS0"]
pub type TS7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of ADSS0"]
    #[inline(always)]
    pub fn ts0(&self) -> TS0_R {
        TS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of ADSS0"]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of ADSS0"]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of ADSS0"]
    #[inline(always)]
    pub fn ts3(&self) -> TS3_R {
        TS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of ADSS0"]
    #[inline(always)]
    pub fn ts4(&self) -> TS4_R {
        TS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of ADSS0"]
    #[inline(always)]
    pub fn ts5(&self) -> TS5_R {
        TS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of ADSS0"]
    #[inline(always)]
    pub fn ts6(&self) -> TS6_R {
        TS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of ADSS0"]
    #[inline(always)]
    pub fn ts7(&self) -> TS7_R {
        TS7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of ADSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ts0(&mut self) -> TS0_W<ADSS0_SPEC> {
        TS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of ADSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<ADSS0_SPEC> {
        TS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of ADSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<ADSS0_SPEC> {
        TS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of ADSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ts3(&mut self) -> TS3_W<ADSS0_SPEC> {
        TS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of ADSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ts4(&mut self) -> TS4_W<ADSS0_SPEC> {
        TS4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of ADSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ts5(&mut self) -> TS5_W<ADSS0_SPEC> {
        TS5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of ADSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ts6(&mut self) -> TS6_W<ADSS0_SPEC> {
        TS6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of ADSS0"]
    #[inline(always)]
    #[must_use]
    pub fn ts7(&mut self) -> TS7_W<ADSS0_SPEC> {
        TS7_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sampling Time Selection Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adss0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adss0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADSS0_SPEC;
impl crate::RegisterSpec for ADSS0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adss0::R`](R) reader structure"]
impl crate::Readable for ADSS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adss0::W`](W) writer structure"]
impl crate::Writable for ADSS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADSS0 to value 0"]
impl crate::Resettable for ADSS0_SPEC {
    const RESET_VALUE: u8 = 0;
}
