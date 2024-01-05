#[doc = "Register `HCNT` reader"]
pub type R = crate::R<HCNT_SPEC>;
#[doc = "Register `HCNT` writer"]
pub type W = crate::W<HCNT_SPEC>;
#[doc = "Field `HOST` reader - host mode bit"]
pub type HOST_R = crate::BitReader;
#[doc = "Field `HOST` writer - host mode bit"]
pub type HOST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URST` reader - bus reset bit"]
pub type URST_R = crate::BitReader;
#[doc = "Field `URST` writer - bus reset bit"]
pub type URST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIRE` reader - SOF interrupt enable bit"]
pub type SOFIRE_R = crate::BitReader;
#[doc = "Field `SOFIRE` writer - SOF interrupt enable bit"]
pub type SOFIRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRE` reader - device disconnection detection interrupt enable bit"]
pub type DIRE_R = crate::BitReader;
#[doc = "Field `DIRE` writer - device disconnection detection interrupt enable bit"]
pub type DIRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNNIRE` reader - device connection detection interrupt enable bit"]
pub type CNNIRE_R = crate::BitReader;
#[doc = "Field `CNNIRE` writer - device connection detection interrupt enable bit"]
pub type CNNIRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIRE` reader - token completion interrupt enable bit"]
pub type CMPIRE_R = crate::BitReader;
#[doc = "Field `CMPIRE` writer - token completion interrupt enable bit"]
pub type CMPIRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URIRE` reader - bus reset interrupt enable bit"]
pub type URIRE_R = crate::BitReader;
#[doc = "Field `URIRE` writer - bus reset interrupt enable bit"]
pub type URIRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKIRE` reader - resume interrupt enable bit"]
pub type RWKIRE_R = crate::BitReader;
#[doc = "Field `RWKIRE` writer - resume interrupt enable bit"]
pub type RWKIRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETRY` reader - retry enable bit"]
pub type RETRY_R = crate::BitReader;
#[doc = "Field `RETRY` writer - retry enable bit"]
pub type RETRY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANCEL` reader - token cancellation enable bit"]
pub type CANCEL_R = crate::BitReader;
#[doc = "Field `CANCEL` writer - token cancellation enable bit"]
pub type CANCEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFSTEP` reader - SOF interrupt occurrence selection bit"]
pub type SOFSTEP_R = crate::BitReader;
#[doc = "Field `SOFSTEP` writer - SOF interrupt occurrence selection bit"]
pub type SOFSTEP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - host mode bit"]
    #[inline(always)]
    pub fn host(&self) -> HOST_R {
        HOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - bus reset bit"]
    #[inline(always)]
    pub fn urst(&self) -> URST_R {
        URST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOF interrupt enable bit"]
    #[inline(always)]
    pub fn sofire(&self) -> SOFIRE_R {
        SOFIRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - device disconnection detection interrupt enable bit"]
    #[inline(always)]
    pub fn dire(&self) -> DIRE_R {
        DIRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - device connection detection interrupt enable bit"]
    #[inline(always)]
    pub fn cnnire(&self) -> CNNIRE_R {
        CNNIRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - token completion interrupt enable bit"]
    #[inline(always)]
    pub fn cmpire(&self) -> CMPIRE_R {
        CMPIRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - bus reset interrupt enable bit"]
    #[inline(always)]
    pub fn urire(&self) -> URIRE_R {
        URIRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - resume interrupt enable bit"]
    #[inline(always)]
    pub fn rwkire(&self) -> RWKIRE_R {
        RWKIRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - retry enable bit"]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - token cancellation enable bit"]
    #[inline(always)]
    pub fn cancel(&self) -> CANCEL_R {
        CANCEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SOF interrupt occurrence selection bit"]
    #[inline(always)]
    pub fn sofstep(&self) -> SOFSTEP_R {
        SOFSTEP_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - host mode bit"]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HOST_W<HCNT_SPEC> {
        HOST_W::new(self, 0)
    }
    #[doc = "Bit 1 - bus reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn urst(&mut self) -> URST_W<HCNT_SPEC> {
        URST_W::new(self, 1)
    }
    #[doc = "Bit 2 - SOF interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn sofire(&mut self) -> SOFIRE_W<HCNT_SPEC> {
        SOFIRE_W::new(self, 2)
    }
    #[doc = "Bit 3 - device disconnection detection interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn dire(&mut self) -> DIRE_W<HCNT_SPEC> {
        DIRE_W::new(self, 3)
    }
    #[doc = "Bit 4 - device connection detection interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cnnire(&mut self) -> CNNIRE_W<HCNT_SPEC> {
        CNNIRE_W::new(self, 4)
    }
    #[doc = "Bit 5 - token completion interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmpire(&mut self) -> CMPIRE_W<HCNT_SPEC> {
        CMPIRE_W::new(self, 5)
    }
    #[doc = "Bit 6 - bus reset interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn urire(&mut self) -> URIRE_W<HCNT_SPEC> {
        URIRE_W::new(self, 6)
    }
    #[doc = "Bit 7 - resume interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rwkire(&mut self) -> RWKIRE_W<HCNT_SPEC> {
        RWKIRE_W::new(self, 7)
    }
    #[doc = "Bit 8 - retry enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn retry(&mut self) -> RETRY_W<HCNT_SPEC> {
        RETRY_W::new(self, 8)
    }
    #[doc = "Bit 9 - token cancellation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cancel(&mut self) -> CANCEL_W<HCNT_SPEC> {
        CANCEL_W::new(self, 9)
    }
    #[doc = "Bit 10 - SOF interrupt occurrence selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn sofstep(&mut self) -> SOFSTEP_W<HCNT_SPEC> {
        SOFSTEP_W::new(self, 10)
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
#[doc = "Host Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCNT_SPEC;
impl crate::RegisterSpec for HCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hcnt::R`](R) reader structure"]
impl crate::Readable for HCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcnt::W`](W) writer structure"]
impl crate::Writable for HCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HCNT to value 0x0100"]
impl crate::Resettable for HCNT_SPEC {
    const RESET_VALUE: u16 = 0x0100;
}
