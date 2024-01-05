#[doc = "Register `EPFR00` reader"]
pub type R = crate::R<EPFR00_SPEC>;
#[doc = "Register `EPFR00` writer"]
pub type W = crate::W<EPFR00_SPEC>;
#[doc = "Field `NMIS` reader - NMIX function select bit"]
pub type NMIS_R = crate::BitReader;
#[doc = "Field `NMIS` writer - NMIX function select bit"]
pub type NMIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CROUTE` reader - Internal high-speed CR oscillation output function select bit"]
pub type CROUTE_R = crate::FieldReader;
#[doc = "Field `CROUTE` writer - Internal high-speed CR oscillation output function select bit"]
pub type CROUTE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SUBOUTE` reader - Sub clock divide output function select bit"]
pub type SUBOUTE_R = crate::FieldReader;
#[doc = "Field `SUBOUTE` writer - Sub clock divide output function select bit"]
pub type SUBOUTE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBP0E` reader - USBch0 function select bit"]
pub type USBP0E_R = crate::BitReader;
#[doc = "Field `USBP0E` writer - USBch0 function select bit"]
pub type USBP0E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBP1E` reader - USBch1 function select bit"]
pub type USBP1E_R = crate::BitReader;
#[doc = "Field `USBP1E` writer - USBch1 function select bit"]
pub type USBP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGEN0B` reader - JTAG function select bit0"]
pub type JTAGEN0B_R = crate::BitReader;
#[doc = "Field `JTAGEN0B` writer - JTAG function select bit0"]
pub type JTAGEN0B_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGEN1S` reader - JTAG function select bit1"]
pub type JTAGEN1S_R = crate::BitReader;
#[doc = "Field `JTAGEN1S` writer - JTAG function select bit1"]
pub type JTAGEN1S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRC0E` reader - TRACED function select bit0"]
pub type TRC0E_R = crate::BitReader;
#[doc = "Field `TRC0E` writer - TRACED function select bit0"]
pub type TRC0E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRC1E` reader - TRACED function select bit1"]
pub type TRC1E_R = crate::BitReader;
#[doc = "Field `TRC1E` writer - TRACED function select bit1"]
pub type TRC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NMIX function select bit"]
    #[inline(always)]
    pub fn nmis(&self) -> NMIS_R {
        NMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Internal high-speed CR oscillation output function select bit"]
    #[inline(always)]
    pub fn croute(&self) -> CROUTE_R {
        CROUTE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Sub clock divide output function select bit"]
    #[inline(always)]
    pub fn suboute(&self) -> SUBOUTE_R {
        SUBOUTE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 9 - USBch0 function select bit"]
    #[inline(always)]
    pub fn usbp0e(&self) -> USBP0E_R {
        USBP0E_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - USBch1 function select bit"]
    #[inline(always)]
    pub fn usbp1e(&self) -> USBP1E_R {
        USBP1E_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - JTAG function select bit0"]
    #[inline(always)]
    pub fn jtagen0b(&self) -> JTAGEN0B_R {
        JTAGEN0B_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - JTAG function select bit1"]
    #[inline(always)]
    pub fn jtagen1s(&self) -> JTAGEN1S_R {
        JTAGEN1S_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - TRACED function select bit0"]
    #[inline(always)]
    pub fn trc0e(&self) -> TRC0E_R {
        TRC0E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TRACED function select bit1"]
    #[inline(always)]
    pub fn trc1e(&self) -> TRC1E_R {
        TRC1E_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMIX function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn nmis(&mut self) -> NMIS_W<EPFR00_SPEC> {
        NMIS_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Internal high-speed CR oscillation output function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn croute(&mut self) -> CROUTE_W<EPFR00_SPEC> {
        CROUTE_W::new(self, 1)
    }
    #[doc = "Bits 6:7 - Sub clock divide output function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn suboute(&mut self) -> SUBOUTE_W<EPFR00_SPEC> {
        SUBOUTE_W::new(self, 6)
    }
    #[doc = "Bit 9 - USBch0 function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbp0e(&mut self) -> USBP0E_W<EPFR00_SPEC> {
        USBP0E_W::new(self, 9)
    }
    #[doc = "Bit 13 - USBch1 function select bit"]
    #[inline(always)]
    #[must_use]
    pub fn usbp1e(&mut self) -> USBP1E_W<EPFR00_SPEC> {
        USBP1E_W::new(self, 13)
    }
    #[doc = "Bit 16 - JTAG function select bit0"]
    #[inline(always)]
    #[must_use]
    pub fn jtagen0b(&mut self) -> JTAGEN0B_W<EPFR00_SPEC> {
        JTAGEN0B_W::new(self, 16)
    }
    #[doc = "Bit 17 - JTAG function select bit1"]
    #[inline(always)]
    #[must_use]
    pub fn jtagen1s(&mut self) -> JTAGEN1S_W<EPFR00_SPEC> {
        JTAGEN1S_W::new(self, 17)
    }
    #[doc = "Bit 24 - TRACED function select bit0"]
    #[inline(always)]
    #[must_use]
    pub fn trc0e(&mut self) -> TRC0E_W<EPFR00_SPEC> {
        TRC0E_W::new(self, 24)
    }
    #[doc = "Bit 25 - TRACED function select bit1"]
    #[inline(always)]
    #[must_use]
    pub fn trc1e(&mut self) -> TRC1E_W<EPFR00_SPEC> {
        TRC1E_W::new(self, 25)
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
#[doc = "Extended pin function setting register 00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr00::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr00::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPFR00_SPEC;
impl crate::RegisterSpec for EPFR00_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epfr00::R`](R) reader structure"]
impl crate::Readable for EPFR00_SPEC {}
#[doc = "`write(|w| ..)` method takes [`epfr00::W`](W) writer structure"]
impl crate::Writable for EPFR00_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPFR00 to value 0x0003_0000"]
impl crate::Resettable for EPFR00_SPEC {
    const RESET_VALUE: u32 = 0x0003_0000;
}
