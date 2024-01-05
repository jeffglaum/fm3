#[doc = "Register `PFR3` reader"]
pub type R = crate::R<PFR3_SPEC>;
#[doc = "Register `PFR3` writer"]
pub type W = crate::W<PFR3_SPEC>;
#[doc = "Field `P30` reader - Bit0 of PFR3"]
pub type P30_R = crate::BitReader;
#[doc = "Field `P30` writer - Bit0 of PFR3"]
pub type P30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P31` reader - Bit1 of PFR3"]
pub type P31_R = crate::BitReader;
#[doc = "Field `P31` writer - Bit1 of PFR3"]
pub type P31_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P32` reader - Bit2 of PFR3"]
pub type P32_R = crate::BitReader;
#[doc = "Field `P32` writer - Bit2 of PFR3"]
pub type P32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P33` reader - Bit3 of PFR3"]
pub type P33_R = crate::BitReader;
#[doc = "Field `P33` writer - Bit3 of PFR3"]
pub type P33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P34` reader - Bit4 of PFR3"]
pub type P34_R = crate::BitReader;
#[doc = "Field `P34` writer - Bit4 of PFR3"]
pub type P34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P35` reader - Bit5 of PFR3"]
pub type P35_R = crate::BitReader;
#[doc = "Field `P35` writer - Bit5 of PFR3"]
pub type P35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P36` reader - Bit6 of PFR3"]
pub type P36_R = crate::BitReader;
#[doc = "Field `P36` writer - Bit6 of PFR3"]
pub type P36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P37` reader - Bit7 of PFR3"]
pub type P37_R = crate::BitReader;
#[doc = "Field `P37` writer - Bit7 of PFR3"]
pub type P37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P38` reader - Bit8 of PFR3"]
pub type P38_R = crate::BitReader;
#[doc = "Field `P38` writer - Bit8 of PFR3"]
pub type P38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P39` reader - Bit9 of PFR3"]
pub type P39_R = crate::BitReader;
#[doc = "Field `P39` writer - Bit9 of PFR3"]
pub type P39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3A` reader - Bit10 of PFR3"]
pub type P3A_R = crate::BitReader;
#[doc = "Field `P3A` writer - Bit10 of PFR3"]
pub type P3A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3B` reader - Bit11 of PFR3"]
pub type P3B_R = crate::BitReader;
#[doc = "Field `P3B` writer - Bit11 of PFR3"]
pub type P3B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3C` reader - Bit12 of PFR3"]
pub type P3C_R = crate::BitReader;
#[doc = "Field `P3C` writer - Bit12 of PFR3"]
pub type P3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3D` reader - Bit13 of PFR3"]
pub type P3D_R = crate::BitReader;
#[doc = "Field `P3D` writer - Bit13 of PFR3"]
pub type P3D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3E` reader - Bit14 of PFR3"]
pub type P3E_R = crate::BitReader;
#[doc = "Field `P3E` writer - Bit14 of PFR3"]
pub type P3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P3F` reader - Bit15 of PFR3"]
pub type P3F_R = crate::BitReader;
#[doc = "Field `P3F` writer - Bit15 of PFR3"]
pub type P3F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR3"]
    #[inline(always)]
    pub fn p30(&self) -> P30_R {
        P30_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR3"]
    #[inline(always)]
    pub fn p31(&self) -> P31_R {
        P31_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR3"]
    #[inline(always)]
    pub fn p32(&self) -> P32_R {
        P32_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR3"]
    #[inline(always)]
    pub fn p33(&self) -> P33_R {
        P33_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFR3"]
    #[inline(always)]
    pub fn p34(&self) -> P34_R {
        P34_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFR3"]
    #[inline(always)]
    pub fn p35(&self) -> P35_R {
        P35_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFR3"]
    #[inline(always)]
    pub fn p36(&self) -> P36_R {
        P36_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFR3"]
    #[inline(always)]
    pub fn p37(&self) -> P37_R {
        P37_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of PFR3"]
    #[inline(always)]
    pub fn p38(&self) -> P38_R {
        P38_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of PFR3"]
    #[inline(always)]
    pub fn p39(&self) -> P39_R {
        P39_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit10 of PFR3"]
    #[inline(always)]
    pub fn p3a(&self) -> P3A_R {
        P3A_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit11 of PFR3"]
    #[inline(always)]
    pub fn p3b(&self) -> P3B_R {
        P3B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bit12 of PFR3"]
    #[inline(always)]
    pub fn p3c(&self) -> P3C_R {
        P3C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit13 of PFR3"]
    #[inline(always)]
    pub fn p3d(&self) -> P3D_R {
        P3D_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit14 of PFR3"]
    #[inline(always)]
    pub fn p3e(&self) -> P3E_R {
        P3E_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bit15 of PFR3"]
    #[inline(always)]
    pub fn p3f(&self) -> P3F_R {
        P3F_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<PFR3_SPEC> {
        P30_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<PFR3_SPEC> {
        P31_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p32(&mut self) -> P32_W<PFR3_SPEC> {
        P32_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p33(&mut self) -> P33_W<PFR3_SPEC> {
        P33_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p34(&mut self) -> P34_W<PFR3_SPEC> {
        P34_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p35(&mut self) -> P35_W<PFR3_SPEC> {
        P35_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p36(&mut self) -> P36_W<PFR3_SPEC> {
        P36_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p37(&mut self) -> P37_W<PFR3_SPEC> {
        P37_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p38(&mut self) -> P38_W<PFR3_SPEC> {
        P38_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p39(&mut self) -> P39_W<PFR3_SPEC> {
        P39_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bit10 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p3a(&mut self) -> P3A_W<PFR3_SPEC> {
        P3A_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit11 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p3b(&mut self) -> P3B_W<PFR3_SPEC> {
        P3B_W::new(self, 11)
    }
    #[doc = "Bit 12 - Bit12 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p3c(&mut self) -> P3C_W<PFR3_SPEC> {
        P3C_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bit13 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p3d(&mut self) -> P3D_W<PFR3_SPEC> {
        P3D_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bit14 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p3e(&mut self) -> P3E_W<PFR3_SPEC> {
        P3E_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bit15 of PFR3"]
    #[inline(always)]
    #[must_use]
    pub fn p3f(&mut self) -> P3F_W<PFR3_SPEC> {
        P3F_W::new(self, 15)
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
#[doc = "Port function setting register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR3_SPEC;
impl crate::RegisterSpec for PFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr3::R`](R) reader structure"]
impl crate::Readable for PFR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr3::W`](W) writer structure"]
impl crate::Writable for PFR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR3 to value 0"]
impl crate::Resettable for PFR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
