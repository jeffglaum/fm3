#[doc = "Register `IRQ29MON` reader"]
pub type R = crate::R<IRQ29MON_SPEC>;
#[doc = "Field `ICU0INT0` reader - Interrupt request on the input capture ch.0 in the MFT unit 0"]
pub type ICU0INT0_R = crate::BitReader;
#[doc = "Field `ICU0INT1` reader - Interrupt request on the input capture ch.1 in the MFT unit 0"]
pub type ICU0INT1_R = crate::BitReader;
#[doc = "Field `ICU0INT2` reader - Interrupt request on the input capture ch.2 in the MFT unit 0"]
pub type ICU0INT2_R = crate::BitReader;
#[doc = "Field `ICU0INT3` reader - Interrupt request on the input capture ch.3 in the MFT unit 0"]
pub type ICU0INT3_R = crate::BitReader;
#[doc = "Field `ICU1INT0` reader - Interrupt request on the input capture ch.0 in the MFT unit 1"]
pub type ICU1INT0_R = crate::BitReader;
#[doc = "Field `ICU1INT1` reader - Interrupt request on the input capture ch.1 in the MFT unit 1"]
pub type ICU1INT1_R = crate::BitReader;
#[doc = "Field `ICU1INT2` reader - Interrupt request on the input capture ch.2 in the MFT unit 1"]
pub type ICU1INT2_R = crate::BitReader;
#[doc = "Field `ICU1INT3` reader - Interrupt request on the input capture ch.3 in the MFT unit 1"]
pub type ICU1INT3_R = crate::BitReader;
#[doc = "Field `ICU2INT0` reader - Interrupt request on the input capture ch.0 in the MFT unit 2"]
pub type ICU2INT0_R = crate::BitReader;
#[doc = "Field `ICU2INT1` reader - Interrupt request on the input capture ch.1 in the MFT unit 2"]
pub type ICU2INT1_R = crate::BitReader;
#[doc = "Field `ICU2INT2` reader - Interrupt request on the input capture ch.2 in the MFT unit 2"]
pub type ICU2INT2_R = crate::BitReader;
#[doc = "Field `ICU2INT3` reader - Interrupt request on the input capture ch.3 in the MFT unit 2"]
pub type ICU2INT3_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on the input capture ch.0 in the MFT unit 0"]
    #[inline(always)]
    pub fn icu0int0(&self) -> ICU0INT0_R {
        ICU0INT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt request on the input capture ch.1 in the MFT unit 0"]
    #[inline(always)]
    pub fn icu0int1(&self) -> ICU0INT1_R {
        ICU0INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt request on the input capture ch.2 in the MFT unit 0"]
    #[inline(always)]
    pub fn icu0int2(&self) -> ICU0INT2_R {
        ICU0INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt request on the input capture ch.3 in the MFT unit 0"]
    #[inline(always)]
    pub fn icu0int3(&self) -> ICU0INT3_R {
        ICU0INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt request on the input capture ch.0 in the MFT unit 1"]
    #[inline(always)]
    pub fn icu1int0(&self) -> ICU1INT0_R {
        ICU1INT0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt request on the input capture ch.1 in the MFT unit 1"]
    #[inline(always)]
    pub fn icu1int1(&self) -> ICU1INT1_R {
        ICU1INT1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt request on the input capture ch.2 in the MFT unit 1"]
    #[inline(always)]
    pub fn icu1int2(&self) -> ICU1INT2_R {
        ICU1INT2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt request on the input capture ch.3 in the MFT unit 1"]
    #[inline(always)]
    pub fn icu1int3(&self) -> ICU1INT3_R {
        ICU1INT3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt request on the input capture ch.0 in the MFT unit 2"]
    #[inline(always)]
    pub fn icu2int0(&self) -> ICU2INT0_R {
        ICU2INT0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt request on the input capture ch.1 in the MFT unit 2"]
    #[inline(always)]
    pub fn icu2int1(&self) -> ICU2INT1_R {
        ICU2INT1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt request on the input capture ch.2 in the MFT unit 2"]
    #[inline(always)]
    pub fn icu2int2(&self) -> ICU2INT2_R {
        ICU2INT2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt request on the input capture ch.3 in the MFT unit 2"]
    #[inline(always)]
    pub fn icu2int3(&self) -> ICU2INT3_R {
        ICU2INT3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "IRQ29 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq29mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ29MON_SPEC;
impl crate::RegisterSpec for IRQ29MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq29mon::R`](R) reader structure"]
impl crate::Readable for IRQ29MON_SPEC {}
#[doc = "`reset()` method sets IRQ29MON to value 0"]
impl crate::Resettable for IRQ29MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
