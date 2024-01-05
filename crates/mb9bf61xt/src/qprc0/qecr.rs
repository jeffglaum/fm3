#[doc = "Register `QECR` reader"]
pub type R = crate::R<QECR_SPEC>;
#[doc = "Register `QECR` writer"]
pub type W = crate::W<QECR_SPEC>;
#[doc = "Field `ORNGMD` reader - Outrange mode selection bit"]
pub type ORNGMD_R = crate::BitReader;
#[doc = "Field `ORNGMD` writer - Outrange mode selection bit"]
pub type ORNGMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORNGF` reader - Outrange interrupt request flag bit"]
pub type ORNGF_R = crate::BitReader;
#[doc = "Field `ORNGF` writer - Outrange interrupt request flag bit"]
pub type ORNGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ORNGIE` reader - Outrange interrupt enable bit"]
pub type ORNGIE_R = crate::BitReader;
#[doc = "Field `ORNGIE` writer - Outrange interrupt enable bit"]
pub type ORNGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Outrange mode selection bit"]
    #[inline(always)]
    pub fn orngmd(&self) -> ORNGMD_R {
        ORNGMD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Outrange interrupt request flag bit"]
    #[inline(always)]
    pub fn orngf(&self) -> ORNGF_R {
        ORNGF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Outrange interrupt enable bit"]
    #[inline(always)]
    pub fn orngie(&self) -> ORNGIE_R {
        ORNGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Outrange mode selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn orngmd(&mut self) -> ORNGMD_W<QECR_SPEC> {
        ORNGMD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Outrange interrupt request flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn orngf(&mut self) -> ORNGF_W<QECR_SPEC> {
        ORNGF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Outrange interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn orngie(&mut self) -> ORNGIE_W<QECR_SPEC> {
        ORNGIE_W::new(self, 2)
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
#[doc = "QPRC Extension Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qecr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qecr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QECR_SPEC;
impl crate::RegisterSpec for QECR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qecr::R`](R) reader structure"]
impl crate::Readable for QECR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qecr::W`](W) writer structure"]
impl crate::Writable for QECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets QECR to value 0"]
impl crate::Resettable for QECR_SPEC {
    const RESET_VALUE: u16 = 0;
}
