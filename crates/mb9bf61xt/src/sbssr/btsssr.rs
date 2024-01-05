#[doc = "Register `BTSSSR` writer"]
pub type W = crate::W<BTSSSR_SPEC>;
#[doc = "Field `SSSR0` writer - Bit0 of BTSSSR"]
pub type SSSR0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR1` writer - Bit1 of BTSSSR"]
pub type SSSR1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR2` writer - Bit2 of BTSSSR"]
pub type SSSR2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR3` writer - Bit3 of BTSSSR"]
pub type SSSR3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR4` writer - Bit4 of BTSSSR"]
pub type SSSR4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR5` writer - Bit5 of BTSSSR"]
pub type SSSR5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR6` writer - Bit6 of BTSSSR"]
pub type SSSR6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR7` writer - Bit7 of BTSSSR"]
pub type SSSR7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR8` writer - Bit8 of BTSSSR"]
pub type SSSR8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR9` writer - Bit9 of BTSSSR"]
pub type SSSR9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR10` writer - Bit10 of BTSSSR"]
pub type SSSR10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR11` writer - Bit11 of BTSSSR"]
pub type SSSR11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR12` writer - Bit12 of BTSSSR"]
pub type SSSR12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR13` writer - Bit13 of BTSSSR"]
pub type SSSR13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR14` writer - Bit14 of BTSSSR"]
pub type SSSR14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSSR15` writer - Bit15 of BTSSSR"]
pub type SSSR15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Bit0 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr0(&mut self) -> SSSR0_W<BTSSSR_SPEC> {
        SSSR0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr1(&mut self) -> SSSR1_W<BTSSSR_SPEC> {
        SSSR1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr2(&mut self) -> SSSR2_W<BTSSSR_SPEC> {
        SSSR2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr3(&mut self) -> SSSR3_W<BTSSSR_SPEC> {
        SSSR3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr4(&mut self) -> SSSR4_W<BTSSSR_SPEC> {
        SSSR4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr5(&mut self) -> SSSR5_W<BTSSSR_SPEC> {
        SSSR5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr6(&mut self) -> SSSR6_W<BTSSSR_SPEC> {
        SSSR6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr7(&mut self) -> SSSR7_W<BTSSSR_SPEC> {
        SSSR7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr8(&mut self) -> SSSR8_W<BTSSSR_SPEC> {
        SSSR8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr9(&mut self) -> SSSR9_W<BTSSSR_SPEC> {
        SSSR9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bit10 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr10(&mut self) -> SSSR10_W<BTSSSR_SPEC> {
        SSSR10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit11 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr11(&mut self) -> SSSR11_W<BTSSSR_SPEC> {
        SSSR11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Bit12 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr12(&mut self) -> SSSR12_W<BTSSSR_SPEC> {
        SSSR12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bit13 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr13(&mut self) -> SSSR13_W<BTSSSR_SPEC> {
        SSSR13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bit14 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr14(&mut self) -> SSSR14_W<BTSSSR_SPEC> {
        SSSR14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bit15 of BTSSSR"]
    #[inline(always)]
    #[must_use]
    pub fn sssr15(&mut self) -> SSSR15_W<BTSSSR_SPEC> {
        SSSR15_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software-based Simultaneous Startup Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btsssr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTSSSR_SPEC;
impl crate::RegisterSpec for BTSSSR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`btsssr::W`](W) writer structure"]
impl crate::Writable for BTSSSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BTSSSR to value 0"]
impl crate::Resettable for BTSSSR_SPEC {
    const RESET_VALUE: u16 = 0;
}
