#[doc = "Register `PFRD` reader"]
pub type R = crate::R<PFRD_SPEC>;
#[doc = "Register `PFRD` writer"]
pub type W = crate::W<PFRD_SPEC>;
#[doc = "Field `PD0` reader - Bit0 of PFRD"]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Bit0 of PFRD"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Bit1 of PFRD"]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Bit1 of PFRD"]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Bit2 of PFRD"]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Bit2 of PFRD"]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Bit3 of PFRD"]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - Bit3 of PFRD"]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFRD"]
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRD"]
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFRD"]
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFRD"]
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFRD"]
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<PFRD_SPEC> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRD"]
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<PFRD_SPEC> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFRD"]
    #[inline(always)]
    #[must_use]
    pub fn pd2(&mut self) -> PD2_W<PFRD_SPEC> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFRD"]
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<PFRD_SPEC> {
        PD3_W::new(self, 3)
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
#[doc = "Port function setting register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFRD_SPEC;
impl crate::RegisterSpec for PFRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfrd::R`](R) reader structure"]
impl crate::Readable for PFRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfrd::W`](W) writer structure"]
impl crate::Writable for PFRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFRD to value 0"]
impl crate::Resettable for PFRD_SPEC {
    const RESET_VALUE: u32 = 0;
}
