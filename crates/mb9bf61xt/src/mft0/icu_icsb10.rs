#[doc = "Register `ICU_ICSB10` reader"]
pub type R = crate::R<ICU_ICSB10_SPEC>;
#[doc = "Field `IEI0` reader - indicates the latest valid edge of ICU-ch.(0)"]
pub type IEI0_R = crate::BitReader;
#[doc = "Field `IEI1` reader - indicates the latest valid edge of ICU-ch.(1)"]
pub type IEI1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - indicates the latest valid edge of ICU-ch.(0)"]
    #[inline(always)]
    pub fn iei0(&self) -> IEI0_R {
        IEI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - indicates the latest valid edge of ICU-ch.(1)"]
    #[inline(always)]
    pub fn iei1(&self) -> IEI1_R {
        IEI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "\"ICU ch.1,0 Control Register B\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icu_icsb10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICU_ICSB10_SPEC;
impl crate::RegisterSpec for ICU_ICSB10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icu_icsb10::R`](R) reader structure"]
impl crate::Readable for ICU_ICSB10_SPEC {}
#[doc = "`reset()` method sets ICU_ICSB10 to value 0"]
impl crate::Resettable for ICU_ICSB10_SPEC {
    const RESET_VALUE: u8 = 0;
}
