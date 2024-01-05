#[doc = "Register `PFRA` reader"]
pub type R = crate::R<PFRA_SPEC>;
#[doc = "Register `PFRA` writer"]
pub type W = crate::W<PFRA_SPEC>;
#[doc = "Field `PA0` reader - Bit0 of PFRA"]
pub type PA0_R = crate::BitReader;
#[doc = "Field `PA0` writer - Bit0 of PFRA"]
pub type PA0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA1` reader - Bit1 of PFRA"]
pub type PA1_R = crate::BitReader;
#[doc = "Field `PA1` writer - Bit1 of PFRA"]
pub type PA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA2` reader - Bit2 of PFRA"]
pub type PA2_R = crate::BitReader;
#[doc = "Field `PA2` writer - Bit2 of PFRA"]
pub type PA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA3` reader - Bit3 of PFRA"]
pub type PA3_R = crate::BitReader;
#[doc = "Field `PA3` writer - Bit3 of PFRA"]
pub type PA3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA4` reader - Bit4 of PFRA"]
pub type PA4_R = crate::BitReader;
#[doc = "Field `PA4` writer - Bit4 of PFRA"]
pub type PA4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA5` reader - Bit5 of PFRA"]
pub type PA5_R = crate::BitReader;
#[doc = "Field `PA5` writer - Bit5 of PFRA"]
pub type PA5_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFRA"]
    #[inline(always)]
    pub fn pa0(&self) -> PA0_R {
        PA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRA"]
    #[inline(always)]
    pub fn pa1(&self) -> PA1_R {
        PA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFRA"]
    #[inline(always)]
    pub fn pa2(&self) -> PA2_R {
        PA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFRA"]
    #[inline(always)]
    pub fn pa3(&self) -> PA3_R {
        PA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFRA"]
    #[inline(always)]
    pub fn pa4(&self) -> PA4_R {
        PA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFRA"]
    #[inline(always)]
    pub fn pa5(&self) -> PA5_R {
        PA5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFRA"]
    #[inline(always)]
    #[must_use]
    pub fn pa0(&mut self) -> PA0_W<PFRA_SPEC> {
        PA0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFRA"]
    #[inline(always)]
    #[must_use]
    pub fn pa1(&mut self) -> PA1_W<PFRA_SPEC> {
        PA1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFRA"]
    #[inline(always)]
    #[must_use]
    pub fn pa2(&mut self) -> PA2_W<PFRA_SPEC> {
        PA2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFRA"]
    #[inline(always)]
    #[must_use]
    pub fn pa3(&mut self) -> PA3_W<PFRA_SPEC> {
        PA3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFRA"]
    #[inline(always)]
    #[must_use]
    pub fn pa4(&mut self) -> PA4_W<PFRA_SPEC> {
        PA4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFRA"]
    #[inline(always)]
    #[must_use]
    pub fn pa5(&mut self) -> PA5_W<PFRA_SPEC> {
        PA5_W::new(self, 5)
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
#[doc = "Port function setting register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFRA_SPEC;
impl crate::RegisterSpec for PFRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfra::R`](R) reader structure"]
impl crate::Readable for PFRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfra::W`](W) writer structure"]
impl crate::Writable for PFRA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFRA to value 0"]
impl crate::Resettable for PFRA_SPEC {
    const RESET_VALUE: u32 = 0;
}
