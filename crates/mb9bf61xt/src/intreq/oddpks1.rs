#[doc = "Register `ODDPKS1` reader"]
pub type R = crate::R<ODDPKS1_SPEC>;
#[doc = "Register `ODDPKS1` writer"]
pub type W = crate::W<ODDPKS1_SPEC>;
#[doc = "Field `ODDPKS10` reader - \"When the transfer destination address of the DMAC is USB.EP1DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS10_R = crate::BitReader;
#[doc = "Field `ODDPKS10` writer - \"When the transfer destination address of the DMAC is USB.EP1DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODDPKS11` reader - \"When the transfer destination address of the DMAC is USB.EP2DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS11_R = crate::BitReader;
#[doc = "Field `ODDPKS11` writer - \"When the transfer destination address of the DMAC is USB.EP2DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODDPKS12` reader - \"When the transfer destination address of the DMAC is USB.EP3DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS12_R = crate::BitReader;
#[doc = "Field `ODDPKS12` writer - \"When the transfer destination address of the DMAC is USB.EP3DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODDPKS13` reader - \"When the transfer destination address of the DMAC is USB.EP4DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS13_R = crate::BitReader;
#[doc = "Field `ODDPKS13` writer - \"When the transfer destination address of the DMAC is USB.EP4DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODDPKS14` reader - \"When the transfer destination address of the DMAC is USB.EP5DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS14_R = crate::BitReader;
#[doc = "Field `ODDPKS14` writer - \"When the transfer destination address of the DMAC is USB.EP5DT, convert the bit width of the last transfer data to byte.\""]
pub type ODDPKS14_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - \"When the transfer destination address of the DMAC is USB.EP1DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    pub fn oddpks10(&self) -> ODDPKS10_R {
        ODDPKS10_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - \"When the transfer destination address of the DMAC is USB.EP2DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    pub fn oddpks11(&self) -> ODDPKS11_R {
        ODDPKS11_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - \"When the transfer destination address of the DMAC is USB.EP3DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    pub fn oddpks12(&self) -> ODDPKS12_R {
        ODDPKS12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - \"When the transfer destination address of the DMAC is USB.EP4DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    pub fn oddpks13(&self) -> ODDPKS13_R {
        ODDPKS13_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - \"When the transfer destination address of the DMAC is USB.EP5DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    pub fn oddpks14(&self) -> ODDPKS14_R {
        ODDPKS14_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - \"When the transfer destination address of the DMAC is USB.EP1DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks10(&mut self) -> ODDPKS10_W<ODDPKS1_SPEC> {
        ODDPKS10_W::new(self, 0)
    }
    #[doc = "Bit 1 - \"When the transfer destination address of the DMAC is USB.EP2DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks11(&mut self) -> ODDPKS11_W<ODDPKS1_SPEC> {
        ODDPKS11_W::new(self, 1)
    }
    #[doc = "Bit 2 - \"When the transfer destination address of the DMAC is USB.EP3DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks12(&mut self) -> ODDPKS12_W<ODDPKS1_SPEC> {
        ODDPKS12_W::new(self, 2)
    }
    #[doc = "Bit 3 - \"When the transfer destination address of the DMAC is USB.EP4DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks13(&mut self) -> ODDPKS13_W<ODDPKS1_SPEC> {
        ODDPKS13_W::new(self, 3)
    }
    #[doc = "Bit 4 - \"When the transfer destination address of the DMAC is USB.EP5DT, convert the bit width of the last transfer data to byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks14(&mut self) -> ODDPKS14_W<ODDPKS1_SPEC> {
        ODDPKS14_W::new(self, 4)
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
#[doc = "USB ch.1 Odd Packet Size DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oddpks1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oddpks1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODDPKS1_SPEC;
impl crate::RegisterSpec for ODDPKS1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oddpks1::R`](R) reader structure"]
impl crate::Readable for ODDPKS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oddpks1::W`](W) writer structure"]
impl crate::Writable for ODDPKS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ODDPKS1 to value 0"]
impl crate::Resettable for ODDPKS1_SPEC {
    const RESET_VALUE: u8 = 0;
}
