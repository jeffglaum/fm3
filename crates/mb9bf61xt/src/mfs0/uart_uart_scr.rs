#[doc = "Register `UART_SCR` reader"]
pub type R = crate::R<UART_UART_SCR_SPEC>;
#[doc = "Register `UART_SCR` writer"]
pub type W = crate::W<UART_UART_SCR_SPEC>;
#[doc = "Field `TXE` reader - Transmission operation enable bit"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - Transmission operation enable bit"]
pub type TXE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXE` reader - Received operation enable bit"]
pub type RXE_R = crate::BitReader;
#[doc = "Field `RXE` writer - Received operation enable bit"]
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
#[doc = "Field `UPCL` reader - Programmable Clear bit"]
pub type UPCL_R = crate::BitReader;
#[doc = "Field `UPCL` writer - Programmable Clear bit"]
pub type UPCL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmission operation enable bit"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received operation enable bit"]
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
    #[doc = "Bit 7 - Programmable Clear bit"]
    #[inline(always)]
    pub fn upcl(&self) -> UPCL_R {
        UPCL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission operation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<UART_UART_SCR_SPEC> {
        TXE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Received operation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rxe(&mut self) -> RXE_W<UART_UART_SCR_SPEC> {
        RXE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit bus idle interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tbie(&mut self) -> TBIE_W<UART_UART_SCR_SPEC> {
        TBIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<UART_UART_SCR_SPEC> {
        TIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Received interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<UART_UART_SCR_SPEC> {
        RIE_W::new(self, 4)
    }
    #[doc = "Bit 7 - Programmable Clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn upcl(&mut self) -> UPCL_W<UART_UART_SCR_SPEC> {
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
#[doc = "Serial Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_scr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_scr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_UART_SCR_SPEC;
impl crate::RegisterSpec for UART_UART_SCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart_uart_scr::R`](R) reader structure"]
impl crate::Readable for UART_UART_SCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_uart_scr::W`](W) writer structure"]
impl crate::Writable for UART_UART_SCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UART_SCR to value 0"]
impl crate::Resettable for UART_UART_SCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
