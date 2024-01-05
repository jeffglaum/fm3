#[doc = "Register `EP0OS` reader"]
pub type R = crate::R<EP0OS_SPEC>;
#[doc = "Register `EP0OS` writer"]
pub type W = crate::W<EP0OS_SPEC>;
#[doc = "Field `SIZE` reader - Packet Size Indication bit"]
pub type SIZE_R = crate::FieldReader;
#[doc = "Field `SPK` reader - Short Packet Interrupt Request bit"]
pub type SPK_R = crate::BitReader;
#[doc = "Field `SPK` writer - Short Packet Interrupt Request bit"]
pub type SPK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQO` reader - Receive Data Interrupt Request bit"]
pub type DRQO_R = crate::BitReader;
#[doc = "Field `DRQO` writer - Receive Data Interrupt Request bit"]
pub type DRQO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPKIE` reader - Short Packet Interrupt Enable bit"]
pub type SPKIE_R = crate::BitReader;
#[doc = "Field `SPKIE` writer - Short Packet Interrupt Enable bit"]
pub type SPKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQOIE` reader - Receive Data Interrupt Enable bit"]
pub type DRQOIE_R = crate::BitReader;
#[doc = "Field `DRQOIE` writer - Receive Data Interrupt Enable bit"]
pub type DRQOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFINI` reader - Receive Buffer Initialization bit"]
pub type BFINI_R = crate::BitReader;
#[doc = "Field `BFINI` writer - Receive Buffer Initialization bit"]
pub type BFINI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Packet Size Indication bit"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Short Packet Interrupt Request bit"]
    #[inline(always)]
    pub fn spk(&self) -> SPK_R {
        SPK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Data Interrupt Request bit"]
    #[inline(always)]
    pub fn drqo(&self) -> DRQO_R {
        DRQO_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Short Packet Interrupt Enable bit"]
    #[inline(always)]
    pub fn spkie(&self) -> SPKIE_R {
        SPKIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Data Interrupt Enable bit"]
    #[inline(always)]
    pub fn drqoie(&self) -> DRQOIE_R {
        DRQOIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Receive Buffer Initialization bit"]
    #[inline(always)]
    pub fn bfini(&self) -> BFINI_R {
        BFINI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Short Packet Interrupt Request bit"]
    #[inline(always)]
    #[must_use]
    pub fn spk(&mut self) -> SPK_W<EP0OS_SPEC> {
        SPK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Receive Data Interrupt Request bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqo(&mut self) -> DRQO_W<EP0OS_SPEC> {
        DRQO_W::new(self, 10)
    }
    #[doc = "Bit 13 - Short Packet Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn spkie(&mut self) -> SPKIE_W<EP0OS_SPEC> {
        SPKIE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Data Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqoie(&mut self) -> DRQOIE_W<EP0OS_SPEC> {
        DRQOIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Receive Buffer Initialization bit"]
    #[inline(always)]
    #[must_use]
    pub fn bfini(&mut self) -> BFINI_W<EP0OS_SPEC> {
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
#[doc = "EP0O Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0os::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0os::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP0OS_SPEC;
impl crate::RegisterSpec for EP0OS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep0os::R`](R) reader structure"]
impl crate::Readable for EP0OS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep0os::W`](W) writer structure"]
impl crate::Writable for EP0OS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EP0OS to value 0x8000"]
impl crate::Resettable for EP0OS_SPEC {
    const RESET_VALUE: u16 = 0x8000;
}
