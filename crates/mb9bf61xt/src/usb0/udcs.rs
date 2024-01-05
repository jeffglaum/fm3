#[doc = "Register `UDCS` reader"]
pub type R = crate::R<UDCS_SPEC>;
#[doc = "Register `UDCS` writer"]
pub type W = crate::W<UDCS_SPEC>;
#[doc = "Field `CONF` reader - Configuration Detection bit"]
pub type CONF_R = crate::BitReader;
#[doc = "Field `CONF` writer - Configuration Detection bit"]
pub type CONF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETP` reader - Setup Stage Detection bit"]
pub type SETP_R = crate::BitReader;
#[doc = "Field `SETP` writer - Setup Stage Detection bit"]
pub type SETP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUP` reader - Wake-up Detection bit"]
pub type WKUP_R = crate::BitReader;
#[doc = "Field `WKUP` writer - Wake-up Detection bit"]
pub type WKUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRST` reader - Bus Reset Detection bit"]
pub type BRST_R = crate::BitReader;
#[doc = "Field `BRST` writer - Bus Reset Detection bit"]
pub type BRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - SOF Detection bit"]
pub type SOF_R = crate::BitReader;
#[doc = "Field `SOF` writer - SOF Detection bit"]
pub type SOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSP` reader - Suspend detection bit"]
pub type SUSP_R = crate::BitReader;
#[doc = "Field `SUSP` writer - Suspend detection bit"]
pub type SUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configuration Detection bit"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setup Stage Detection bit"]
    #[inline(always)]
    pub fn setp(&self) -> SETP_R {
        SETP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up Detection bit"]
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bus Reset Detection bit"]
    #[inline(always)]
    pub fn brst(&self) -> BRST_R {
        BRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SOF Detection bit"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Suspend detection bit"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configuration Detection bit"]
    #[inline(always)]
    #[must_use]
    pub fn conf(&mut self) -> CONF_W<UDCS_SPEC> {
        CONF_W::new(self, 0)
    }
    #[doc = "Bit 1 - Setup Stage Detection bit"]
    #[inline(always)]
    #[must_use]
    pub fn setp(&mut self) -> SETP_W<UDCS_SPEC> {
        SETP_W::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-up Detection bit"]
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<UDCS_SPEC> {
        WKUP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bus Reset Detection bit"]
    #[inline(always)]
    #[must_use]
    pub fn brst(&mut self) -> BRST_W<UDCS_SPEC> {
        BRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - SOF Detection bit"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<UDCS_SPEC> {
        SOF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Suspend detection bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<UDCS_SPEC> {
        SUSP_W::new(self, 5)
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
#[doc = "UDC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udcs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udcs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UDCS_SPEC;
impl crate::RegisterSpec for UDCS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`udcs::R`](R) reader structure"]
impl crate::Readable for UDCS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`udcs::W`](W) writer structure"]
impl crate::Writable for UDCS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UDCS to value 0"]
impl crate::Resettable for UDCS_SPEC {
    const RESET_VALUE: u8 = 0;
}
