#[doc = "Register `HERR` reader"]
pub type R = crate::R<HERR_SPEC>;
#[doc = "Register `HERR` writer"]
pub type W = crate::W<HERR_SPEC>;
#[doc = "Field `HS` reader - handshake status flags"]
pub type HS_R = crate::FieldReader;
#[doc = "Field `HS` writer - handshake status flags"]
pub type HS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STUFF` reader - stuffing error flag"]
pub type STUFF_R = crate::BitReader;
#[doc = "Field `STUFF` writer - stuffing error flag"]
pub type STUFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGERR` reader - toggle error flag"]
pub type TGERR_R = crate::BitReader;
#[doc = "Field `TGERR` writer - toggle error flag"]
pub type TGERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC error flag"]
pub type CRC_R = crate::BitReader;
#[doc = "Field `CRC` writer - CRC error flag"]
pub type CRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUT` reader - timeout flag"]
pub type TOUT_R = crate::BitReader;
#[doc = "Field `TOUT` writer - timeout flag"]
pub type TOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RERR` reader - receive error flag"]
pub type RERR_R = crate::BitReader;
#[doc = "Field `RERR` writer - receive error flag"]
pub type RERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTSOF` reader - lost SOF flag"]
pub type LSTSOF_R = crate::BitReader;
#[doc = "Field `LSTSOF` writer - lost SOF flag"]
pub type LSTSOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - handshake status flags"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - stuffing error flag"]
    #[inline(always)]
    pub fn stuff(&self) -> STUFF_R {
        STUFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - toggle error flag"]
    #[inline(always)]
    pub fn tgerr(&self) -> TGERR_R {
        TGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - timeout flag"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - receive error flag"]
    #[inline(always)]
    pub fn rerr(&self) -> RERR_R {
        RERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - lost SOF flag"]
    #[inline(always)]
    pub fn lstsof(&self) -> LSTSOF_R {
        LSTSOF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - handshake status flags"]
    #[inline(always)]
    #[must_use]
    pub fn hs(&mut self) -> HS_W<HERR_SPEC> {
        HS_W::new(self, 0)
    }
    #[doc = "Bit 2 - stuffing error flag"]
    #[inline(always)]
    #[must_use]
    pub fn stuff(&mut self) -> STUFF_W<HERR_SPEC> {
        STUFF_W::new(self, 2)
    }
    #[doc = "Bit 3 - toggle error flag"]
    #[inline(always)]
    #[must_use]
    pub fn tgerr(&mut self) -> TGERR_W<HERR_SPEC> {
        TGERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<HERR_SPEC> {
        CRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - timeout flag"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<HERR_SPEC> {
        TOUT_W::new(self, 5)
    }
    #[doc = "Bit 6 - receive error flag"]
    #[inline(always)]
    #[must_use]
    pub fn rerr(&mut self) -> RERR_W<HERR_SPEC> {
        RERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - lost SOF flag"]
    #[inline(always)]
    #[must_use]
    pub fn lstsof(&mut self) -> LSTSOF_W<HERR_SPEC> {
        LSTSOF_W::new(self, 7)
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
#[doc = "Host Error Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`herr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`herr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HERR_SPEC;
impl crate::RegisterSpec for HERR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`herr::R`](R) reader structure"]
impl crate::Readable for HERR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`herr::W`](W) writer structure"]
impl crate::Writable for HERR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HERR to value 0x03"]
impl crate::Resettable for HERR_SPEC {
    const RESET_VALUE: u8 = 0x03;
}
