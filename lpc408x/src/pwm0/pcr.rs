#[doc = "Register `PCR` reader"]
pub struct R(crate::R<PCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCR` writer"]
pub struct W(crate::W<PCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PWM\\[2\\]
output single/double edge mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL2_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL = 1,
}
impl From<PWMSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL2` reader - PWM\\[2\\]
output single/double edge mode control."]
pub struct PWMSEL2_R(crate::FieldReader<bool, PWMSEL2_A>);
impl PWMSEL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMSEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL2_A {
        match self.bits {
            false => PWMSEL2_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL2_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        **self == PWMSEL2_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        **self == PWMSEL2_A::DOUBLE_EDGE_CONTROLL
    }
}
impl core::ops::Deref for PWMSEL2_R {
    type Target = crate::FieldReader<bool, PWMSEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMSEL2` writer - PWM\\[2\\]
output single/double edge mode control."]
pub struct PWMSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL2_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL2_A::DOUBLE_EDGE_CONTROLL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "PWM\\[3\\]
output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL3_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL = 1,
}
impl From<PWMSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL3` reader - PWM\\[3\\]
output edge control."]
pub struct PWMSEL3_R(crate::FieldReader<bool, PWMSEL3_A>);
impl PWMSEL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMSEL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL3_A {
        match self.bits {
            false => PWMSEL3_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL3_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        **self == PWMSEL3_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        **self == PWMSEL3_A::DOUBLE_EDGE_CONTROLL
    }
}
impl core::ops::Deref for PWMSEL3_R {
    type Target = crate::FieldReader<bool, PWMSEL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMSEL3` writer - PWM\\[3\\]
output edge control."]
pub struct PWMSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL3_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL3_A::DOUBLE_EDGE_CONTROLL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "PWM\\[4\\]
output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL4_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL = 1,
}
impl From<PWMSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL4` reader - PWM\\[4\\]
output edge control."]
pub struct PWMSEL4_R(crate::FieldReader<bool, PWMSEL4_A>);
impl PWMSEL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMSEL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL4_A {
        match self.bits {
            false => PWMSEL4_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL4_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        **self == PWMSEL4_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        **self == PWMSEL4_A::DOUBLE_EDGE_CONTROLL
    }
}
impl core::ops::Deref for PWMSEL4_R {
    type Target = crate::FieldReader<bool, PWMSEL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMSEL4` writer - PWM\\[4\\]
output edge control."]
pub struct PWMSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL4_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL4_A::DOUBLE_EDGE_CONTROLL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "PWM\\[5\\]
output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL5_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL = 1,
}
impl From<PWMSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL5` reader - PWM\\[5\\]
output edge control."]
pub struct PWMSEL5_R(crate::FieldReader<bool, PWMSEL5_A>);
impl PWMSEL5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMSEL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL5_A {
        match self.bits {
            false => PWMSEL5_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL5_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        **self == PWMSEL5_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        **self == PWMSEL5_A::DOUBLE_EDGE_CONTROLL
    }
}
impl core::ops::Deref for PWMSEL5_R {
    type Target = crate::FieldReader<bool, PWMSEL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMSEL5` writer - PWM\\[5\\]
output edge control."]
pub struct PWMSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL5_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL5_A::DOUBLE_EDGE_CONTROLL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "PWM\\[6\\]
output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL6_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL = 1,
}
impl From<PWMSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL6` reader - PWM\\[6\\]
output edge control."]
pub struct PWMSEL6_R(crate::FieldReader<bool, PWMSEL6_A>);
impl PWMSEL6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMSEL6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL6_A {
        match self.bits {
            false => PWMSEL6_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL6_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        **self == PWMSEL6_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        **self == PWMSEL6_A::DOUBLE_EDGE_CONTROLL
    }
}
impl core::ops::Deref for PWMSEL6_R {
    type Target = crate::FieldReader<bool, PWMSEL6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMSEL6` writer - PWM\\[6\\]
output edge control."]
pub struct PWMSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL6_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL6_A::DOUBLE_EDGE_CONTROLL)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "PWM\\[1\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA1_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI = 0,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN = 1,
}
impl From<PWMENA1_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA1` reader - PWM\\[1\\]
output enable control."]
pub struct PWMENA1_R(crate::FieldReader<bool, PWMENA1_A>);
impl PWMENA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMENA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA1_A {
        match self.bits {
            false => PWMENA1_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA1_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        **self == PWMENA1_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        **self == PWMENA1_A::THE_PWM_OUTPUT_IS_EN
    }
}
impl core::ops::Deref for PWMENA1_R {
    type Target = crate::FieldReader<bool, PWMENA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMENA1` writer - PWM\\[1\\]
output enable control."]
pub struct PWMENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA1_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA1_A::THE_PWM_OUTPUT_IS_EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "PWM\\[2\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA2_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI = 0,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN = 1,
}
impl From<PWMENA2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA2` reader - PWM\\[2\\]
output enable control."]
pub struct PWMENA2_R(crate::FieldReader<bool, PWMENA2_A>);
impl PWMENA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMENA2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA2_A {
        match self.bits {
            false => PWMENA2_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA2_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        **self == PWMENA2_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        **self == PWMENA2_A::THE_PWM_OUTPUT_IS_EN
    }
}
impl core::ops::Deref for PWMENA2_R {
    type Target = crate::FieldReader<bool, PWMENA2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMENA2` writer - PWM\\[2\\]
output enable control."]
pub struct PWMENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA2_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA2_A::THE_PWM_OUTPUT_IS_EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "PWM\\[3\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA3_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI = 0,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN = 1,
}
impl From<PWMENA3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA3` reader - PWM\\[3\\]
output enable control."]
pub struct PWMENA3_R(crate::FieldReader<bool, PWMENA3_A>);
impl PWMENA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMENA3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA3_A {
        match self.bits {
            false => PWMENA3_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA3_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        **self == PWMENA3_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        **self == PWMENA3_A::THE_PWM_OUTPUT_IS_EN
    }
}
impl core::ops::Deref for PWMENA3_R {
    type Target = crate::FieldReader<bool, PWMENA3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMENA3` writer - PWM\\[3\\]
output enable control."]
pub struct PWMENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA3_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA3_A::THE_PWM_OUTPUT_IS_EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "PWM\\[4\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA4_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI = 0,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN = 1,
}
impl From<PWMENA4_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA4` reader - PWM\\[4\\]
output enable control."]
pub struct PWMENA4_R(crate::FieldReader<bool, PWMENA4_A>);
impl PWMENA4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMENA4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA4_A {
        match self.bits {
            false => PWMENA4_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA4_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        **self == PWMENA4_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        **self == PWMENA4_A::THE_PWM_OUTPUT_IS_EN
    }
}
impl core::ops::Deref for PWMENA4_R {
    type Target = crate::FieldReader<bool, PWMENA4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMENA4` writer - PWM\\[4\\]
output enable control."]
pub struct PWMENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA4_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA4_A::THE_PWM_OUTPUT_IS_EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "PWM\\[5\\]
output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA5_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI = 0,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN = 1,
}
impl From<PWMENA5_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA5` reader - PWM\\[5\\]
output enable control."]
pub struct PWMENA5_R(crate::FieldReader<bool, PWMENA5_A>);
impl PWMENA5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMENA5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA5_A {
        match self.bits {
            false => PWMENA5_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA5_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        **self == PWMENA5_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        **self == PWMENA5_A::THE_PWM_OUTPUT_IS_EN
    }
}
impl core::ops::Deref for PWMENA5_R {
    type Target = crate::FieldReader<bool, PWMENA5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMENA5` writer - PWM\\[5\\]
output enable control."]
pub struct PWMENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA5_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA5_A::THE_PWM_OUTPUT_IS_EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "PWM\\[6\\]
output enable control. See PWMENA1 for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA6_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI = 0,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN = 1,
}
impl From<PWMENA6_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA6` reader - PWM\\[6\\]
output enable control. See PWMENA1 for details."]
pub struct PWMENA6_R(crate::FieldReader<bool, PWMENA6_A>);
impl PWMENA6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMENA6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA6_A {
        match self.bits {
            false => PWMENA6_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA6_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        **self == PWMENA6_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        **self == PWMENA6_A::THE_PWM_OUTPUT_IS_EN
    }
}
impl core::ops::Deref for PWMENA6_R {
    type Target = crate::FieldReader<bool, PWMENA6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMENA6` writer - PWM\\[6\\]
output enable control. See PWMENA1 for details."]
pub struct PWMENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA6_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA6_A::THE_PWM_OUTPUT_IS_EN)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - PWM\\[2\\]
output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&self) -> PWMSEL2_R {
        PWMSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM\\[3\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&self) -> PWMSEL3_R {
        PWMSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM\\[4\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&self) -> PWMSEL4_R {
        PWMSEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM\\[5\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&self) -> PWMSEL5_R {
        PWMSEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM\\[6\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&self) -> PWMSEL6_R {
        PWMSEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWM\\[1\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena1(&self) -> PWMENA1_R {
        PWMENA1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM\\[2\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena2(&self) -> PWMENA2_R {
        PWMENA2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PWM\\[3\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena3(&self) -> PWMENA3_R {
        PWMENA3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM\\[4\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena4(&self) -> PWMENA4_R {
        PWMENA4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PWM\\[5\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena5(&self) -> PWMENA5_R {
        PWMENA5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PWM\\[6\\]
output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&self) -> PWMENA6_R {
        PWMENA6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PWM\\[2\\]
output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&mut self) -> PWMSEL2_W {
        PWMSEL2_W { w: self }
    }
    #[doc = "Bit 3 - PWM\\[3\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&mut self) -> PWMSEL3_W {
        PWMSEL3_W { w: self }
    }
    #[doc = "Bit 4 - PWM\\[4\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&mut self) -> PWMSEL4_W {
        PWMSEL4_W { w: self }
    }
    #[doc = "Bit 5 - PWM\\[5\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&mut self) -> PWMSEL5_W {
        PWMSEL5_W { w: self }
    }
    #[doc = "Bit 6 - PWM\\[6\\]
output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&mut self) -> PWMSEL6_W {
        PWMSEL6_W { w: self }
    }
    #[doc = "Bit 9 - PWM\\[1\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena1(&mut self) -> PWMENA1_W {
        PWMENA1_W { w: self }
    }
    #[doc = "Bit 10 - PWM\\[2\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena2(&mut self) -> PWMENA2_W {
        PWMENA2_W { w: self }
    }
    #[doc = "Bit 11 - PWM\\[3\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena3(&mut self) -> PWMENA3_W {
        PWMENA3_W { w: self }
    }
    #[doc = "Bit 12 - PWM\\[4\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena4(&mut self) -> PWMENA4_W {
        PWMENA4_W { w: self }
    }
    #[doc = "Bit 13 - PWM\\[5\\]
output enable control."]
    #[inline(always)]
    pub fn pwmena5(&mut self) -> PWMENA5_W {
        PWMENA5_W { w: self }
    }
    #[doc = "Bit 14 - PWM\\[6\\]
output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&mut self) -> PWMENA6_W {
        PWMENA6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcr](index.html) module"]
pub struct PCR_SPEC;
impl crate::RegisterSpec for PCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcr::R](R) reader structure"]
impl crate::Readable for PCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcr::W](W) writer structure"]
impl crate::Writable for PCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCR to value 0"]
impl crate::Resettable for PCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
