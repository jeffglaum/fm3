#[doc = "Register `HIRQ` reader"]
pub type R = crate::R<HIRQ_SPEC>;
#[doc = "Register `HIRQ` writer"]
pub type W = crate::W<HIRQ_SPEC>;
#[doc = "Field `SOFIRQ` reader - SOF starting flag"]
pub type SOFIRQ_R = crate::BitReader;
#[doc = "Field `SOFIRQ` writer - SOF starting flag"]
pub type SOFIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRQ` reader - device disconnection detection flag"]
pub type DIRQ_R = crate::BitReader;
#[doc = "Field `DIRQ` writer - device disconnection detection flag"]
pub type DIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNNIRQ` reader - device connection detection flag"]
pub type CNNIRQ_R = crate::BitReader;
#[doc = "Field `CNNIRQ` writer - device connection detection flag"]
pub type CNNIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPIRQ` reader - token completion flag"]
pub type CMPIRQ_R = crate::BitReader;
#[doc = "Field `CMPIRQ` writer - token completion flag"]
pub type CMPIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URIRQ` reader - bus reset end flag"]
pub type URIRQ_R = crate::BitReader;
#[doc = "Field `URIRQ` writer - bus reset end flag"]
pub type URIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWKIRQ` reader - remote Wake-up end flag"]
pub type RWKIRQ_R = crate::BitReader;
#[doc = "Field `RWKIRQ` writer - remote Wake-up end flag"]
pub type RWKIRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCAN` reader - token cancellation flag"]
pub type TCAN_R = crate::BitReader;
#[doc = "Field `TCAN` writer - token cancellation flag"]
pub type TCAN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SOF starting flag"]
    #[inline(always)]
    pub fn sofirq(&self) -> SOFIRQ_R {
        SOFIRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - device disconnection detection flag"]
    #[inline(always)]
    pub fn dirq(&self) -> DIRQ_R {
        DIRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - device connection detection flag"]
    #[inline(always)]
    pub fn cnnirq(&self) -> CNNIRQ_R {
        CNNIRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - token completion flag"]
    #[inline(always)]
    pub fn cmpirq(&self) -> CMPIRQ_R {
        CMPIRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - bus reset end flag"]
    #[inline(always)]
    pub fn urirq(&self) -> URIRQ_R {
        URIRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - remote Wake-up end flag"]
    #[inline(always)]
    pub fn rwkirq(&self) -> RWKIRQ_R {
        RWKIRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - token cancellation flag"]
    #[inline(always)]
    pub fn tcan(&self) -> TCAN_R {
        TCAN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOF starting flag"]
    #[inline(always)]
    #[must_use]
    pub fn sofirq(&mut self) -> SOFIRQ_W<HIRQ_SPEC> {
        SOFIRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - device disconnection detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn dirq(&mut self) -> DIRQ_W<HIRQ_SPEC> {
        DIRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - device connection detection flag"]
    #[inline(always)]
    #[must_use]
    pub fn cnnirq(&mut self) -> CNNIRQ_W<HIRQ_SPEC> {
        CNNIRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - token completion flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmpirq(&mut self) -> CMPIRQ_W<HIRQ_SPEC> {
        CMPIRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - bus reset end flag"]
    #[inline(always)]
    #[must_use]
    pub fn urirq(&mut self) -> URIRQ_W<HIRQ_SPEC> {
        URIRQ_W::new(self, 4)
    }
    #[doc = "Bit 5 - remote Wake-up end flag"]
    #[inline(always)]
    #[must_use]
    pub fn rwkirq(&mut self) -> RWKIRQ_W<HIRQ_SPEC> {
        RWKIRQ_W::new(self, 5)
    }
    #[doc = "Bit 7 - token cancellation flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcan(&mut self) -> TCAN_W<HIRQ_SPEC> {
        TCAN_W::new(self, 7)
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
#[doc = "Host Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hirq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hirq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIRQ_SPEC;
impl crate::RegisterSpec for HIRQ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hirq::R`](R) reader structure"]
impl crate::Readable for HIRQ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hirq::W`](W) writer structure"]
impl crate::Writable for HIRQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIRQ to value 0"]
impl crate::Resettable for HIRQ_SPEC {
    const RESET_VALUE: u8 = 0;
}
