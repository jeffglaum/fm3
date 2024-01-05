#[doc = "Register `ODDPKS` reader"]
pub type R = crate::R<ODDPKS_SPEC>;
#[doc = "Register `ODDPKS` writer"]
pub type W = crate::W<ODDPKS_SPEC>;
#[doc = "Field `ODDPKS0` reader - \"When the transfer destination address of DMAC is USB.EP1DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS0_R = crate::BitReader;
#[doc = "Field `ODDPKS0` writer - \"When the transfer destination address of DMAC is USB.EP1DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODDPKS1` reader - \"When the transfer destination address of DMAC is USB.EP2DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS1_R = crate::BitReader;
#[doc = "Field `ODDPKS1` writer - \"When the transfer destination address of DMAC is USB.EP2DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODDPKS2` reader - \"When the transfer destination address of DMAC is USB.EP3DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS2_R = crate::BitReader;
#[doc = "Field `ODDPKS2` writer - \"When the transfer destination address of DMAC is USB.EP3DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODDPKS3` reader - \"When the transfer destination address of DMAC is USB.EP4DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS3_R = crate::BitReader;
#[doc = "Field `ODDPKS3` writer - \"When the transfer destination address of DMAC is USB.EP4DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODDPKS4` reader - \"When the transfer destination address of DMAC is USB.EP5DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS4_R = crate::BitReader;
#[doc = "Field `ODDPKS4` writer - \"When the transfer destination address of DMAC is USB.EP5DT, the bit width of the last transfer data is converted to Byte.\""]
pub type ODDPKS4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - \"When the transfer destination address of DMAC is USB.EP1DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    pub fn oddpks0(&self) -> ODDPKS0_R {
        ODDPKS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - \"When the transfer destination address of DMAC is USB.EP2DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    pub fn oddpks1(&self) -> ODDPKS1_R {
        ODDPKS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - \"When the transfer destination address of DMAC is USB.EP3DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    pub fn oddpks2(&self) -> ODDPKS2_R {
        ODDPKS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - \"When the transfer destination address of DMAC is USB.EP4DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    pub fn oddpks3(&self) -> ODDPKS3_R {
        ODDPKS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - \"When the transfer destination address of DMAC is USB.EP5DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    pub fn oddpks4(&self) -> ODDPKS4_R {
        ODDPKS4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - \"When the transfer destination address of DMAC is USB.EP1DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks0(&mut self) -> ODDPKS0_W<ODDPKS_SPEC> {
        ODDPKS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - \"When the transfer destination address of DMAC is USB.EP2DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks1(&mut self) -> ODDPKS1_W<ODDPKS_SPEC> {
        ODDPKS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - \"When the transfer destination address of DMAC is USB.EP3DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks2(&mut self) -> ODDPKS2_W<ODDPKS_SPEC> {
        ODDPKS2_W::new(self, 2)
    }
    #[doc = "Bit 3 - \"When the transfer destination address of DMAC is USB.EP4DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks3(&mut self) -> ODDPKS3_W<ODDPKS_SPEC> {
        ODDPKS3_W::new(self, 3)
    }
    #[doc = "Bit 4 - \"When the transfer destination address of DMAC is USB.EP5DT, the bit width of the last transfer data is converted to Byte.\""]
    #[inline(always)]
    #[must_use]
    pub fn oddpks4(&mut self) -> ODDPKS4_W<ODDPKS_SPEC> {
        ODDPKS4_W::new(self, 4)
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
#[doc = "USB ch.0 Odd Packet Size DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oddpks::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oddpks::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ODDPKS_SPEC;
impl crate::RegisterSpec for ODDPKS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`oddpks::R`](R) reader structure"]
impl crate::Readable for ODDPKS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oddpks::W`](W) writer structure"]
impl crate::Writable for ODDPKS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ODDPKS to value 0"]
impl crate::Resettable for ODDPKS_SPEC {
    const RESET_VALUE: u8 = 0;
}
