#[doc = "Register `FCR` reader"]
pub type R = crate::R<FCR_SPEC>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FCR_SPEC>;
#[doc = "Field `FCB_BPA` reader - Flow Control Busy/Backpressure Activate"]
pub type FCB_BPA_R = crate::BitReader;
#[doc = "Field `FCB_BPA` writer - Flow Control Busy/Backpressure Activate"]
pub type FCB_BPA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFE` reader - Transmit Flow Control Enable"]
pub type TFE_R = crate::BitReader;
#[doc = "Field `TFE` writer - Transmit Flow Control Enable"]
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFE` reader - Receive Flow Control Enable"]
pub type RFE_R = crate::BitReader;
#[doc = "Field `RFE` writer - Receive Flow Control Enable"]
pub type RFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - Unicast Pause Frame detect"]
pub type UP_R = crate::BitReader;
#[doc = "Field `UP` writer - Unicast Pause Frame detect"]
pub type UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLT` reader - Pause Low Threshold"]
pub type PLT_R = crate::FieldReader;
#[doc = "Field `PLT` writer - Pause Low Threshold"]
pub type PLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause"]
pub type DZPQ_R = crate::BitReader;
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause"]
pub type DZPQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PT` reader - Pause Time"]
pub type PT_R = crate::FieldReader<u16>;
#[doc = "Field `PT` writer - Pause Time"]
pub type PT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate"]
    #[inline(always)]
    pub fn fcb_bpa(&self) -> FCB_BPA_R {
        FCB_BPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Unicast Pause Frame detect"]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate"]
    #[inline(always)]
    #[must_use]
    pub fn fcb_bpa(&mut self) -> FCB_BPA_W<FCR_SPEC> {
        FCB_BPA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<FCR_SPEC> {
        TFE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<FCR_SPEC> {
        RFE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Unicast Pause Frame detect"]
    #[inline(always)]
    #[must_use]
    pub fn up(&mut self) -> UP_W<FCR_SPEC> {
        UP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn plt(&mut self) -> PLT_W<FCR_SPEC> {
        PLT_W::new(self, 4)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    #[must_use]
    pub fn dzpq(&mut self) -> DZPQ_W<FCR_SPEC> {
        DZPQ_W::new(self, 7)
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<FCR_SPEC> {
        PT_W::new(self, 16)
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
#[doc = "Flow Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
