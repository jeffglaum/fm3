#[doc = "Register `RST_STR` reader"]
pub type R = crate::R<RST_STR_SPEC>;
#[doc = "Field `PONR` reader - Power-on reset/low-voltage detection reset flag"]
pub type PONR_R = crate::BitReader;
#[doc = "Field `INITX` reader - INITX pin input reset flag"]
pub type INITX_R = crate::BitReader;
#[doc = "Field `SWDT` reader - Software watchdog reset flag"]
pub type SWDT_R = crate::BitReader;
#[doc = "Field `HWDT` reader - Hardware watchdog reset flag"]
pub type HWDT_R = crate::BitReader;
#[doc = "Field `CSVR` reader - Clock failure detection reset flag"]
pub type CSVR_R = crate::BitReader;
#[doc = "Field `FCSR` reader - Flag for anomalous frequency detection reset"]
pub type FCSR_R = crate::BitReader;
#[doc = "Field `SRST` reader - Software reset flag"]
pub type SRST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power-on reset/low-voltage detection reset flag"]
    #[inline(always)]
    pub fn ponr(&self) -> PONR_R {
        PONR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INITX pin input reset flag"]
    #[inline(always)]
    pub fn initx(&self) -> INITX_R {
        INITX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Software watchdog reset flag"]
    #[inline(always)]
    pub fn swdt(&self) -> SWDT_R {
        SWDT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware watchdog reset flag"]
    #[inline(always)]
    pub fn hwdt(&self) -> HWDT_R {
        HWDT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock failure detection reset flag"]
    #[inline(always)]
    pub fn csvr(&self) -> CSVR_R {
        CSVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flag for anomalous frequency detection reset"]
    #[inline(always)]
    pub fn fcsr(&self) -> FCSR_R {
        FCSR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software reset flag"]
    #[inline(always)]
    pub fn srst(&self) -> SRST_R {
        SRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Reset Cause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst_str::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RST_STR_SPEC;
impl crate::RegisterSpec for RST_STR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rst_str::R`](R) reader structure"]
impl crate::Readable for RST_STR_SPEC {}
#[doc = "`reset()` method sets RST_STR to value 0x01"]
impl crate::Resettable for RST_STR_SPEC {
    const RESET_VALUE: u16 = 0x01;
}
