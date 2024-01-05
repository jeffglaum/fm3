#[doc = "Register `UART_SMR` reader"]
pub type R = crate::R<UART_UART_SMR_SPEC>;
#[doc = "Register `UART_SMR` writer"]
pub type W = crate::W<UART_UART_SMR_SPEC>;
#[doc = "Field `SOE` reader - Serial data output enable bit"]
pub type SOE_R = crate::BitReader;
#[doc = "Field `SOE` writer - Serial data output enable bit"]
pub type SOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDS` reader - Transfer direction select bit"]
pub type BDS_R = crate::BitReader;
#[doc = "Field `BDS` writer - Transfer direction select bit"]
pub type BDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBL` reader - Stop bit length select bit"]
pub type SBL_R = crate::BitReader;
#[doc = "Field `SBL` writer - Stop bit length select bit"]
pub type SBL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCR` reader - Wake-up control bit"]
pub type WUCR_R = crate::BitReader;
#[doc = "Field `WUCR` writer - Wake-up control bit"]
pub type WUCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD` reader - Operation mode set bit"]
pub type MD_R = crate::FieldReader;
#[doc = "Field `MD` writer - Operation mode set bit"]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Serial data output enable bit"]
    #[inline(always)]
    pub fn soe(&self) -> SOE_R {
        SOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer direction select bit"]
    #[inline(always)]
    pub fn bds(&self) -> BDS_R {
        BDS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stop bit length select bit"]
    #[inline(always)]
    pub fn sbl(&self) -> SBL_R {
        SBL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-up control bit"]
    #[inline(always)]
    pub fn wucr(&self) -> WUCR_R {
        WUCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Operation mode set bit"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 0 - Serial data output enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn soe(&mut self) -> SOE_W<UART_UART_SMR_SPEC> {
        SOE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Transfer direction select bit"]
    #[inline(always)]
    #[must_use]
    pub fn bds(&mut self) -> BDS_W<UART_UART_SMR_SPEC> {
        BDS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stop bit length select bit"]
    #[inline(always)]
    #[must_use]
    pub fn sbl(&mut self) -> SBL_W<UART_UART_SMR_SPEC> {
        SBL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-up control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wucr(&mut self) -> WUCR_W<UART_UART_SMR_SPEC> {
        WUCR_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Operation mode set bit"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<UART_UART_SMR_SPEC> {
        MD_W::new(self, 5)
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
#[doc = "Serial Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_smr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_smr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_UART_SMR_SPEC;
impl crate::RegisterSpec for UART_UART_SMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart_uart_smr::R`](R) reader structure"]
impl crate::Readable for UART_UART_SMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_uart_smr::W`](W) writer structure"]
impl crate::Writable for UART_UART_SMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UART_SMR to value 0"]
impl crate::Resettable for UART_UART_SMR_SPEC {
    const RESET_VALUE: u8 = 0;
}
