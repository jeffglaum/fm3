#[doc = "Register `ADSR` reader"]
pub type R = crate::R<ADSR_SPEC>;
#[doc = "Register `ADSR` writer"]
pub type W = crate::W<ADSR_SPEC>;
#[doc = "Field `SCS` reader - Scan conversion status flag"]
pub type SCS_R = crate::BitReader;
#[doc = "Field `SCS` writer - Scan conversion status flag"]
pub type SCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCS` reader - Priority conversion status flag"]
pub type PCS_R = crate::BitReader;
#[doc = "Field `PCS` writer - Priority conversion status flag"]
pub type PCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNS` reader - Priority conversion pending flag"]
pub type PCNS_R = crate::BitReader;
#[doc = "Field `PCNS` writer - Priority conversion pending flag"]
pub type PCNS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDAS` reader - FIFO data placement selection bit"]
pub type FDAS_R = crate::BitReader;
#[doc = "Field `FDAS` writer - FIFO data placement selection bit"]
pub type FDAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - A/D conversion forced stop bit"]
pub type ADSTP_R = crate::BitReader;
#[doc = "Field `ADSTP` writer - A/D conversion forced stop bit"]
pub type ADSTP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Scan conversion status flag"]
    #[inline(always)]
    pub fn scs(&self) -> SCS_R {
        SCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Priority conversion status flag"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Priority conversion pending flag"]
    #[inline(always)]
    pub fn pcns(&self) -> PCNS_R {
        PCNS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO data placement selection bit"]
    #[inline(always)]
    pub fn fdas(&self) -> FDAS_R {
        FDAS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A/D conversion forced stop bit"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scan conversion status flag"]
    #[inline(always)]
    #[must_use]
    pub fn scs(&mut self) -> SCS_W<ADSR_SPEC> {
        SCS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Priority conversion status flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PCS_W<ADSR_SPEC> {
        PCS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Priority conversion pending flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcns(&mut self) -> PCNS_W<ADSR_SPEC> {
        PCNS_W::new(self, 2)
    }
    #[doc = "Bit 6 - FIFO data placement selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn fdas(&mut self) -> FDAS_W<ADSR_SPEC> {
        FDAS_W::new(self, 6)
    }
    #[doc = "Bit 7 - A/D conversion forced stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<ADSR_SPEC> {
        ADSTP_W::new(self, 7)
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
#[doc = "A/D Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADSR_SPEC;
impl crate::RegisterSpec for ADSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adsr::R`](R) reader structure"]
impl crate::Readable for ADSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adsr::W`](W) writer structure"]
impl crate::Writable for ADSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADSR to value 0"]
impl crate::Resettable for ADSR_SPEC {
    const RESET_VALUE: u8 = 0;
}
