#[doc = "Register `LIN_SCR` reader"]
pub type R = crate::R<LIN_LIN_SCR_SPEC>;
#[doc = "Register `LIN_SCR` writer"]
pub type W = crate::W<LIN_LIN_SCR_SPEC>;
#[doc = "Field `TXE` reader - Data transmission enable bit"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - Data transmission enable bit"]
pub type TXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - Data reception enable bit"]
pub type RXE_R = crate::BitReader;
#[doc = "Field `RXE` writer - Data reception enable bit"]
pub type RXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBIE` reader - Transmit bus idle interrupt enable bit"]
pub type TBIE_R = crate::BitReader;
#[doc = "Field `TBIE` writer - Transmit bus idle interrupt enable bit"]
pub type TBIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Transmit interrupt enable bit"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable bit"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Received interrupt enable bit"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Received interrupt enable bit"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBR` reader - LIN Break Field setting bit (valid in master mode only)"]
pub type LBR_R = crate::BitReader;
#[doc = "Field `LBR` writer - LIN Break Field setting bit (valid in master mode only)"]
pub type LBR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - Master/Slave function select bit"]
pub type MS_R = crate::BitReader;
#[doc = "Field `MS` writer - Master/Slave function select bit"]
pub type MS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPCL` reader - Programmable clear bit"]
pub type UPCL_R = crate::BitReader;
#[doc = "Field `UPCL` writer - Programmable clear bit"]
pub type UPCL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data transmission enable bit"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data reception enable bit"]
    #[inline(always)]
    pub fn rxe(&self) -> RXE_R {
        RXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit bus idle interrupt enable bit"]
    #[inline(always)]
    pub fn tbie(&self) -> TBIE_R {
        TBIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit interrupt enable bit"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received interrupt enable bit"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN Break Field setting bit (valid in master mode only)"]
    #[inline(always)]
    pub fn lbr(&self) -> LBR_R {
        LBR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master/Slave function select bit"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Programmable clear bit"]
    #[inline(always)]
    pub fn upcl(&self) -> UPCL_R {
        UPCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data transmission enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<LIN_LIN_SCR_SPEC> {
        TXE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data reception enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<LIN_LIN_SCR_SPEC> {
        RXE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit bus idle interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tbie(&mut self) -> TBIE_W<LIN_LIN_SCR_SPEC> {
        TBIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<LIN_LIN_SCR_SPEC> {
        TIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Received interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<LIN_LIN_SCR_SPEC> {
        RIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - LIN Break Field setting bit (valid in master mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn lbr(&mut self) -> LBR_W<LIN_LIN_SCR_SPEC> {
        LBR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Master/Slave function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<LIN_LIN_SCR_SPEC> {
        MS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Programmable clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn upcl(&mut self) -> UPCL_W<LIN_LIN_SCR_SPEC> {
        UPCL_W::new(self, 7)
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
#[doc = "Serial Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lin_lin_scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIN_LIN_SCR_SPEC;
impl crate::RegisterSpec for LIN_LIN_SCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lin_lin_scr::R`](R) reader structure"]
impl crate::Readable for LIN_LIN_SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lin_lin_scr::W`](W) writer structure"]
impl crate::Writable for LIN_LIN_SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LIN_SCR to value 0"]
impl crate::Resettable for LIN_LIN_SCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
