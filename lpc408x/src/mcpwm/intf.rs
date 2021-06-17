#[doc = "Register `INTF` reader"]
pub struct R(crate::R<INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Limit interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ILIM0_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM0_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM0_F` reader - Limit interrupt flag for channel 0."]
pub struct ILIM0_F_R(crate::FieldReader<bool, ILIM0_F_A>);
impl ILIM0_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILIM0_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM0_F_A {
        match self.bits {
            false => ILIM0_F_A::THIS_INTERRUPT_SOURC,
            true => ILIM0_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == ILIM0_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == ILIM0_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for ILIM0_F_R {
    type Target = crate::FieldReader<bool, ILIM0_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Match interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<IMAT0_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT0_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT0_F` reader - Match interrupt flag for channel 0."]
pub struct IMAT0_F_R(crate::FieldReader<bool, IMAT0_F_A>);
impl IMAT0_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMAT0_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT0_F_A {
        match self.bits {
            false => IMAT0_F_A::THIS_INTERRUPT_SOURC,
            true => IMAT0_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == IMAT0_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == IMAT0_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for IMAT0_F_R {
    type Target = crate::FieldReader<bool, IMAT0_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Capture interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP0_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ICAP0_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP0_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP0_F` reader - Capture interrupt flag for channel 0."]
pub struct ICAP0_F_R(crate::FieldReader<bool, ICAP0_F_A>);
impl ICAP0_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICAP0_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP0_F_A {
        match self.bits {
            false => ICAP0_F_A::THIS_INTERRUPT_SOURC,
            true => ICAP0_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == ICAP0_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == ICAP0_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for ICAP0_F_R {
    type Target = crate::FieldReader<bool, ICAP0_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Limit interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ILIM1_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM1_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM1_F` reader - Limit interrupt flag for channel 1."]
pub struct ILIM1_F_R(crate::FieldReader<bool, ILIM1_F_A>);
impl ILIM1_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILIM1_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM1_F_A {
        match self.bits {
            false => ILIM1_F_A::THIS_INTERRUPT_SOURC,
            true => ILIM1_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == ILIM1_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == ILIM1_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for ILIM1_F_R {
    type Target = crate::FieldReader<bool, ILIM1_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Match interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<IMAT1_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT1_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT1_F` reader - Match interrupt flag for channel 1."]
pub struct IMAT1_F_R(crate::FieldReader<bool, IMAT1_F_A>);
impl IMAT1_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMAT1_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT1_F_A {
        match self.bits {
            false => IMAT1_F_A::THIS_INTERRUPT_SOURC,
            true => IMAT1_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == IMAT1_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == IMAT1_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for IMAT1_F_R {
    type Target = crate::FieldReader<bool, IMAT1_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Capture interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP1_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ICAP1_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP1_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP1_F` reader - Capture interrupt flag for channel 1."]
pub struct ICAP1_F_R(crate::FieldReader<bool, ICAP1_F_A>);
impl ICAP1_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICAP1_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP1_F_A {
        match self.bits {
            false => ICAP1_F_A::THIS_INTERRUPT_SOURC,
            true => ICAP1_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == ICAP1_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == ICAP1_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for ICAP1_F_R {
    type Target = crate::FieldReader<bool, ICAP1_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Limit interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIM2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ILIM2_F_A> for bool {
    #[inline(always)]
    fn from(variant: ILIM2_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM2_F` reader - Limit interrupt flag for channel 2."]
pub struct ILIM2_F_R(crate::FieldReader<bool, ILIM2_F_A>);
impl ILIM2_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ILIM2_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIM2_F_A {
        match self.bits {
            false => ILIM2_F_A::THIS_INTERRUPT_SOURC,
            true => ILIM2_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == ILIM2_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == ILIM2_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for ILIM2_F_R {
    type Target = crate::FieldReader<bool, ILIM2_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Match interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMAT2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<IMAT2_F_A> for bool {
    #[inline(always)]
    fn from(variant: IMAT2_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT2_F` reader - Match interrupt flag for channel 2."]
pub struct IMAT2_F_R(crate::FieldReader<bool, IMAT2_F_A>);
impl IMAT2_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMAT2_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMAT2_F_A {
        match self.bits {
            false => IMAT2_F_A::THIS_INTERRUPT_SOURC,
            true => IMAT2_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == IMAT2_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == IMAT2_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for IMAT2_F_R {
    type Target = crate::FieldReader<bool, IMAT2_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Capture interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICAP2_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ICAP2_F_A> for bool {
    #[inline(always)]
    fn from(variant: ICAP2_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP2_F` reader - Capture interrupt flag for channel 2."]
pub struct ICAP2_F_R(crate::FieldReader<bool, ICAP2_F_A>);
impl ICAP2_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICAP2_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICAP2_F_A {
        match self.bits {
            false => ICAP2_F_A::THIS_INTERRUPT_SOURC,
            true => ICAP2_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == ICAP2_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == ICAP2_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for ICAP2_F_R {
    type Target = crate::FieldReader<bool, ICAP2_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Fast abort interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABORT_F_A {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    THIS_INTERRUPT_SOURC = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IF_THE_CORRESPONDING = 1,
}
impl From<ABORT_F_A> for bool {
    #[inline(always)]
    fn from(variant: ABORT_F_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT_F` reader - Fast abort interrupt flag."]
pub struct ABORT_F_R(crate::FieldReader<bool, ABORT_F_A>);
impl ABORT_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABORT_F_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABORT_F_A {
        match self.bits {
            false => ABORT_F_A::THIS_INTERRUPT_SOURC,
            true => ABORT_F_A::IF_THE_CORRESPONDING,
        }
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_SOURC`"]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        **self == ABORT_F_A::THIS_INTERRUPT_SOURC
    }
    #[doc = "Checks if the value of the field is `IF_THE_CORRESPONDING`"]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        **self == ABORT_F_A::IF_THE_CORRESPONDING
    }
}
impl core::ops::Deref for ABORT_F_R {
    type Target = crate::FieldReader<bool, ABORT_F_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt flag for channel 0."]
    #[inline(always)]
    pub fn ilim0_f(&self) -> ILIM0_F_R {
        ILIM0_F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Match interrupt flag for channel 0."]
    #[inline(always)]
    pub fn imat0_f(&self) -> IMAT0_F_R {
        IMAT0_F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt flag for channel 0."]
    #[inline(always)]
    pub fn icap0_f(&self) -> ICAP0_F_R {
        ICAP0_F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt flag for channel 1."]
    #[inline(always)]
    pub fn ilim1_f(&self) -> ILIM1_F_R {
        ILIM1_F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Match interrupt flag for channel 1."]
    #[inline(always)]
    pub fn imat1_f(&self) -> IMAT1_F_R {
        IMAT1_F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt flag for channel 1."]
    #[inline(always)]
    pub fn icap1_f(&self) -> ICAP1_F_R {
        ICAP1_F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt flag for channel 2."]
    #[inline(always)]
    pub fn ilim2_f(&self) -> ILIM2_F_R {
        ILIM2_F_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Match interrupt flag for channel 2."]
    #[inline(always)]
    pub fn imat2_f(&self) -> IMAT2_F_R {
        IMAT2_F_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt flag for channel 2."]
    #[inline(always)]
    pub fn icap2_f(&self) -> ICAP2_F_R {
        ICAP2_F_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt flag."]
    #[inline(always)]
    pub fn abort_f(&self) -> ABORT_F_R {
        ABORT_F_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Interrupt flags read address\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intf](index.html) module"]
pub struct INTF_SPEC;
impl crate::RegisterSpec for INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intf::R](R) reader structure"]
impl crate::Readable for INTF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
