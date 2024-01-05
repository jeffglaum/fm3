#[doc = "Register `EP0IS` reader"]
pub type R = crate::R<EP0IS_SPEC>;
#[doc = "Register `EP0IS` writer"]
pub type W = crate::W<EP0IS_SPEC>;
#[doc = "Field `DRQI` reader - Send/Receive Data Interrupt Request bit"]
pub type DRQI_R = crate::BitReader;
#[doc = "Field `DRQI` writer - Send/Receive Data Interrupt Request bit"]
pub type DRQI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQIIE` reader - Send Data Interrupt Enable bit"]
pub type DRQIIE_R = crate::BitReader;
#[doc = "Field `DRQIIE` writer - Send Data Interrupt Enable bit"]
pub type DRQIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFINI` reader - Send Buffer Initialization bit"]
pub type BFINI_R = crate::BitReader;
#[doc = "Field `BFINI` writer - Send Buffer Initialization bit"]
pub type BFINI_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - Send/Receive Data Interrupt Request bit"]
    #[inline(always)]
    pub fn drqi(&self) -> DRQI_R {
        DRQI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - Send Data Interrupt Enable bit"]
    #[inline(always)]
    pub fn drqiie(&self) -> DRQIIE_R {
        DRQIIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Send Buffer Initialization bit"]
    #[inline(always)]
    pub fn bfini(&self) -> BFINI_R {
        BFINI_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Send/Receive Data Interrupt Request bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqi(&mut self) -> DRQI_W<EP0IS_SPEC> {
        DRQI_W::new(self, 10)
    }
    #[doc = "Bit 14 - Send Data Interrupt Enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn drqiie(&mut self) -> DRQIIE_W<EP0IS_SPEC> {
        DRQIIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Send Buffer Initialization bit"]
    #[inline(always)]
    #[must_use]
    pub fn bfini(&mut self) -> BFINI_W<EP0IS_SPEC> {
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
#[doc = "EP0I Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep0is::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep0is::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EP0IS_SPEC;
impl crate::RegisterSpec for EP0IS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ep0is::R`](R) reader structure"]
impl crate::Readable for EP0IS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ep0is::W`](W) writer structure"]
impl crate::Writable for EP0IS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EP0IS to value 0x8400"]
impl crate::Resettable for EP0IS_SPEC {
    const RESET_VALUE: u16 = 0x8400;
}
