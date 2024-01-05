#[doc = "Register `UART_ESCR` reader"]
pub type R = crate::R<UART_UART_ESCR_SPEC>;
#[doc = "Register `UART_ESCR` writer"]
pub type W = crate::W<UART_UART_ESCR_SPEC>;
#[doc = "Field `L` reader - Data length select bit"]
pub type L_R = crate::FieldReader;
#[doc = "Field `L` writer - Data length select bit"]
pub type L_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `P` reader - Parity select bit (only functions in operation mode 0)"]
pub type P_R = crate::BitReader;
#[doc = "Field `P` writer - Parity select bit (only functions in operation mode 0)"]
pub type P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Parity enable bit (only functions in operation mode 0)"]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable bit (only functions in operation mode 0)"]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INV` reader - Inverted serial data format bit"]
pub type INV_R = crate::BitReader;
#[doc = "Field `INV` writer - Inverted serial data format bit"]
pub type INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESBL` reader - Extension stop bit length select bit"]
pub type ESBL_R = crate::BitReader;
#[doc = "Field `ESBL` writer - Extension stop bit length select bit"]
pub type ESBL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLWEN` reader - Flow control enable bit"]
pub type FLWEN_R = crate::BitReader;
#[doc = "Field `FLWEN` writer - Flow control enable bit"]
pub type FLWEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Data length select bit"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Parity select bit (only functions in operation mode 0)"]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Parity enable bit (only functions in operation mode 0)"]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Inverted serial data format bit"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Extension stop bit length select bit"]
    #[inline(always)]
    pub fn esbl(&self) -> ESBL_R {
        ESBL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flow control enable bit"]
    #[inline(always)]
    pub fn flwen(&self) -> FLWEN_R {
        FLWEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data length select bit"]
    #[inline(always)]
    #[must_use]
    pub fn l(&mut self) -> L_W<UART_UART_ESCR_SPEC> {
        L_W::new(self, 0)
    }
    #[doc = "Bit 3 - Parity select bit (only functions in operation mode 0)"]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<UART_UART_ESCR_SPEC> {
        P_W::new(self, 3)
    }
    #[doc = "Bit 4 - Parity enable bit (only functions in operation mode 0)"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<UART_UART_ESCR_SPEC> {
        PEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Inverted serial data format bit"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<UART_UART_ESCR_SPEC> {
        INV_W::new(self, 5)
    }
    #[doc = "Bit 6 - Extension stop bit length select bit"]
    #[inline(always)]
    #[must_use]
    pub fn esbl(&mut self) -> ESBL_W<UART_UART_ESCR_SPEC> {
        ESBL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Flow control enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn flwen(&mut self) -> FLWEN_W<UART_UART_ESCR_SPEC> {
        FLWEN_W::new(self, 7)
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
#[doc = "Extended Communication Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_escr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_escr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_UART_ESCR_SPEC;
impl crate::RegisterSpec for UART_UART_ESCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart_uart_escr::R`](R) reader structure"]
impl crate::Readable for UART_UART_ESCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_uart_escr::W`](W) writer structure"]
impl crate::Writable for UART_UART_ESCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UART_ESCR to value 0"]
impl crate::Resettable for UART_UART_ESCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
