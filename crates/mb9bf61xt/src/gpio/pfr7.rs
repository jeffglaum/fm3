#[doc = "Register `PFR7` reader"]
pub type R = crate::R<PFR7_SPEC>;
#[doc = "Register `PFR7` writer"]
pub type W = crate::W<PFR7_SPEC>;
#[doc = "Field `P70` reader - Bit0 of PFR7"]
pub type P70_R = crate::BitReader;
#[doc = "Field `P70` writer - Bit0 of PFR7"]
pub type P70_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P71` reader - Bit1 of PFR7"]
pub type P71_R = crate::BitReader;
#[doc = "Field `P71` writer - Bit1 of PFR7"]
pub type P71_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P72` reader - Bit2 of PFR7"]
pub type P72_R = crate::BitReader;
#[doc = "Field `P72` writer - Bit2 of PFR7"]
pub type P72_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P73` reader - Bit3 of PFR7"]
pub type P73_R = crate::BitReader;
#[doc = "Field `P73` writer - Bit3 of PFR7"]
pub type P73_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P74` reader - Bit4 of PFR7"]
pub type P74_R = crate::BitReader;
#[doc = "Field `P74` writer - Bit4 of PFR7"]
pub type P74_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P75` reader - Bit5 of PFR7"]
pub type P75_R = crate::BitReader;
#[doc = "Field `P75` writer - Bit5 of PFR7"]
pub type P75_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P76` reader - Bit6 of PFR7"]
pub type P76_R = crate::BitReader;
#[doc = "Field `P76` writer - Bit6 of PFR7"]
pub type P76_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P77` reader - Bit7 of PFR7"]
pub type P77_R = crate::BitReader;
#[doc = "Field `P77` writer - Bit7 of PFR7"]
pub type P77_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P78` reader - Bit8 of PFR7"]
pub type P78_R = crate::BitReader;
#[doc = "Field `P78` writer - Bit8 of PFR7"]
pub type P78_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P79` reader - Bit9 of PFR7"]
pub type P79_R = crate::BitReader;
#[doc = "Field `P79` writer - Bit9 of PFR7"]
pub type P79_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7A` reader - Bit10 of PFR7"]
pub type P7A_R = crate::BitReader;
#[doc = "Field `P7A` writer - Bit10 of PFR7"]
pub type P7A_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7B` reader - Bit11 of PFR7"]
pub type P7B_R = crate::BitReader;
#[doc = "Field `P7B` writer - Bit11 of PFR7"]
pub type P7B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7C` reader - Bit12 of PFR7"]
pub type P7C_R = crate::BitReader;
#[doc = "Field `P7C` writer - Bit12 of PFR7"]
pub type P7C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7D` reader - Bit13 of PFR7"]
pub type P7D_R = crate::BitReader;
#[doc = "Field `P7D` writer - Bit13 of PFR7"]
pub type P7D_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7E` reader - Bit14 of PFR7"]
pub type P7E_R = crate::BitReader;
#[doc = "Field `P7E` writer - Bit14 of PFR7"]
pub type P7E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P7F` reader - Bit15 of PFR7"]
pub type P7F_R = crate::BitReader;
#[doc = "Field `P7F` writer - Bit15 of PFR7"]
pub type P7F_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR7"]
    #[inline(always)]
    pub fn p70(&self) -> P70_R {
        P70_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR7"]
    #[inline(always)]
    pub fn p71(&self) -> P71_R {
        P71_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR7"]
    #[inline(always)]
    pub fn p72(&self) -> P72_R {
        P72_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR7"]
    #[inline(always)]
    pub fn p73(&self) -> P73_R {
        P73_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFR7"]
    #[inline(always)]
    pub fn p74(&self) -> P74_R {
        P74_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFR7"]
    #[inline(always)]
    pub fn p75(&self) -> P75_R {
        P75_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFR7"]
    #[inline(always)]
    pub fn p76(&self) -> P76_R {
        P76_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFR7"]
    #[inline(always)]
    pub fn p77(&self) -> P77_R {
        P77_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of PFR7"]
    #[inline(always)]
    pub fn p78(&self) -> P78_R {
        P78_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of PFR7"]
    #[inline(always)]
    pub fn p79(&self) -> P79_R {
        P79_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit10 of PFR7"]
    #[inline(always)]
    pub fn p7a(&self) -> P7A_R {
        P7A_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit11 of PFR7"]
    #[inline(always)]
    pub fn p7b(&self) -> P7B_R {
        P7B_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bit12 of PFR7"]
    #[inline(always)]
    pub fn p7c(&self) -> P7C_R {
        P7C_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit13 of PFR7"]
    #[inline(always)]
    pub fn p7d(&self) -> P7D_R {
        P7D_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit14 of PFR7"]
    #[inline(always)]
    pub fn p7e(&self) -> P7E_R {
        P7E_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bit15 of PFR7"]
    #[inline(always)]
    pub fn p7f(&self) -> P7F_R {
        P7F_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p70(&mut self) -> P70_W<PFR7_SPEC> {
        P70_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p71(&mut self) -> P71_W<PFR7_SPEC> {
        P71_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p72(&mut self) -> P72_W<PFR7_SPEC> {
        P72_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p73(&mut self) -> P73_W<PFR7_SPEC> {
        P73_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p74(&mut self) -> P74_W<PFR7_SPEC> {
        P74_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p75(&mut self) -> P75_W<PFR7_SPEC> {
        P75_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p76(&mut self) -> P76_W<PFR7_SPEC> {
        P76_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p77(&mut self) -> P77_W<PFR7_SPEC> {
        P77_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p78(&mut self) -> P78_W<PFR7_SPEC> {
        P78_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p79(&mut self) -> P79_W<PFR7_SPEC> {
        P79_W::new(self, 9)
    }
    #[doc = "Bit 10 - Bit10 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p7a(&mut self) -> P7A_W<PFR7_SPEC> {
        P7A_W::new(self, 10)
    }
    #[doc = "Bit 11 - Bit11 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p7b(&mut self) -> P7B_W<PFR7_SPEC> {
        P7B_W::new(self, 11)
    }
    #[doc = "Bit 12 - Bit12 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p7c(&mut self) -> P7C_W<PFR7_SPEC> {
        P7C_W::new(self, 12)
    }
    #[doc = "Bit 13 - Bit13 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p7d(&mut self) -> P7D_W<PFR7_SPEC> {
        P7D_W::new(self, 13)
    }
    #[doc = "Bit 14 - Bit14 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p7e(&mut self) -> P7E_W<PFR7_SPEC> {
        P7E_W::new(self, 14)
    }
    #[doc = "Bit 15 - Bit15 of PFR7"]
    #[inline(always)]
    #[must_use]
    pub fn p7f(&mut self) -> P7F_W<PFR7_SPEC> {
        P7F_W::new(self, 15)
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
#[doc = "Port function setting register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR7_SPEC;
impl crate::RegisterSpec for PFR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr7::R`](R) reader structure"]
impl crate::Readable for PFR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr7::W`](W) writer structure"]
impl crate::Writable for PFR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR7 to value 0"]
impl crate::Resettable for PFR7_SPEC {
    const RESET_VALUE: u32 = 0;
}
