#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN0_A {
    #[doc = "0: Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 0 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN0` reader - Interrupt enable"]
pub struct ADINTEN0_R(crate::FieldReader<bool, ADINTEN0_A>);
impl ADINTEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN0_A {
        match self.bits {
            false => ADINTEN0_A::DISABLE,
            true => ADINTEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADINTEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADINTEN0_A::ENABLE
    }
}
impl core::ops::Deref for ADINTEN0_R {
    type Target = crate::FieldReader<bool, ADINTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINTEN0` writer - Interrupt enable"]
pub struct ADINTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN0_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN1_A {
    #[doc = "0: Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 1 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN1` reader - Interrupt enable"]
pub struct ADINTEN1_R(crate::FieldReader<bool, ADINTEN1_A>);
impl ADINTEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINTEN1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN1_A {
        match self.bits {
            false => ADINTEN1_A::DISABLE,
            true => ADINTEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADINTEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADINTEN1_A::ENABLE
    }
}
impl core::ops::Deref for ADINTEN1_R {
    type Target = crate::FieldReader<bool, ADINTEN1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINTEN1` writer - Interrupt enable"]
pub struct ADINTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN1_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN2_A {
    #[doc = "0: Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 2 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN2` reader - Interrupt enable"]
pub struct ADINTEN2_R(crate::FieldReader<bool, ADINTEN2_A>);
impl ADINTEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINTEN2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN2_A {
        match self.bits {
            false => ADINTEN2_A::DISABLE,
            true => ADINTEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADINTEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADINTEN2_A::ENABLE
    }
}
impl core::ops::Deref for ADINTEN2_R {
    type Target = crate::FieldReader<bool, ADINTEN2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINTEN2` writer - Interrupt enable"]
pub struct ADINTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN2_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN2_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN3_A {
    #[doc = "0: Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 3 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN3` reader - Interrupt enable"]
pub struct ADINTEN3_R(crate::FieldReader<bool, ADINTEN3_A>);
impl ADINTEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINTEN3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN3_A {
        match self.bits {
            false => ADINTEN3_A::DISABLE,
            true => ADINTEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADINTEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADINTEN3_A::ENABLE
    }
}
impl core::ops::Deref for ADINTEN3_R {
    type Target = crate::FieldReader<bool, ADINTEN3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINTEN3` writer - Interrupt enable"]
pub struct ADINTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN3_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN3_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN4_A {
    #[doc = "0: Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 4 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN4` reader - Interrupt enable"]
pub struct ADINTEN4_R(crate::FieldReader<bool, ADINTEN4_A>);
impl ADINTEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINTEN4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN4_A {
        match self.bits {
            false => ADINTEN4_A::DISABLE,
            true => ADINTEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADINTEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADINTEN4_A::ENABLE
    }
}
impl core::ops::Deref for ADINTEN4_R {
    type Target = crate::FieldReader<bool, ADINTEN4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINTEN4` writer - Interrupt enable"]
pub struct ADINTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN4_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN4_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN5_A {
    #[doc = "0: Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 5 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN5` reader - Interrupt enable"]
pub struct ADINTEN5_R(crate::FieldReader<bool, ADINTEN5_A>);
impl ADINTEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINTEN5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN5_A {
        match self.bits {
            false => ADINTEN5_A::DISABLE,
            true => ADINTEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADINTEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADINTEN5_A::ENABLE
    }
}
impl core::ops::Deref for ADINTEN5_R {
    type Target = crate::FieldReader<bool, ADINTEN5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINTEN5` writer - Interrupt enable"]
pub struct ADINTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN5_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN5_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN6_A {
    #[doc = "0: Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 6 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN6_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN6` reader - Interrupt enable"]
pub struct ADINTEN6_R(crate::FieldReader<bool, ADINTEN6_A>);
impl ADINTEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINTEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN6_A {
        match self.bits {
            false => ADINTEN6_A::DISABLE,
            true => ADINTEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADINTEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADINTEN6_A::ENABLE
    }
}
impl core::ops::Deref for ADINTEN6_R {
    type Target = crate::FieldReader<bool, ADINTEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINTEN6` writer - Interrupt enable"]
pub struct ADINTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN6_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN6_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN7_A {
    #[doc = "0: Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 7 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN7_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN7` reader - Interrupt enable"]
pub struct ADINTEN7_R(crate::FieldReader<bool, ADINTEN7_A>);
impl ADINTEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADINTEN7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN7_A {
        match self.bits {
            false => ADINTEN7_A::DISABLE,
            true => ADINTEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADINTEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADINTEN7_A::ENABLE
    }
}
impl core::ops::Deref for ADINTEN7_R {
    type Target = crate::FieldReader<bool, ADINTEN7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADINTEN7` writer - Interrupt enable"]
pub struct ADINTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN7_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN7_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADGINTEN_A {
    #[doc = "0: Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    CHANNELS = 0,
    #[doc = "1: The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    GLOBAL = 1,
}
impl From<ADGINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADGINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADGINTEN` reader - Interrupt enable"]
pub struct ADGINTEN_R(crate::FieldReader<bool, ADGINTEN_A>);
impl ADGINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADGINTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADGINTEN_A {
        match self.bits {
            false => ADGINTEN_A::CHANNELS,
            true => ADGINTEN_A::GLOBAL,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNELS`"]
    #[inline(always)]
    pub fn is_channels(&self) -> bool {
        **self == ADGINTEN_A::CHANNELS
    }
    #[doc = "Checks if the value of the field is `GLOBAL`"]
    #[inline(always)]
    pub fn is_global(&self) -> bool {
        **self == ADGINTEN_A::GLOBAL
    }
}
impl core::ops::Deref for ADGINTEN_R {
    type Target = crate::FieldReader<bool, ADGINTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADGINTEN` writer - Interrupt enable"]
pub struct ADGINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADGINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADGINTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    #[inline(always)]
    pub fn channels(self) -> &'a mut W {
        self.variant(ADGINTEN_A::CHANNELS)
    }
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    #[inline(always)]
    pub fn global(self) -> &'a mut W {
        self.variant(ADGINTEN_A::GLOBAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&self) -> ADINTEN0_R {
        ADINTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&self) -> ADINTEN1_R {
        ADINTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&self) -> ADINTEN2_R {
        ADINTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&self) -> ADINTEN3_R {
        ADINTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&self) -> ADINTEN4_R {
        ADINTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&self) -> ADINTEN5_R {
        ADINTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&self) -> ADINTEN6_R {
        ADINTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&self) -> ADINTEN7_R {
        ADINTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&self) -> ADGINTEN_R {
        ADGINTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&mut self) -> ADINTEN0_W {
        ADINTEN0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&mut self) -> ADINTEN1_W {
        ADINTEN1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&mut self) -> ADINTEN2_W {
        ADINTEN2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&mut self) -> ADINTEN3_W {
        ADINTEN3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&mut self) -> ADINTEN4_W {
        ADINTEN4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&mut self) -> ADINTEN5_W {
        ADINTEN5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&mut self) -> ADINTEN6_W {
        ADINTEN6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&mut self) -> ADINTEN7_W {
        ADINTEN7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&mut self) -> ADGINTEN_W {
        ADGINTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0x0100"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
