#[doc = "Register `MFFR` reader"]
pub type R = crate::R<MFFR_SPEC>;
#[doc = "Register `MFFR` writer"]
pub type W = crate::W<MFFR_SPEC>;
#[doc = "Field `PR` reader - Promiscuous Mode"]
pub type PR_R = crate::BitReader;
#[doc = "Field `PR` writer - Promiscuous Mode"]
pub type PR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - Hash Unicast"]
pub type HUC_R = crate::BitReader;
#[doc = "Field `HUC` writer - Hash Unicast"]
pub type HUC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - Hash Multicast"]
pub type HMC_R = crate::BitReader;
#[doc = "Field `HMC` writer - Hash Multicast"]
pub type HMC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - DA Inverse Filtering"]
pub type DAIF_R = crate::BitReader;
#[doc = "Field `DAIF` writer - DA Inverse Filtering"]
pub type DAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - Pass All Multicast"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - Pass All Multicast"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DB` reader - Disable Broadcast Frames"]
pub type DB_R = crate::BitReader;
#[doc = "Field `DB` writer - Disable Broadcast Frames"]
pub type DB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Pass Control Frames"]
pub type PCF_R = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass Control Frames"]
pub type PCF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - Source Address Inverse Filter"]
pub type SAIF_R = crate::BitReader;
#[doc = "Field `SAIF` writer - Source Address Inverse Filter"]
pub type SAIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source Address Filter"]
pub type SAF_R = crate::BitReader;
#[doc = "Field `SAF` writer - Source Address Filter"]
pub type SAF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - Hash or Perfect Filter"]
pub type HPF_R = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or Perfect Filter"]
pub type HPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - Receive All"]
pub type RA_R = crate::BitReader;
#[doc = "Field `RA` writer - Receive All"]
pub type RA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HUC_R {
        HUC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HMC_R {
        HMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DAIF_R {
        DAIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    pub fn db(&self) -> DB_R {
        DB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PCF_R {
        PCF_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source Address Inverse Filter"]
    #[inline(always)]
    pub fn saif(&self) -> SAIF_R {
        SAIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source Address Filter"]
    #[inline(always)]
    pub fn saf(&self) -> SAF_R {
        SAF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HPF_R {
        HPF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<MFFR_SPEC> {
        PR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Hash Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HUC_W<MFFR_SPEC> {
        HUC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Hash Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HMC_W<MFFR_SPEC> {
        HMC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DA Inverse Filtering"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DAIF_W<MFFR_SPEC> {
        DAIF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Pass All Multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<MFFR_SPEC> {
        PM_W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Broadcast Frames"]
    #[inline(always)]
    #[must_use]
    pub fn db(&mut self) -> DB_W<MFFR_SPEC> {
        DB_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass Control Frames"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PCF_W<MFFR_SPEC> {
        PCF_W::new(self, 6)
    }
    #[doc = "Bit 8 - Source Address Inverse Filter"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SAIF_W<MFFR_SPEC> {
        SAIF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Source Address Filter"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SAF_W<MFFR_SPEC> {
        SAF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or Perfect Filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HPF_W<MFFR_SPEC> {
        HPF_W::new(self, 10)
    }
    #[doc = "Bit 31 - Receive All"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<MFFR_SPEC> {
        RA_W::new(self, 31)
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
#[doc = "MAC Frame Filter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mffr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mffr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MFFR_SPEC;
impl crate::RegisterSpec for MFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mffr::R`](R) reader structure"]
impl crate::Readable for MFFR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mffr::W`](W) writer structure"]
impl crate::Writable for MFFR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MFFR to value 0"]
impl crate::Resettable for MFFR_SPEC {
    const RESET_VALUE: u32 = 0;
}
