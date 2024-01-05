#[doc = "Register `PFRE` reader"]
pub type R = crate::R<PFRE_SPEC>;
#[doc = "Register `PFRE` writer"]
pub type W = crate::W<PFRE_SPEC>;
#[doc = "Field `PE0` reader - Bit0 of PFRE"]
pub type PE0_R = crate::BitReader;
#[doc = "Field `PE0` writer - Bit0 of PFRE"]
pub type PE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE2` reader - Bit2 of PFRE"]
pub type PE2_R = crate::BitReader;
#[doc = "Field `PE2` writer - Bit2 of PFRE"]
pub type PE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE3` reader - Bit3 of PFRE"]
pub type PE3_R = crate::BitReader;
#[doc = "Field `PE3` writer - Bit3 of PFRE"]
pub type PE3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFRE"]
    #[inline(always)]
    pub fn pe0(&self) -> PE0_R {
        PE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFRE"]
    #[inline(always)]
    pub fn pe2(&self) -> PE2_R {
        PE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFRE"]
    #[inline(always)]
    pub fn pe3(&self) -> PE3_R {
        PE3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFRE"]
    #[inline(always)]
    #[must_use]
    pub fn pe0(&mut self) -> PE0_W<PFRE_SPEC> {
        PE0_W::new(self, 0)
    }
    #[doc = "Bit 2 - Bit2 of PFRE"]
    #[inline(always)]
    #[must_use]
    pub fn pe2(&mut self) -> PE2_W<PFRE_SPEC> {
        PE2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFRE"]
    #[inline(always)]
    #[must_use]
    pub fn pe3(&mut self) -> PE3_W<PFRE_SPEC> {
        PE3_W::new(self, 3)
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
#[doc = "Port function setting register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfre::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfre::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFRE_SPEC;
impl crate::RegisterSpec for PFRE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfre::R`](R) reader structure"]
impl crate::Readable for PFRE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfre::W`](W) writer structure"]
impl crate::Writable for PFRE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFRE to value 0"]
impl crate::Resettable for PFRE_SPEC {
    const RESET_VALUE: u32 = 0;
}
