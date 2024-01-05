#[doc = "Register `EP2S` reader"]
pub type R = crate::R<EP2S_SPEC>;
#[doc = "Register `EP2S` writer"]
pub type W = crate::W<EP2S_SPEC>;
#[doc = "Field `SIZE` reader - packet SIZE"]
pub type SIZE_R = crate::FieldReader;
#[doc = "Field `SPK` reader - Short Packet Interrupt Request bit"]
pub type SPK_R = crate::BitReader;
#[doc = "Field `SPK` writer - Short Packet Interrupt Request bit"]
pub type SPK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQ` reader - Packet Transfer Interrupt Request bit"]
pub type DRQ_R = crate::BitReader;
#[doc = "Field `DRQ` writer - Packet Transfer Interrupt Request bit"]
pub type DRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - Busy Flag bit"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `SPKIE` reader - Short Packet Interrupt Enable bit"]
pub type SPKIE_R = crate::BitReader;
#[doc = "Field `SPKIE` writer - Short Packet Interrupt Enable bit"]
pub type SPKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQIE` reader - Packet Transfer Interrupt Enable bit"]
pub type DRQIE_R = crate::BitReader;
#[doc = "Field `DRQIE` writer - Packet Transfer Interrupt Enable bit"]
pub type DRQIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFINI` reader - Send/Receive Buffer Initialization bit"]
pub type BFINI_R = crate::BitReader;
#[doc = "Field `BFINI` writer - Send/Receive Buffer Initialization bit"]
pub type BFINI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - packet SIZE"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Short Packet Interrupt Request bit"]
    #[inline(always)]
    pub fn spk(&self) -> SPK_R {
        SPK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Packet Transfer Interrupt Request bit"]
    #[inline(always)]
    pub fn drq(&self) -> DRQ_R {
        DRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Busy Flag bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Short Packet Interrupt Enable bit"]
    #[inline(always)]
    pub fn spkie(&self) -> SPKIE_R {
        SPKIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Packet Transfer Interrupt Enable bit"]
    #[inline(always)]
    pub fn drqie(&self) -> DRQIE_R {
        DRQIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Send/Receive Buffer Initialization bit"]
    #[inline(always)]
    pub fn bfini(&self) -> BFINI_R {
        BFINI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Short Packet Interrupt Request bit"]
    #[inline(always)]
    #[must_use]
    pub fn spk(&mut self) -> SPK_W<EP2S_SPEC> {
        SPK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Packet Transfer Interrupt Request bit"]
    #[inline(always)]
    #[must_use]
    pub fn drq(&mut self) -> DRQ_W<EP2S_SPEC> {
        DRQ_W::new(self, 10)
    }
    #[doc = "Bit 13 - Short Packet Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn spkie(&mut self) -> SPKIE_W<EP2S_SPEC> {
        SPKIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Packet Transfer Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqie(&mut self) -> DRQIE_W<EP2S_SPEC> {
        DRQIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Send/Receive Buffer Initialization bit"]
    #[inline(always)]
    #[must_use]
    pub fn bfini(&mut self) -> BFINI_W<EP2S_SPEC> {
        BFINI_W::new(self, 15)
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
#[doc = "EP2 Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep2s::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep2s::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP2S_SPEC;
impl crate::RegisterSpec for EP2S_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep2s::R`](R) reader structure"]
impl crate::Readable for EP2S_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep2s::W`](W) writer structure"]
impl crate::Writable for EP2S_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EP2S to value 0x8000"]
impl crate::Resettable for EP2S_SPEC {
    const RESET_VALUE: u16 = 0x8000;
}
