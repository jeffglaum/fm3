#[doc = "Register `MODE0` reader"]
pub type R = crate::R<MODE0_SPEC>;
#[doc = "Register `MODE0` writer"]
pub type W = crate::W<MODE0_SPEC>;
#[doc = "Field `WDTH` reader - specify Data Width"]
pub type WDTH_R = crate::FieldReader;
#[doc = "Field `WDTH` writer - specify Data Width"]
pub type WDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RBMON` reader - Read Byte Mask ON"]
pub type RBMON_R = crate::BitReader;
#[doc = "Field `RBMON` writer - Read Byte Mask ON"]
pub type RBMON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEOFF` reader - disable the write enable signal (MWEX) operation"]
pub type WEOFF_R = crate::BitReader;
#[doc = "Field `WEOFF` writer - disable the write enable signal (MWEX) operation"]
pub type WEOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAND` reader - NAND Flash memory mode"]
pub type NAND_R = crate::BitReader;
#[doc = "Field `NAND` writer - NAND Flash memory mode"]
pub type NAND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAGE` reader - NOR Flash memory page access mode"]
pub type PAGE_R = crate::BitReader;
#[doc = "Field `PAGE` writer - NOR Flash memory page access mode"]
pub type PAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDY` reader - control the external RDY function"]
pub type RDY_R = crate::BitReader;
#[doc = "Field `RDY` writer - control the external RDY function"]
pub type RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHRTDOUT` reader - select to which idle cycle the write data output is extended"]
pub type SHRTDOUT_R = crate::BitReader;
#[doc = "Field `SHRTDOUT` writer - select to which idle cycle the write data output is extended"]
pub type SHRTDOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPXMODE` reader - select operation bus mode"]
pub type MPXMODE_R = crate::BitReader;
#[doc = "Field `MPXMODE` writer - select operation bus mode"]
pub type MPXMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALEINV` reader - set up the polarity of the ALE signal"]
pub type ALEINV_R = crate::BitReader;
#[doc = "Field `ALEINV` writer - set up the polarity of the ALE signal"]
pub type ALEINV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPXDOFF` reader - select whether or not the address is output to the data lines in multiplex mode"]
pub type MPXDOFF_R = crate::BitReader;
#[doc = "Field `MPXDOFF` writer - select whether or not the address is output to the data lines in multiplex mode"]
pub type MPXDOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPXCSOF` reader - select a CS assertion from the start of accessing to the end of address output"]
pub type MPXCSOF_R = crate::BitReader;
#[doc = "Field `MPXCSOF` writer - select a CS assertion from the start of accessing to the end of address output"]
pub type MPXCSOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOEXEUP` reader - select how to set the MOEX width"]
pub type MOEXEUP_R = crate::BitReader;
#[doc = "Field `MOEXEUP` writer - select how to set the MOEX width"]
pub type MOEXEUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - specify Data Width"]
    #[inline(always)]
    pub fn wdth(&self) -> WDTH_R {
        WDTH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Read Byte Mask ON"]
    #[inline(always)]
    pub fn rbmon(&self) -> RBMON_R {
        RBMON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - disable the write enable signal (MWEX) operation"]
    #[inline(always)]
    pub fn weoff(&self) -> WEOFF_R {
        WEOFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAND Flash memory mode"]
    #[inline(always)]
    pub fn nand(&self) -> NAND_R {
        NAND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NOR Flash memory page access mode"]
    #[inline(always)]
    pub fn page(&self) -> PAGE_R {
        PAGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - control the external RDY function"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - select to which idle cycle the write data output is extended"]
    #[inline(always)]
    pub fn shrtdout(&self) -> SHRTDOUT_R {
        SHRTDOUT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - select operation bus mode"]
    #[inline(always)]
    pub fn mpxmode(&self) -> MPXMODE_R {
        MPXMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - set up the polarity of the ALE signal"]
    #[inline(always)]
    pub fn aleinv(&self) -> ALEINV_R {
        ALEINV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - select whether or not the address is output to the data lines in multiplex mode"]
    #[inline(always)]
    pub fn mpxdoff(&self) -> MPXDOFF_R {
        MPXDOFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - select a CS assertion from the start of accessing to the end of address output"]
    #[inline(always)]
    pub fn mpxcsof(&self) -> MPXCSOF_R {
        MPXCSOF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - select how to set the MOEX width"]
    #[inline(always)]
    pub fn moexeup(&self) -> MOEXEUP_R {
        MOEXEUP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - specify Data Width"]
    #[inline(always)]
    #[must_use]
    pub fn wdth(&mut self) -> WDTH_W<MODE0_SPEC> {
        WDTH_W::new(self, 0)
    }
    #[doc = "Bit 2 - Read Byte Mask ON"]
    #[inline(always)]
    #[must_use]
    pub fn rbmon(&mut self) -> RBMON_W<MODE0_SPEC> {
        RBMON_W::new(self, 2)
    }
    #[doc = "Bit 3 - disable the write enable signal (MWEX) operation"]
    #[inline(always)]
    #[must_use]
    pub fn weoff(&mut self) -> WEOFF_W<MODE0_SPEC> {
        WEOFF_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAND Flash memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn nand(&mut self) -> NAND_W<MODE0_SPEC> {
        NAND_W::new(self, 4)
    }
    #[doc = "Bit 5 - NOR Flash memory page access mode"]
    #[inline(always)]
    #[must_use]
    pub fn page(&mut self) -> PAGE_W<MODE0_SPEC> {
        PAGE_W::new(self, 5)
    }
    #[doc = "Bit 6 - control the external RDY function"]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<MODE0_SPEC> {
        RDY_W::new(self, 6)
    }
    #[doc = "Bit 7 - select to which idle cycle the write data output is extended"]
    #[inline(always)]
    #[must_use]
    pub fn shrtdout(&mut self) -> SHRTDOUT_W<MODE0_SPEC> {
        SHRTDOUT_W::new(self, 7)
    }
    #[doc = "Bit 8 - select operation bus mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpxmode(&mut self) -> MPXMODE_W<MODE0_SPEC> {
        MPXMODE_W::new(self, 8)
    }
    #[doc = "Bit 9 - set up the polarity of the ALE signal"]
    #[inline(always)]
    #[must_use]
    pub fn aleinv(&mut self) -> ALEINV_W<MODE0_SPEC> {
        ALEINV_W::new(self, 9)
    }
    #[doc = "Bit 11 - select whether or not the address is output to the data lines in multiplex mode"]
    #[inline(always)]
    #[must_use]
    pub fn mpxdoff(&mut self) -> MPXDOFF_W<MODE0_SPEC> {
        MPXDOFF_W::new(self, 11)
    }
    #[doc = "Bit 12 - select a CS assertion from the start of accessing to the end of address output"]
    #[inline(always)]
    #[must_use]
    pub fn mpxcsof(&mut self) -> MPXCSOF_W<MODE0_SPEC> {
        MPXCSOF_W::new(self, 12)
    }
    #[doc = "Bit 13 - select how to set the MOEX width"]
    #[inline(always)]
    #[must_use]
    pub fn moexeup(&mut self) -> MOEXEUP_W<MODE0_SPEC> {
        MOEXEUP_W::new(self, 13)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Mode Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODE0_SPEC;
impl crate::RegisterSpec for MODE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode0::R`](R) reader structure"]
impl crate::Readable for MODE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mode0::W`](W) writer structure"]
impl crate::Writable for MODE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE0 to value 0"]
impl crate::Resettable for MODE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
