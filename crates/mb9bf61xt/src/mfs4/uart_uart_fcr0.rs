#[doc = "Register `UART_FCR0` reader"]
pub type R = crate::R<UART_UART_FCR0_SPEC>;
#[doc = "Register `UART_FCR0` writer"]
pub type W = crate::W<UART_UART_FCR0_SPEC>;
#[doc = "Field `FE1` reader - FIFO1 operation enable bit"]
pub type FE1_R = crate::BitReader;
#[doc = "Field `FE1` writer - FIFO1 operation enable bit"]
pub type FE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE2` reader - FIFO2 operation enable bit"]
pub type FE2_R = crate::BitReader;
#[doc = "Field `FE2` writer - FIFO2 operation enable bit"]
pub type FE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCL1` reader - FIFO1 reset bit"]
pub type FCL1_R = crate::BitReader;
#[doc = "Field `FCL1` writer - FIFO1 reset bit"]
pub type FCL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCL2` reader - FIFO2 reset bit"]
pub type FCL2_R = crate::BitReader;
#[doc = "Field `FCL2` writer - FIFO2 reset bit"]
pub type FCL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSET` reader - FIFO pointer save bit"]
pub type FSET_R = crate::BitReader;
#[doc = "Field `FSET` writer - FIFO pointer save bit"]
pub type FSET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLD` reader - FIFO pointer reload bit"]
pub type FLD_R = crate::BitReader;
#[doc = "Field `FLD` writer - FIFO pointer reload bit"]
pub type FLD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLST` reader - FIFO re-transmit data lost flag bit"]
pub type FLST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FIFO1 operation enable bit"]
    #[inline(always)]
    pub fn fe1(&self) -> FE1_R {
        FE1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO2 operation enable bit"]
    #[inline(always)]
    pub fn fe2(&self) -> FE2_R {
        FE2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO1 reset bit"]
    #[inline(always)]
    pub fn fcl1(&self) -> FCL1_R {
        FCL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO2 reset bit"]
    #[inline(always)]
    pub fn fcl2(&self) -> FCL2_R {
        FCL2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FIFO pointer save bit"]
    #[inline(always)]
    pub fn fset(&self) -> FSET_R {
        FSET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIFO pointer reload bit"]
    #[inline(always)]
    pub fn fld(&self) -> FLD_R {
        FLD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO re-transmit data lost flag bit"]
    #[inline(always)]
    pub fn flst(&self) -> FLST_R {
        FLST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO1 operation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fe1(&mut self) -> FE1_W<UART_UART_FCR0_SPEC> {
        FE1_W::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO2 operation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn fe2(&mut self) -> FE2_W<UART_UART_FCR0_SPEC> {
        FE2_W::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO1 reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn fcl1(&mut self) -> FCL1_W<UART_UART_FCR0_SPEC> {
        FCL1_W::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO2 reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn fcl2(&mut self) -> FCL2_W<UART_UART_FCR0_SPEC> {
        FCL2_W::new(self, 3)
    }
    #[doc = "Bit 4 - FIFO pointer save bit"]
    #[inline(always)]
    #[must_use]
    pub fn fset(&mut self) -> FSET_W<UART_UART_FCR0_SPEC> {
        FSET_W::new(self, 4)
    }
    #[doc = "Bit 5 - FIFO pointer reload bit"]
    #[inline(always)]
    #[must_use]
    pub fn fld(&mut self) -> FLD_W<UART_UART_FCR0_SPEC> {
        FLD_W::new(self, 5)
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
#[doc = "FIFO Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_uart_fcr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_uart_fcr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_UART_FCR0_SPEC;
impl crate::RegisterSpec for UART_UART_FCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart_uart_fcr0::R`](R) reader structure"]
impl crate::Readable for UART_UART_FCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_uart_fcr0::W`](W) writer structure"]
impl crate::Writable for UART_UART_FCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UART_FCR0 to value 0"]
impl crate::Resettable for UART_UART_FCR0_SPEC {
    const RESET_VALUE: u8 = 0;
}
