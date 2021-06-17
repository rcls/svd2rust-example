#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Controls the current source used by the comparators. These bits must be set when either comparator is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP_PD_IREF_A {
    #[doc = "0: The comparator current source is disabled."]
    DISABLED = 0,
    #[doc = "1: The comparator current source is disabled in Deep Sleep and Power-down modes and restored automatically when exiting those modes."]
    DIS_DEEPSLP_PWRDWN = 1,
    #[doc = "2: The comparator current source is disabled in Power-down mode and restored automatically when exiting Power-down."]
    DIS_PWRDWN = 2,
    #[doc = "3: The comparator current source is powered up."]
    ENABLED = 3,
}
impl From<CMP_PD_IREF_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_PD_IREF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP_PD_IREF` reader - Controls the current source used by the comparators. These bits must be set when either comparator is used."]
pub struct CMP_PD_IREF_R(crate::FieldReader<u8, CMP_PD_IREF_A>);
impl CMP_PD_IREF_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_PD_IREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_PD_IREF_A {
        match self.bits {
            0 => CMP_PD_IREF_A::DISABLED,
            1 => CMP_PD_IREF_A::DIS_DEEPSLP_PWRDWN,
            2 => CMP_PD_IREF_A::DIS_PWRDWN,
            3 => CMP_PD_IREF_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMP_PD_IREF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIS_DEEPSLP_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_deepslp_pwrdwn(&self) -> bool {
        **self == CMP_PD_IREF_A::DIS_DEEPSLP_PWRDWN
    }
    #[doc = "Checks if the value of the field is `DIS_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_pwrdwn(&self) -> bool {
        **self == CMP_PD_IREF_A::DIS_PWRDWN
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMP_PD_IREF_A::ENABLED
    }
}
impl core::ops::Deref for CMP_PD_IREF_R {
    type Target = crate::FieldReader<u8, CMP_PD_IREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_PD_IREF` writer - Controls the current source used by the comparators. These bits must be set when either comparator is used."]
pub struct CMP_PD_IREF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_PD_IREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_PD_IREF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The comparator current source is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP_PD_IREF_A::DISABLED)
    }
    #[doc = "The comparator current source is disabled in Deep Sleep and Power-down modes and restored automatically when exiting those modes."]
    #[inline(always)]
    pub fn dis_deepslp_pwrdwn(self) -> &'a mut W {
        self.variant(CMP_PD_IREF_A::DIS_DEEPSLP_PWRDWN)
    }
    #[doc = "The comparator current source is disabled in Power-down mode and restored automatically when exiting Power-down."]
    #[inline(always)]
    pub fn dis_pwrdwn(self) -> &'a mut W {
        self.variant(CMP_PD_IREF_A::DIS_PWRDWN)
    }
    #[doc = "The comparator current source is powered up."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP_PD_IREF_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Controls the bandgap reference source that is used by the comparators. These bits must be set when either comparator is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP_PD_VBG_A {
    #[doc = "0: The comparator bandgap reference is disabled."]
    DISABLED = 0,
    #[doc = "1: The comparator bandgap reference is disabled in Deep Sleep and Power-down modes and restored automatically when exiting those modes."]
    DIS_DEEPSLP_PWRDWN = 1,
    #[doc = "2: The comparator bandgap reference is disabled in Power-down mode and restored automatically when exiting Power-down."]
    DIS_PWRDWN = 2,
    #[doc = "3: The comparator bandgap reference is powered up."]
    ENABLED = 3,
}
impl From<CMP_PD_VBG_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_PD_VBG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP_PD_VBG` reader - Controls the bandgap reference source that is used by the comparators. These bits must be set when either comparator is used."]
pub struct CMP_PD_VBG_R(crate::FieldReader<u8, CMP_PD_VBG_A>);
impl CMP_PD_VBG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_PD_VBG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_PD_VBG_A {
        match self.bits {
            0 => CMP_PD_VBG_A::DISABLED,
            1 => CMP_PD_VBG_A::DIS_DEEPSLP_PWRDWN,
            2 => CMP_PD_VBG_A::DIS_PWRDWN,
            3 => CMP_PD_VBG_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMP_PD_VBG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIS_DEEPSLP_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_deepslp_pwrdwn(&self) -> bool {
        **self == CMP_PD_VBG_A::DIS_DEEPSLP_PWRDWN
    }
    #[doc = "Checks if the value of the field is `DIS_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_pwrdwn(&self) -> bool {
        **self == CMP_PD_VBG_A::DIS_PWRDWN
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMP_PD_VBG_A::ENABLED
    }
}
impl core::ops::Deref for CMP_PD_VBG_R {
    type Target = crate::FieldReader<u8, CMP_PD_VBG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_PD_VBG` writer - Controls the bandgap reference source that is used by the comparators. These bits must be set when either comparator is used."]
pub struct CMP_PD_VBG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_PD_VBG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_PD_VBG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The comparator bandgap reference is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP_PD_VBG_A::DISABLED)
    }
    #[doc = "The comparator bandgap reference is disabled in Deep Sleep and Power-down modes and restored automatically when exiting those modes."]
    #[inline(always)]
    pub fn dis_deepslp_pwrdwn(self) -> &'a mut W {
        self.variant(CMP_PD_VBG_A::DIS_DEEPSLP_PWRDWN)
    }
    #[doc = "The comparator bandgap reference is disabled in Power-down mode and restored automatically when exiting Power-down."]
    #[inline(always)]
    pub fn dis_pwrdwn(self) -> &'a mut W {
        self.variant(CMP_PD_VBG_A::DIS_PWRDWN)
    }
    #[doc = "The comparator bandgap reference is powered up."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP_PD_VBG_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Controls the voltage reference of the temperature sensor. These bits must be set when the temperature sensor is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP_VTEMP_A {
    #[doc = "0: The temperature sensor voltage reference is disabled."]
    DISABLED = 0,
    #[doc = "1: The temperature sensor voltage reference is disabled in Deep Sleep and Power-down modes and restored automatically when exiting those modes."]
    DIS_DEEPSLP_PWRDWN = 1,
    #[doc = "2: The temperature sensor voltage reference is disabled in Power-down mode and restored automatically when exiting Power-down."]
    DIS_PWRDWN = 2,
    #[doc = "3: The temperature sensor voltage reference is powered up."]
    ENABLED = 3,
}
impl From<CMP_VTEMP_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_VTEMP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP_VTEMP` reader - Controls the voltage reference of the temperature sensor. These bits must be set when the temperature sensor is used."]
pub struct CMP_VTEMP_R(crate::FieldReader<u8, CMP_VTEMP_A>);
impl CMP_VTEMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_VTEMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_VTEMP_A {
        match self.bits {
            0 => CMP_VTEMP_A::DISABLED,
            1 => CMP_VTEMP_A::DIS_DEEPSLP_PWRDWN,
            2 => CMP_VTEMP_A::DIS_PWRDWN,
            3 => CMP_VTEMP_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMP_VTEMP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIS_DEEPSLP_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_deepslp_pwrdwn(&self) -> bool {
        **self == CMP_VTEMP_A::DIS_DEEPSLP_PWRDWN
    }
    #[doc = "Checks if the value of the field is `DIS_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_pwrdwn(&self) -> bool {
        **self == CMP_VTEMP_A::DIS_PWRDWN
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMP_VTEMP_A::ENABLED
    }
}
impl core::ops::Deref for CMP_VTEMP_R {
    type Target = crate::FieldReader<u8, CMP_VTEMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_VTEMP` writer - Controls the voltage reference of the temperature sensor. These bits must be set when the temperature sensor is used."]
pub struct CMP_VTEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_VTEMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_VTEMP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The temperature sensor voltage reference is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP_VTEMP_A::DISABLED)
    }
    #[doc = "The temperature sensor voltage reference is disabled in Deep Sleep and Power-down modes and restored automatically when exiting those modes."]
    #[inline(always)]
    pub fn dis_deepslp_pwrdwn(self) -> &'a mut W {
        self.variant(CMP_VTEMP_A::DIS_DEEPSLP_PWRDWN)
    }
    #[doc = "The temperature sensor voltage reference is disabled in Power-down mode and restored automatically when exiting Power-down."]
    #[inline(always)]
    pub fn dis_pwrdwn(self) -> &'a mut W {
        self.variant(CMP_VTEMP_A::DIS_PWRDWN)
    }
    #[doc = "The temperature sensor voltage reference is powered up."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP_VTEMP_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Enables the temperature sensor. These bits must be set when the temperature sensor is used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP_TEMPSEN_A {
    #[doc = "0: Temperature sensor is disabled."]
    DISABLED = 0,
    #[doc = "1: The temperature sensor is disabled in Deep Sleep and Power-down modes and restored automatically when exiting those modes."]
    DIS_DEEPSLP_PWRDWN = 1,
    #[doc = "2: The temperature sensor is disabled in Power-down mode and restored automatically when exiting Power-down."]
    DIS_PWRDWN = 2,
    #[doc = "3: Temperature sensor is enabled."]
    ENABLED = 3,
}
impl From<CMP_TEMPSEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP_TEMPSEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP_TEMPSEN` reader - Enables the temperature sensor. These bits must be set when the temperature sensor is used."]
pub struct CMP_TEMPSEN_R(crate::FieldReader<u8, CMP_TEMPSEN_A>);
impl CMP_TEMPSEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP_TEMPSEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_TEMPSEN_A {
        match self.bits {
            0 => CMP_TEMPSEN_A::DISABLED,
            1 => CMP_TEMPSEN_A::DIS_DEEPSLP_PWRDWN,
            2 => CMP_TEMPSEN_A::DIS_PWRDWN,
            3 => CMP_TEMPSEN_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMP_TEMPSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIS_DEEPSLP_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_deepslp_pwrdwn(&self) -> bool {
        **self == CMP_TEMPSEN_A::DIS_DEEPSLP_PWRDWN
    }
    #[doc = "Checks if the value of the field is `DIS_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_pwrdwn(&self) -> bool {
        **self == CMP_TEMPSEN_A::DIS_PWRDWN
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMP_TEMPSEN_A::ENABLED
    }
}
impl core::ops::Deref for CMP_TEMPSEN_R {
    type Target = crate::FieldReader<u8, CMP_TEMPSEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_TEMPSEN` writer - Enables the temperature sensor. These bits must be set when the temperature sensor is used."]
pub struct CMP_TEMPSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_TEMPSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_TEMPSEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Temperature sensor is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP_TEMPSEN_A::DISABLED)
    }
    #[doc = "The temperature sensor is disabled in Deep Sleep and Power-down modes and restored automatically when exiting those modes."]
    #[inline(always)]
    pub fn dis_deepslp_pwrdwn(self) -> &'a mut W {
        self.variant(CMP_TEMPSEN_A::DIS_DEEPSLP_PWRDWN)
    }
    #[doc = "The temperature sensor is disabled in Power-down mode and restored automatically when exiting Power-down."]
    #[inline(always)]
    pub fn dis_pwrdwn(self) -> &'a mut W {
        self.variant(CMP_TEMPSEN_A::DIS_PWRDWN)
    }
    #[doc = "Temperature sensor is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP_TEMPSEN_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Selects the inputs for the flip/flops that provide the CMP_ROSC output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_ROSCCTL_A {
    #[doc = "0: The CMP_ROSC output is set by CMP1 and reset by CMP0."]
    CMP1 = 0,
    #[doc = "1: The CMP_ROSC output is set by CMP0 and reset by CMP1."]
    CMP0 = 1,
}
impl From<CMP_ROSCCTL_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_ROSCCTL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_ROSCCTL` reader - Selects the inputs for the flip/flops that provide the CMP_ROSC output."]
pub struct CMP_ROSCCTL_R(crate::FieldReader<bool, CMP_ROSCCTL_A>);
impl CMP_ROSCCTL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_ROSCCTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_ROSCCTL_A {
        match self.bits {
            false => CMP_ROSCCTL_A::CMP1,
            true => CMP_ROSCCTL_A::CMP0,
        }
    }
    #[doc = "Checks if the value of the field is `CMP1`"]
    #[inline(always)]
    pub fn is_cmp1(&self) -> bool {
        **self == CMP_ROSCCTL_A::CMP1
    }
    #[doc = "Checks if the value of the field is `CMP0`"]
    #[inline(always)]
    pub fn is_cmp0(&self) -> bool {
        **self == CMP_ROSCCTL_A::CMP0
    }
}
impl core::ops::Deref for CMP_ROSCCTL_R {
    type Target = crate::FieldReader<bool, CMP_ROSCCTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_ROSCCTL` writer - Selects the inputs for the flip/flops that provide the CMP_ROSC output."]
pub struct CMP_ROSCCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_ROSCCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_ROSCCTL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The CMP_ROSC output is set by CMP1 and reset by CMP0."]
    #[inline(always)]
    pub fn cmp1(self) -> &'a mut W {
        self.variant(CMP_ROSCCTL_A::CMP1)
    }
    #[doc = "The CMP_ROSC output is set by CMP0 and reset by CMP1."]
    #[inline(always)]
    pub fn cmp0(self) -> &'a mut W {
        self.variant(CMP_ROSCCTL_A::CMP0)
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
#[doc = "Selects the reset source for the CMP_ROSC output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_EXT_RESET_A {
    #[doc = "0: The CMP_ROSC output is reset by the internal chip reset."]
    INTRESET = 0,
    #[doc = "1: The CMP_ROSC output is reset by the CMP_RESET input."]
    CMP_RESETIN = 1,
}
impl From<CMP_EXT_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_EXT_RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_EXT_RESET` reader - Selects the reset source for the CMP_ROSC output."]
pub struct CMP_EXT_RESET_R(crate::FieldReader<bool, CMP_EXT_RESET_A>);
impl CMP_EXT_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_EXT_RESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_EXT_RESET_A {
        match self.bits {
            false => CMP_EXT_RESET_A::INTRESET,
            true => CMP_EXT_RESET_A::CMP_RESETIN,
        }
    }
    #[doc = "Checks if the value of the field is `INTRESET`"]
    #[inline(always)]
    pub fn is_intreset(&self) -> bool {
        **self == CMP_EXT_RESET_A::INTRESET
    }
    #[doc = "Checks if the value of the field is `CMP_RESETIN`"]
    #[inline(always)]
    pub fn is_cmp_resetin(&self) -> bool {
        **self == CMP_EXT_RESET_A::CMP_RESETIN
    }
}
impl core::ops::Deref for CMP_EXT_RESET_R {
    type Target = crate::FieldReader<bool, CMP_EXT_RESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_EXT_RESET` writer - Selects the reset source for the CMP_ROSC output."]
pub struct CMP_EXT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_EXT_RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_EXT_RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The CMP_ROSC output is reset by the internal chip reset."]
    #[inline(always)]
    pub fn intreset(self) -> &'a mut W {
        self.variant(CMP_EXT_RESET_A::INTRESET)
    }
    #[doc = "The CMP_ROSC output is reset by the CMP_RESET input."]
    #[inline(always)]
    pub fn cmp_resetin(self) -> &'a mut W {
        self.variant(CMP_EXT_RESET_A::CMP_RESETIN)
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
#[doc = "Selects the input for Timer 0 capture input 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_T0CAP2_A {
    #[doc = "0: T0CAP2 is connected to comparator 0 level output."]
    COMP0 = 0,
    #[doc = "1: T0CAP2 is connected to comparator 1 level output."]
    COMP1 = 1,
}
impl From<CMP_T0CAP2_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_T0CAP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_T0CAP2` reader - Selects the input for Timer 0 capture input 2."]
pub struct CMP_T0CAP2_R(crate::FieldReader<bool, CMP_T0CAP2_A>);
impl CMP_T0CAP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_T0CAP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_T0CAP2_A {
        match self.bits {
            false => CMP_T0CAP2_A::COMP0,
            true => CMP_T0CAP2_A::COMP1,
        }
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        **self == CMP_T0CAP2_A::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        **self == CMP_T0CAP2_A::COMP1
    }
}
impl core::ops::Deref for CMP_T0CAP2_R {
    type Target = crate::FieldReader<bool, CMP_T0CAP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_T0CAP2` writer - Selects the input for Timer 0 capture input 2."]
pub struct CMP_T0CAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_T0CAP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_T0CAP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "T0CAP2 is connected to comparator 0 level output."]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(CMP_T0CAP2_A::COMP0)
    }
    #[doc = "T0CAP2 is connected to comparator 1 level output."]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(CMP_T0CAP2_A::COMP1)
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
#[doc = "Selects the input for Timer 0 capture input 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_T0CAP3_A {
    #[doc = "0: T0CAP3 is connected to comparator 0 edge output."]
    COMP0 = 0,
    #[doc = "1: T0CAP3 is connected to comparator 1 edge output."]
    COMP1 = 1,
}
impl From<CMP_T0CAP3_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_T0CAP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_T0CAP3` reader - Selects the input for Timer 0 capture input 3."]
pub struct CMP_T0CAP3_R(crate::FieldReader<bool, CMP_T0CAP3_A>);
impl CMP_T0CAP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_T0CAP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_T0CAP3_A {
        match self.bits {
            false => CMP_T0CAP3_A::COMP0,
            true => CMP_T0CAP3_A::COMP1,
        }
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        **self == CMP_T0CAP3_A::COMP0
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        **self == CMP_T0CAP3_A::COMP1
    }
}
impl core::ops::Deref for CMP_T0CAP3_R {
    type Target = crate::FieldReader<bool, CMP_T0CAP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_T0CAP3` writer - Selects the input for Timer 0 capture input 3."]
pub struct CMP_T0CAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_T0CAP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_T0CAP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "T0CAP3 is connected to comparator 0 edge output."]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(CMP_T0CAP3_A::COMP0)
    }
    #[doc = "T0CAP3 is connected to comparator 1 edge output."]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(CMP_T0CAP3_A::COMP1)
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
#[doc = "Selects the input for Timer 1 capture input 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_T1CAP2_A {
    #[doc = "0: T1CAP2 is connected to comparator 1 edge output."]
    COMP1 = 0,
    #[doc = "1: T1CAP2 is connected to comparator 0 level output."]
    COMP0 = 1,
}
impl From<CMP_T1CAP2_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_T1CAP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_T1CAP2` reader - Selects the input for Timer 1 capture input 2."]
pub struct CMP_T1CAP2_R(crate::FieldReader<bool, CMP_T1CAP2_A>);
impl CMP_T1CAP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_T1CAP2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_T1CAP2_A {
        match self.bits {
            false => CMP_T1CAP2_A::COMP1,
            true => CMP_T1CAP2_A::COMP0,
        }
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        **self == CMP_T1CAP2_A::COMP1
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        **self == CMP_T1CAP2_A::COMP0
    }
}
impl core::ops::Deref for CMP_T1CAP2_R {
    type Target = crate::FieldReader<bool, CMP_T1CAP2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_T1CAP2` writer - Selects the input for Timer 1 capture input 2."]
pub struct CMP_T1CAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_T1CAP2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_T1CAP2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "T1CAP2 is connected to comparator 1 edge output."]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(CMP_T1CAP2_A::COMP1)
    }
    #[doc = "T1CAP2 is connected to comparator 0 level output."]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(CMP_T1CAP2_A::COMP0)
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
#[doc = "Selects the input for Timer 1 capture input 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_T1CAP3_A {
    #[doc = "0: T1CAP3 is connected to comparator 1 level output."]
    COMP1 = 0,
    #[doc = "1: T1CAP3 is connected to comparator 0 edge output."]
    COMP0 = 1,
}
impl From<CMP_T1CAP3_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_T1CAP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_T1CAP3` reader - Selects the input for Timer 1 capture input 3."]
pub struct CMP_T1CAP3_R(crate::FieldReader<bool, CMP_T1CAP3_A>);
impl CMP_T1CAP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_T1CAP3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_T1CAP3_A {
        match self.bits {
            false => CMP_T1CAP3_A::COMP1,
            true => CMP_T1CAP3_A::COMP0,
        }
    }
    #[doc = "Checks if the value of the field is `COMP1`"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        **self == CMP_T1CAP3_A::COMP1
    }
    #[doc = "Checks if the value of the field is `COMP0`"]
    #[inline(always)]
    pub fn is_comp0(&self) -> bool {
        **self == CMP_T1CAP3_A::COMP0
    }
}
impl core::ops::Deref for CMP_T1CAP3_R {
    type Target = crate::FieldReader<bool, CMP_T1CAP3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_T1CAP3` writer - Selects the input for Timer 1 capture input 3."]
pub struct CMP_T1CAP3_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_T1CAP3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_T1CAP3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "T1CAP3 is connected to comparator 1 level output."]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut W {
        self.variant(CMP_T1CAP3_A::COMP1)
    }
    #[doc = "T1CAP3 is connected to comparator 0 edge output."]
    #[inline(always)]
    pub fn comp0(self) -> &'a mut W {
        self.variant(CMP_T1CAP3_A::COMP0)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Controls the current source used by the comparators. These bits must be set when either comparator is used."]
    #[inline(always)]
    pub fn cmp_pd_iref(&self) -> CMP_PD_IREF_R {
        CMP_PD_IREF_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Controls the bandgap reference source that is used by the comparators. These bits must be set when either comparator is used."]
    #[inline(always)]
    pub fn cmp_pd_vbg(&self) -> CMP_PD_VBG_R {
        CMP_PD_VBG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Controls the voltage reference of the temperature sensor. These bits must be set when the temperature sensor is used."]
    #[inline(always)]
    pub fn cmp_vtemp(&self) -> CMP_VTEMP_R {
        CMP_VTEMP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Enables the temperature sensor. These bits must be set when the temperature sensor is used."]
    #[inline(always)]
    pub fn cmp_tempsen(&self) -> CMP_TEMPSEN_R {
        CMP_TEMPSEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Selects the inputs for the flip/flops that provide the CMP_ROSC output."]
    #[inline(always)]
    pub fn cmp_roscctl(&self) -> CMP_ROSCCTL_R {
        CMP_ROSCCTL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Selects the reset source for the CMP_ROSC output."]
    #[inline(always)]
    pub fn cmp_ext_reset(&self) -> CMP_EXT_RESET_R {
        CMP_EXT_RESET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Selects the input for Timer 0 capture input 2."]
    #[inline(always)]
    pub fn cmp_t0cap2(&self) -> CMP_T0CAP2_R {
        CMP_T0CAP2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Selects the input for Timer 0 capture input 3."]
    #[inline(always)]
    pub fn cmp_t0cap3(&self) -> CMP_T0CAP3_R {
        CMP_T0CAP3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Selects the input for Timer 1 capture input 2."]
    #[inline(always)]
    pub fn cmp_t1cap2(&self) -> CMP_T1CAP2_R {
        CMP_T1CAP2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects the input for Timer 1 capture input 3."]
    #[inline(always)]
    pub fn cmp_t1cap3(&self) -> CMP_T1CAP3_R {
        CMP_T1CAP3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls the current source used by the comparators. These bits must be set when either comparator is used."]
    #[inline(always)]
    pub fn cmp_pd_iref(&mut self) -> CMP_PD_IREF_W {
        CMP_PD_IREF_W { w: self }
    }
    #[doc = "Bits 2:3 - Controls the bandgap reference source that is used by the comparators. These bits must be set when either comparator is used."]
    #[inline(always)]
    pub fn cmp_pd_vbg(&mut self) -> CMP_PD_VBG_W {
        CMP_PD_VBG_W { w: self }
    }
    #[doc = "Bits 4:5 - Controls the voltage reference of the temperature sensor. These bits must be set when the temperature sensor is used."]
    #[inline(always)]
    pub fn cmp_vtemp(&mut self) -> CMP_VTEMP_W {
        CMP_VTEMP_W { w: self }
    }
    #[doc = "Bits 6:7 - Enables the temperature sensor. These bits must be set when the temperature sensor is used."]
    #[inline(always)]
    pub fn cmp_tempsen(&mut self) -> CMP_TEMPSEN_W {
        CMP_TEMPSEN_W { w: self }
    }
    #[doc = "Bit 8 - Selects the inputs for the flip/flops that provide the CMP_ROSC output."]
    #[inline(always)]
    pub fn cmp_roscctl(&mut self) -> CMP_ROSCCTL_W {
        CMP_ROSCCTL_W { w: self }
    }
    #[doc = "Bit 9 - Selects the reset source for the CMP_ROSC output."]
    #[inline(always)]
    pub fn cmp_ext_reset(&mut self) -> CMP_EXT_RESET_W {
        CMP_EXT_RESET_W { w: self }
    }
    #[doc = "Bit 12 - Selects the input for Timer 0 capture input 2."]
    #[inline(always)]
    pub fn cmp_t0cap2(&mut self) -> CMP_T0CAP2_W {
        CMP_T0CAP2_W { w: self }
    }
    #[doc = "Bit 13 - Selects the input for Timer 0 capture input 3."]
    #[inline(always)]
    pub fn cmp_t0cap3(&mut self) -> CMP_T0CAP3_W {
        CMP_T0CAP3_W { w: self }
    }
    #[doc = "Bit 14 - Selects the input for Timer 1 capture input 2."]
    #[inline(always)]
    pub fn cmp_t1cap2(&mut self) -> CMP_T1CAP2_W {
        CMP_T1CAP2_W { w: self }
    }
    #[doc = "Bit 15 - Selects the input for Timer 1 capture input 3."]
    #[inline(always)]
    pub fn cmp_t1cap3(&mut self) -> CMP_T1CAP3_W {
        CMP_T1CAP3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator block control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
