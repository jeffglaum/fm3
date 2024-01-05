#[doc = "Register `PMTR` reader"]
pub type R = crate::R<PMTR_SPEC>;
#[doc = "Register `PMTR` writer"]
pub type W = crate::W<PMTR_SPEC>;
#[doc = "Field `PD` reader - Power Down"]
pub type PD_R = crate::BitReader;
#[doc = "Field `PD` writer - Power Down"]
pub type PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPE` reader - Magic Packet Enable"]
pub type MPE_R = crate::BitReader;
#[doc = "Field `MPE` writer - Magic Packet Enable"]
pub type MPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFE` reader - Wake-Up Frame Enable"]
pub type WFE_R = crate::BitReader;
#[doc = "Field `WFE` writer - Wake-Up Frame Enable"]
pub type WFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPR` reader - Magic Packet Received"]
pub type MPR_R = crate::BitReader;
#[doc = "Field `MPR` writer - Magic Packet Received"]
pub type MPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPR` reader - Wake Up Frame Receive"]
pub type WPR_R = crate::BitReader;
#[doc = "Field `WPR` writer - Wake Up Frame Receive"]
pub type WPR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GU` reader - Global Unicast"]
pub type GU_R = crate::BitReader;
#[doc = "Field `GU` writer - Global Unicast"]
pub type GU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWFFRPR` reader - Remote Wake-up Frame Filter Register Pointer Reset"]
pub type RWFFRPR_R = crate::BitReader;
#[doc = "Field `RWFFRPR` writer - Remote Wake-up Frame Filter Register Pointer Reset"]
pub type RWFFRPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    pub fn mpe(&self) -> MPE_R {
        MPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    pub fn mpr(&self) -> MPR_R {
        MPR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake Up Frame Receive"]
    #[inline(always)]
    pub fn wpr(&self) -> WPR_R {
        WPR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    pub fn gu(&self) -> GU_R {
        GU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Remote Wake-up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    pub fn rwffrpr(&self) -> RWFFRPR_R {
        RWFFRPR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<PMTR_SPEC> {
        PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Magic Packet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpe(&mut self) -> MPE_W<PMTR_SPEC> {
        MPE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-Up Frame Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wfe(&mut self) -> WFE_W<PMTR_SPEC> {
        WFE_W::new(self, 2)
    }
    #[doc = "Bit 5 - Magic Packet Received"]
    #[inline(always)]
    #[must_use]
    pub fn mpr(&mut self) -> MPR_W<PMTR_SPEC> {
        MPR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Wake Up Frame Receive"]
    #[inline(always)]
    #[must_use]
    pub fn wpr(&mut self) -> WPR_W<PMTR_SPEC> {
        WPR_W::new(self, 6)
    }
    #[doc = "Bit 9 - Global Unicast"]
    #[inline(always)]
    #[must_use]
    pub fn gu(&mut self) -> GU_W<PMTR_SPEC> {
        GU_W::new(self, 9)
    }
    #[doc = "Bit 31 - Remote Wake-up Frame Filter Register Pointer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rwffrpr(&mut self) -> RWFFRPR_W<PMTR_SPEC> {
        RWFFRPR_W::new(self, 31)
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
#[doc = "PMT Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMTR_SPEC;
impl crate::RegisterSpec for PMTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmtr::R`](R) reader structure"]
impl crate::Readable for PMTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pmtr::W`](W) writer structure"]
impl crate::Writable for PMTR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMTR to value 0"]
impl crate::Resettable for PMTR_SPEC {
    const RESET_VALUE: u32 = 0;
}
