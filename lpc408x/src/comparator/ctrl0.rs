#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Comparator 0 enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0_EN_A {
    #[doc = "0: Comparator 0 disabled."]
    DISABLED = 0,
    #[doc = "1: Comparator 0 is disabled in Deep Sleep and Power-down modes and re-enabled automatically when exiting those modes."]
    DIS_DEEPSLP_PWRDWN = 1,
    #[doc = "2: Comparator 0 is disabled in Power-down mode and re-enabled automatically when exiting Power-down."]
    DIS_PWRDWN = 2,
    #[doc = "3: Comparator 0 is enabled."]
    ENABLED = 3,
}
impl From<CMP0_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0_EN` reader - Comparator 0 enable control."]
pub struct CMP0_EN_R(crate::FieldReader<u8, CMP0_EN_A>);
impl CMP0_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP0_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_EN_A {
        match self.bits {
            0 => CMP0_EN_A::DISABLED,
            1 => CMP0_EN_A::DIS_DEEPSLP_PWRDWN,
            2 => CMP0_EN_A::DIS_PWRDWN,
            3 => CMP0_EN_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMP0_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIS_DEEPSLP_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_deepslp_pwrdwn(&self) -> bool {
        **self == CMP0_EN_A::DIS_DEEPSLP_PWRDWN
    }
    #[doc = "Checks if the value of the field is `DIS_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_pwrdwn(&self) -> bool {
        **self == CMP0_EN_A::DIS_PWRDWN
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMP0_EN_A::ENABLED
    }
}
impl core::ops::Deref for CMP0_EN_R {
    type Target = crate::FieldReader<u8, CMP0_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_EN` writer - Comparator 0 enable control."]
pub struct CMP0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_EN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Comparator 0 disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP0_EN_A::DISABLED)
    }
    #[doc = "Comparator 0 is disabled in Deep Sleep and Power-down modes and re-enabled automatically when exiting those modes."]
    #[inline(always)]
    pub fn dis_deepslp_pwrdwn(self) -> &'a mut W {
        self.variant(CMP0_EN_A::DIS_DEEPSLP_PWRDWN)
    }
    #[doc = "Comparator 0 is disabled in Power-down mode and re-enabled automatically when exiting Power-down."]
    #[inline(always)]
    pub fn dis_pwrdwn(self) -> &'a mut W {
        self.variant(CMP0_EN_A::DIS_PWRDWN)
    }
    #[doc = "Comparator 0 is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP0_EN_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Comparator 0 output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0_OE_A {
    #[doc = "0: Comparator 0 output is disabled."]
    DISABLED = 0,
    #[doc = "1: Comparator 0 output is enabled."]
    ENABLED = 1,
}
impl From<CMP0_OE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0_OE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0_OE` reader - Comparator 0 output enable."]
pub struct CMP0_OE_R(crate::FieldReader<bool, CMP0_OE_A>);
impl CMP0_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_OE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_OE_A {
        match self.bits {
            false => CMP0_OE_A::DISABLED,
            true => CMP0_OE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMP0_OE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMP0_OE_A::ENABLED
    }
}
impl core::ops::Deref for CMP0_OE_R {
    type Target = crate::FieldReader<bool, CMP0_OE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_OE` writer - Comparator 0 output enable."]
pub struct CMP0_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_OE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_OE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 0 output is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP0_OE_A::DISABLED)
    }
    #[doc = "Comparator 0 output is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP0_OE_A::ENABLED)
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
#[doc = "Field `CMP0_STAT` reader - Comparator 0 status. This bit reflects the comparator 0 output, and is not affected by CMP0_OE."]
pub struct CMP0_STAT_R(crate::FieldReader<bool, bool>);
impl CMP0_STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP0_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_STAT` writer - Comparator 0 status. This bit reflects the comparator 0 output, and is not affected by CMP0_OE."]
pub struct CMP0_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_STAT_W<'a> {
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
#[doc = "Comparator 0 VM input select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0_VM_A {
    #[doc = "0: Vref divider 0."]
    VREF_DIVIDER_0_ = 0,
    #[doc = "1: CMP0_IN\\[0\\]."]
    CMP0_IN0 = 1,
    #[doc = "2: CMP0_IN\\[1\\]."]
    CMP0_IN1 = 2,
    #[doc = "3: CMP0_IN\\[2\\]."]
    CMP0_IN2 = 3,
    #[doc = "4: CMP0_IN\\[3\\]."]
    CMP0_IN3 = 4,
    #[doc = "5: CMP1_IN\\[0\\]."]
    CMP1_IN0 = 5,
    #[doc = "6: internal 0.9 V band gap reference."]
    INTERNAL_0_9_V_BAND_ = 6,
    #[doc = "7: temperature sensor."]
    TEMPERATURE_SENSOR_ = 7,
}
impl From<CMP0_VM_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0_VM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0_VM` reader - Comparator 0 VM input select."]
pub struct CMP0_VM_R(crate::FieldReader<u8, CMP0_VM_A>);
impl CMP0_VM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP0_VM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_VM_A {
        match self.bits {
            0 => CMP0_VM_A::VREF_DIVIDER_0_,
            1 => CMP0_VM_A::CMP0_IN0,
            2 => CMP0_VM_A::CMP0_IN1,
            3 => CMP0_VM_A::CMP0_IN2,
            4 => CMP0_VM_A::CMP0_IN3,
            5 => CMP0_VM_A::CMP1_IN0,
            6 => CMP0_VM_A::INTERNAL_0_9_V_BAND_,
            7 => CMP0_VM_A::TEMPERATURE_SENSOR_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREF_DIVIDER_0_`"]
    #[inline(always)]
    pub fn is_vref_divider_0_(&self) -> bool {
        **self == CMP0_VM_A::VREF_DIVIDER_0_
    }
    #[doc = "Checks if the value of the field is `CMP0_IN0`"]
    #[inline(always)]
    pub fn is_cmp0_in0(&self) -> bool {
        **self == CMP0_VM_A::CMP0_IN0
    }
    #[doc = "Checks if the value of the field is `CMP0_IN1`"]
    #[inline(always)]
    pub fn is_cmp0_in1(&self) -> bool {
        **self == CMP0_VM_A::CMP0_IN1
    }
    #[doc = "Checks if the value of the field is `CMP0_IN2`"]
    #[inline(always)]
    pub fn is_cmp0_in2(&self) -> bool {
        **self == CMP0_VM_A::CMP0_IN2
    }
    #[doc = "Checks if the value of the field is `CMP0_IN3`"]
    #[inline(always)]
    pub fn is_cmp0_in3(&self) -> bool {
        **self == CMP0_VM_A::CMP0_IN3
    }
    #[doc = "Checks if the value of the field is `CMP1_IN0`"]
    #[inline(always)]
    pub fn is_cmp1_in0(&self) -> bool {
        **self == CMP0_VM_A::CMP1_IN0
    }
    #[doc = "Checks if the value of the field is `INTERNAL_0_9_V_BAND_`"]
    #[inline(always)]
    pub fn is_internal_0_9_v_band_(&self) -> bool {
        **self == CMP0_VM_A::INTERNAL_0_9_V_BAND_
    }
    #[doc = "Checks if the value of the field is `TEMPERATURE_SENSOR_`"]
    #[inline(always)]
    pub fn is_temperature_sensor_(&self) -> bool {
        **self == CMP0_VM_A::TEMPERATURE_SENSOR_
    }
}
impl core::ops::Deref for CMP0_VM_R {
    type Target = crate::FieldReader<u8, CMP0_VM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_VM` writer - Comparator 0 VM input select."]
pub struct CMP0_VM_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_VM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_VM_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Vref divider 0."]
    #[inline(always)]
    pub fn vref_divider_0_(self) -> &'a mut W {
        self.variant(CMP0_VM_A::VREF_DIVIDER_0_)
    }
    #[doc = "CMP0_IN\\[0\\]."]
    #[inline(always)]
    pub fn cmp0_in0(self) -> &'a mut W {
        self.variant(CMP0_VM_A::CMP0_IN0)
    }
    #[doc = "CMP0_IN\\[1\\]."]
    #[inline(always)]
    pub fn cmp0_in1(self) -> &'a mut W {
        self.variant(CMP0_VM_A::CMP0_IN1)
    }
    #[doc = "CMP0_IN\\[2\\]."]
    #[inline(always)]
    pub fn cmp0_in2(self) -> &'a mut W {
        self.variant(CMP0_VM_A::CMP0_IN2)
    }
    #[doc = "CMP0_IN\\[3\\]."]
    #[inline(always)]
    pub fn cmp0_in3(self) -> &'a mut W {
        self.variant(CMP0_VM_A::CMP0_IN3)
    }
    #[doc = "CMP1_IN\\[0\\]."]
    #[inline(always)]
    pub fn cmp1_in0(self) -> &'a mut W {
        self.variant(CMP0_VM_A::CMP1_IN0)
    }
    #[doc = "internal 0.9 V band gap reference."]
    #[inline(always)]
    pub fn internal_0_9_v_band_(self) -> &'a mut W {
        self.variant(CMP0_VM_A::INTERNAL_0_9_V_BAND_)
    }
    #[doc = "temperature sensor."]
    #[inline(always)]
    pub fn temperature_sensor_(self) -> &'a mut W {
        self.variant(CMP0_VM_A::TEMPERATURE_SENSOR_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Comparator 0 VP input select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0_VP_A {
    #[doc = "0: Vref divider 0."]
    VREF_DIVIDER_0_ = 0,
    #[doc = "1: CMP0_IN\\[0\\]."]
    CMP0_IN0 = 1,
    #[doc = "2: CMP0_IN\\[1\\]."]
    CMP0_IN1 = 2,
    #[doc = "3: CMP0_IN\\[2\\]."]
    CMP0_IN2 = 3,
    #[doc = "4: CMP0_IN\\[3\\]."]
    CMP0_IN3 = 4,
    #[doc = "5: CMP1_IN\\[0\\]."]
    CMP1_IN0 = 5,
    #[doc = "6: internal 0.9 V band gap reference."]
    INTERNAL_0_9_V_BAND_ = 6,
    #[doc = "7: temperature sensor."]
    TEMPERATURE_SENSOR_ = 7,
}
impl From<CMP0_VP_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0_VP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0_VP` reader - Comparator 0 VP input select."]
pub struct CMP0_VP_R(crate::FieldReader<u8, CMP0_VP_A>);
impl CMP0_VP_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP0_VP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_VP_A {
        match self.bits {
            0 => CMP0_VP_A::VREF_DIVIDER_0_,
            1 => CMP0_VP_A::CMP0_IN0,
            2 => CMP0_VP_A::CMP0_IN1,
            3 => CMP0_VP_A::CMP0_IN2,
            4 => CMP0_VP_A::CMP0_IN3,
            5 => CMP0_VP_A::CMP1_IN0,
            6 => CMP0_VP_A::INTERNAL_0_9_V_BAND_,
            7 => CMP0_VP_A::TEMPERATURE_SENSOR_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREF_DIVIDER_0_`"]
    #[inline(always)]
    pub fn is_vref_divider_0_(&self) -> bool {
        **self == CMP0_VP_A::VREF_DIVIDER_0_
    }
    #[doc = "Checks if the value of the field is `CMP0_IN0`"]
    #[inline(always)]
    pub fn is_cmp0_in0(&self) -> bool {
        **self == CMP0_VP_A::CMP0_IN0
    }
    #[doc = "Checks if the value of the field is `CMP0_IN1`"]
    #[inline(always)]
    pub fn is_cmp0_in1(&self) -> bool {
        **self == CMP0_VP_A::CMP0_IN1
    }
    #[doc = "Checks if the value of the field is `CMP0_IN2`"]
    #[inline(always)]
    pub fn is_cmp0_in2(&self) -> bool {
        **self == CMP0_VP_A::CMP0_IN2
    }
    #[doc = "Checks if the value of the field is `CMP0_IN3`"]
    #[inline(always)]
    pub fn is_cmp0_in3(&self) -> bool {
        **self == CMP0_VP_A::CMP0_IN3
    }
    #[doc = "Checks if the value of the field is `CMP1_IN0`"]
    #[inline(always)]
    pub fn is_cmp1_in0(&self) -> bool {
        **self == CMP0_VP_A::CMP1_IN0
    }
    #[doc = "Checks if the value of the field is `INTERNAL_0_9_V_BAND_`"]
    #[inline(always)]
    pub fn is_internal_0_9_v_band_(&self) -> bool {
        **self == CMP0_VP_A::INTERNAL_0_9_V_BAND_
    }
    #[doc = "Checks if the value of the field is `TEMPERATURE_SENSOR_`"]
    #[inline(always)]
    pub fn is_temperature_sensor_(&self) -> bool {
        **self == CMP0_VP_A::TEMPERATURE_SENSOR_
    }
}
impl core::ops::Deref for CMP0_VP_R {
    type Target = crate::FieldReader<u8, CMP0_VP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_VP` writer - Comparator 0 VP input select."]
pub struct CMP0_VP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_VP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_VP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Vref divider 0."]
    #[inline(always)]
    pub fn vref_divider_0_(self) -> &'a mut W {
        self.variant(CMP0_VP_A::VREF_DIVIDER_0_)
    }
    #[doc = "CMP0_IN\\[0\\]."]
    #[inline(always)]
    pub fn cmp0_in0(self) -> &'a mut W {
        self.variant(CMP0_VP_A::CMP0_IN0)
    }
    #[doc = "CMP0_IN\\[1\\]."]
    #[inline(always)]
    pub fn cmp0_in1(self) -> &'a mut W {
        self.variant(CMP0_VP_A::CMP0_IN1)
    }
    #[doc = "CMP0_IN\\[2\\]."]
    #[inline(always)]
    pub fn cmp0_in2(self) -> &'a mut W {
        self.variant(CMP0_VP_A::CMP0_IN2)
    }
    #[doc = "CMP0_IN\\[3\\]."]
    #[inline(always)]
    pub fn cmp0_in3(self) -> &'a mut W {
        self.variant(CMP0_VP_A::CMP0_IN3)
    }
    #[doc = "CMP1_IN\\[0\\]."]
    #[inline(always)]
    pub fn cmp1_in0(self) -> &'a mut W {
        self.variant(CMP0_VP_A::CMP1_IN0)
    }
    #[doc = "internal 0.9 V band gap reference."]
    #[inline(always)]
    pub fn internal_0_9_v_band_(self) -> &'a mut W {
        self.variant(CMP0_VP_A::INTERNAL_0_9_V_BAND_)
    }
    #[doc = "temperature sensor."]
    #[inline(always)]
    pub fn temperature_sensor_(self) -> &'a mut W {
        self.variant(CMP0_VP_A::TEMPERATURE_SENSOR_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Comparator 0 output synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0_SYNC_A {
    #[doc = "0: The comparator 0 output is used directly."]
    DIRECT = 0,
    #[doc = "1: The comparator 0 output is synchronized with the internal bus clock for output to other peripherals."]
    SYNCH = 1,
}
impl From<CMP0_SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0_SYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0_SYNC` reader - Comparator 0 output synchronization control."]
pub struct CMP0_SYNC_R(crate::FieldReader<bool, CMP0_SYNC_A>);
impl CMP0_SYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_SYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_SYNC_A {
        match self.bits {
            false => CMP0_SYNC_A::DIRECT,
            true => CMP0_SYNC_A::SYNCH,
        }
    }
    #[doc = "Checks if the value of the field is `DIRECT`"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        **self == CMP0_SYNC_A::DIRECT
    }
    #[doc = "Checks if the value of the field is `SYNCH`"]
    #[inline(always)]
    pub fn is_synch(&self) -> bool {
        **self == CMP0_SYNC_A::SYNCH
    }
}
impl core::ops::Deref for CMP0_SYNC_R {
    type Target = crate::FieldReader<bool, CMP0_SYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_SYNC` writer - Comparator 0 output synchronization control."]
pub struct CMP0_SYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_SYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_SYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The comparator 0 output is used directly."]
    #[inline(always)]
    pub fn direct(self) -> &'a mut W {
        self.variant(CMP0_SYNC_A::DIRECT)
    }
    #[doc = "The comparator 0 output is synchronized with the internal bus clock for output to other peripherals."]
    #[inline(always)]
    pub fn synch(self) -> &'a mut W {
        self.variant(CMP0_SYNC_A::SYNCH)
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
#[doc = "Comparator 0 hysteresis control. When enabled, hysteresis determines the difference required between the comparator inputs before the comparator output switches. The difference must be in the direction opposite of the current comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0_HYS_A {
    #[doc = "0: Hysteresis is turned off, comparator output will change as the input voltages cross."]
    HYSTERESISOFF = 0,
    #[doc = "1: Hysteresis = 5 mV."]
    HYSTERESIS_EQ_5_MV_ = 1,
    #[doc = "2: Hysteresis = 10 mV."]
    HYSTERESIS_EQ_10_MV_ = 2,
    #[doc = "3: Hysteresis = 15 mV."]
    HYSTERESIS_EQ_15_MV_ = 3,
}
impl From<CMP0_HYS_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0_HYS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0_HYS` reader - Comparator 0 hysteresis control. When enabled, hysteresis determines the difference required between the comparator inputs before the comparator output switches. The difference must be in the direction opposite of the current comparator output."]
pub struct CMP0_HYS_R(crate::FieldReader<u8, CMP0_HYS_A>);
impl CMP0_HYS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP0_HYS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_HYS_A {
        match self.bits {
            0 => CMP0_HYS_A::HYSTERESISOFF,
            1 => CMP0_HYS_A::HYSTERESIS_EQ_5_MV_,
            2 => CMP0_HYS_A::HYSTERESIS_EQ_10_MV_,
            3 => CMP0_HYS_A::HYSTERESIS_EQ_15_MV_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYSTERESISOFF`"]
    #[inline(always)]
    pub fn is_hysteresisoff(&self) -> bool {
        **self == CMP0_HYS_A::HYSTERESISOFF
    }
    #[doc = "Checks if the value of the field is `HYSTERESIS_EQ_5_MV_`"]
    #[inline(always)]
    pub fn is_hysteresis_eq_5_mv_(&self) -> bool {
        **self == CMP0_HYS_A::HYSTERESIS_EQ_5_MV_
    }
    #[doc = "Checks if the value of the field is `HYSTERESIS_EQ_10_MV_`"]
    #[inline(always)]
    pub fn is_hysteresis_eq_10_mv_(&self) -> bool {
        **self == CMP0_HYS_A::HYSTERESIS_EQ_10_MV_
    }
    #[doc = "Checks if the value of the field is `HYSTERESIS_EQ_15_MV_`"]
    #[inline(always)]
    pub fn is_hysteresis_eq_15_mv_(&self) -> bool {
        **self == CMP0_HYS_A::HYSTERESIS_EQ_15_MV_
    }
}
impl core::ops::Deref for CMP0_HYS_R {
    type Target = crate::FieldReader<u8, CMP0_HYS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_HYS` writer - Comparator 0 hysteresis control. When enabled, hysteresis determines the difference required between the comparator inputs before the comparator output switches. The difference must be in the direction opposite of the current comparator output."]
pub struct CMP0_HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_HYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_HYS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Hysteresis is turned off, comparator output will change as the input voltages cross."]
    #[inline(always)]
    pub fn hysteresisoff(self) -> &'a mut W {
        self.variant(CMP0_HYS_A::HYSTERESISOFF)
    }
    #[doc = "Hysteresis = 5 mV."]
    #[inline(always)]
    pub fn hysteresis_eq_5_mv_(self) -> &'a mut W {
        self.variant(CMP0_HYS_A::HYSTERESIS_EQ_5_MV_)
    }
    #[doc = "Hysteresis = 10 mV."]
    #[inline(always)]
    pub fn hysteresis_eq_10_mv_(self) -> &'a mut W {
        self.variant(CMP0_HYS_A::HYSTERESIS_EQ_10_MV_)
    }
    #[doc = "Hysteresis = 15 mV."]
    #[inline(always)]
    pub fn hysteresis_eq_15_mv_(self) -> &'a mut W {
        self.variant(CMP0_HYS_A::HYSTERESIS_EQ_15_MV_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Selects the polarity of the CMP0 output for purposes of generating level interrupts. See Table 412.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0_INTPOL_A {
    #[doc = "0: The CMP0 output is used as-is for generating interrupts."]
    NOTINVERTED = 0,
    #[doc = "1: The CMP0 output is used inverted for generating interrupts."]
    INVERTED = 1,
}
impl From<CMP0_INTPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0_INTPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0_INTPOL` reader - Selects the polarity of the CMP0 output for purposes of generating level interrupts. See Table 412."]
pub struct CMP0_INTPOL_R(crate::FieldReader<bool, CMP0_INTPOL_A>);
impl CMP0_INTPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_INTPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_INTPOL_A {
        match self.bits {
            false => CMP0_INTPOL_A::NOTINVERTED,
            true => CMP0_INTPOL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTINVERTED`"]
    #[inline(always)]
    pub fn is_notinverted(&self) -> bool {
        **self == CMP0_INTPOL_A::NOTINVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == CMP0_INTPOL_A::INVERTED
    }
}
impl core::ops::Deref for CMP0_INTPOL_R {
    type Target = crate::FieldReader<bool, CMP0_INTPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_INTPOL` writer - Selects the polarity of the CMP0 output for purposes of generating level interrupts. See Table 412."]
pub struct CMP0_INTPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_INTPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_INTPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The CMP0 output is used as-is for generating interrupts."]
    #[inline(always)]
    pub fn notinverted(self) -> &'a mut W {
        self.variant(CMP0_INTPOL_A::NOTINVERTED)
    }
    #[doc = "The CMP0 output is used inverted for generating interrupts."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(CMP0_INTPOL_A::INVERTED)
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
#[doc = "Select comparator 0 interrupt type. See Table 412.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0_INTTYPE_A {
    #[doc = "0: Comparator 0 interrupt is edge triggered."]
    EDGE = 0,
    #[doc = "1: Comparator 0 interrupt is level triggered."]
    LEVEL = 1,
}
impl From<CMP0_INTTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0_INTTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0_INTTYPE` reader - Select comparator 0 interrupt type. See Table 412."]
pub struct CMP0_INTTYPE_R(crate::FieldReader<bool, CMP0_INTTYPE_A>);
impl CMP0_INTTYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_INTTYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_INTTYPE_A {
        match self.bits {
            false => CMP0_INTTYPE_A::EDGE,
            true => CMP0_INTTYPE_A::LEVEL,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE`"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        **self == CMP0_INTTYPE_A::EDGE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        **self == CMP0_INTTYPE_A::LEVEL
    }
}
impl core::ops::Deref for CMP0_INTTYPE_R {
    type Target = crate::FieldReader<bool, CMP0_INTTYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_INTTYPE` writer - Select comparator 0 interrupt type. See Table 412."]
pub struct CMP0_INTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_INTTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_INTTYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Comparator 0 interrupt is edge triggered."]
    #[inline(always)]
    pub fn edge(self) -> &'a mut W {
        self.variant(CMP0_INTTYPE_A::EDGE)
    }
    #[doc = "Comparator 0 interrupt is level triggered."]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(CMP0_INTTYPE_A::LEVEL)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Select edge triggered interrupt to be active on either high or low transitions, when CMP0_IntType = 0. See Table 412.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0_INTEDGE_A {
    #[doc = "0: Comparator 0 interrupt is active on falling edges."]
    FALLING = 0,
    #[doc = "1: Comparator 0 interrupt is active on rising edges."]
    RISING = 1,
    #[doc = "2: Comparator 0 Interrupt is active on both edges."]
    DUALEDGE = 2,
    #[doc = "3: reserved."]
    RESERVED_ = 3,
}
impl From<CMP0_INTEDGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0_INTEDGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0_INTEDGE` reader - Select edge triggered interrupt to be active on either high or low transitions, when CMP0_IntType = 0. See Table 412."]
pub struct CMP0_INTEDGE_R(crate::FieldReader<u8, CMP0_INTEDGE_A>);
impl CMP0_INTEDGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP0_INTEDGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_INTEDGE_A {
        match self.bits {
            0 => CMP0_INTEDGE_A::FALLING,
            1 => CMP0_INTEDGE_A::RISING,
            2 => CMP0_INTEDGE_A::DUALEDGE,
            3 => CMP0_INTEDGE_A::RESERVED_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == CMP0_INTEDGE_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == CMP0_INTEDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `DUALEDGE`"]
    #[inline(always)]
    pub fn is_dualedge(&self) -> bool {
        **self == CMP0_INTEDGE_A::DUALEDGE
    }
    #[doc = "Checks if the value of the field is `RESERVED_`"]
    #[inline(always)]
    pub fn is_reserved_(&self) -> bool {
        **self == CMP0_INTEDGE_A::RESERVED_
    }
}
impl core::ops::Deref for CMP0_INTEDGE_R {
    type Target = crate::FieldReader<u8, CMP0_INTEDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_INTEDGE` writer - Select edge triggered interrupt to be active on either high or low transitions, when CMP0_IntType = 0. See Table 412."]
pub struct CMP0_INTEDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_INTEDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_INTEDGE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Comparator 0 interrupt is active on falling edges."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(CMP0_INTEDGE_A::FALLING)
    }
    #[doc = "Comparator 0 interrupt is active on rising edges."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(CMP0_INTEDGE_A::RISING)
    }
    #[doc = "Comparator 0 Interrupt is active on both edges."]
    #[inline(always)]
    pub fn dualedge(self) -> &'a mut W {
        self.variant(CMP0_INTEDGE_A::DUALEDGE)
    }
    #[doc = "reserved."]
    #[inline(always)]
    pub fn reserved_(self) -> &'a mut W {
        self.variant(CMP0_INTEDGE_A::RESERVED_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Comparator 0 interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0_INTFLAG_A {
    #[doc = "0: The Comparator 0 interrupt is not pending."]
    NOTPENDING = 0,
    #[doc = "1: The Comparator 0 interrupt is pending. Writing a 1 to this bit clears the flag."]
    PENDING = 1,
}
impl From<CMP0_INTFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0_INTFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0_INTFLAG` reader - Comparator 0 interrupt flag."]
pub struct CMP0_INTFLAG_R(crate::FieldReader<bool, CMP0_INTFLAG_A>);
impl CMP0_INTFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_INTFLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_INTFLAG_A {
        match self.bits {
            false => CMP0_INTFLAG_A::NOTPENDING,
            true => CMP0_INTFLAG_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_notpending(&self) -> bool {
        **self == CMP0_INTFLAG_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == CMP0_INTFLAG_A::PENDING
    }
}
impl core::ops::Deref for CMP0_INTFLAG_R {
    type Target = crate::FieldReader<bool, CMP0_INTFLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_INTFLAG` writer - Comparator 0 interrupt flag."]
pub struct CMP0_INTFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_INTFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_INTFLAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The Comparator 0 interrupt is not pending."]
    #[inline(always)]
    pub fn notpending(self) -> &'a mut W {
        self.variant(CMP0_INTFLAG_A::NOTPENDING)
    }
    #[doc = "The Comparator 0 interrupt is pending. Writing a 1 to this bit clears the flag."]
    #[inline(always)]
    pub fn pending(self) -> &'a mut W {
        self.variant(CMP0_INTFLAG_A::PENDING)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Voltage ladder enable for comparator 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMP0_VLADEN_A {
    #[doc = "0: The Comparator 0 voltage ladder is disabled."]
    DISABLED = 0,
    #[doc = "1: The Comparator 0 voltage ladder is disabled in Deep Sleep and Power-down modes and re-enabled automatically when exiting those modes."]
    DIS_DEEPSLP_PWRDWN = 1,
    #[doc = "2: The Comparator 0 voltage ladder is disabled in Power-down mode and re-enabled automatically when exiting Power-down."]
    DIS_PWRDWN = 2,
    #[doc = "3: The Comparator 0 voltage ladder is enabled."]
    ENABLED = 3,
}
impl From<CMP0_VLADEN_A> for u8 {
    #[inline(always)]
    fn from(variant: CMP0_VLADEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMP0_VLADEN` reader - Voltage ladder enable for comparator 0."]
pub struct CMP0_VLADEN_R(crate::FieldReader<u8, CMP0_VLADEN_A>);
impl CMP0_VLADEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP0_VLADEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_VLADEN_A {
        match self.bits {
            0 => CMP0_VLADEN_A::DISABLED,
            1 => CMP0_VLADEN_A::DIS_DEEPSLP_PWRDWN,
            2 => CMP0_VLADEN_A::DIS_PWRDWN,
            3 => CMP0_VLADEN_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMP0_VLADEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `DIS_DEEPSLP_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_deepslp_pwrdwn(&self) -> bool {
        **self == CMP0_VLADEN_A::DIS_DEEPSLP_PWRDWN
    }
    #[doc = "Checks if the value of the field is `DIS_PWRDWN`"]
    #[inline(always)]
    pub fn is_dis_pwrdwn(&self) -> bool {
        **self == CMP0_VLADEN_A::DIS_PWRDWN
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMP0_VLADEN_A::ENABLED
    }
}
impl core::ops::Deref for CMP0_VLADEN_R {
    type Target = crate::FieldReader<u8, CMP0_VLADEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_VLADEN` writer - Voltage ladder enable for comparator 0."]
pub struct CMP0_VLADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_VLADEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_VLADEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The Comparator 0 voltage ladder is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMP0_VLADEN_A::DISABLED)
    }
    #[doc = "The Comparator 0 voltage ladder is disabled in Deep Sleep and Power-down modes and re-enabled automatically when exiting those modes."]
    #[inline(always)]
    pub fn dis_deepslp_pwrdwn(self) -> &'a mut W {
        self.variant(CMP0_VLADEN_A::DIS_DEEPSLP_PWRDWN)
    }
    #[doc = "The Comparator 0 voltage ladder is disabled in Power-down mode and re-enabled automatically when exiting Power-down."]
    #[inline(always)]
    pub fn dis_pwrdwn(self) -> &'a mut W {
        self.variant(CMP0_VLADEN_A::DIS_PWRDWN)
    }
    #[doc = "The Comparator 0 voltage ladder is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMP0_VLADEN_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Voltage reference select for comparator 0 voltage ladder.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP0_VLADREF_A {
    #[doc = "0: VREF_CMP pin."]
    VREF_CMP_PIN_ = 0,
    #[doc = "1: VDDA pin."]
    VDDA_PIN_ = 1,
}
impl From<CMP0_VLADREF_A> for bool {
    #[inline(always)]
    fn from(variant: CMP0_VLADREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0_VLADREF` reader - Voltage reference select for comparator 0 voltage ladder."]
pub struct CMP0_VLADREF_R(crate::FieldReader<bool, CMP0_VLADREF_A>);
impl CMP0_VLADREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP0_VLADREF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP0_VLADREF_A {
        match self.bits {
            false => CMP0_VLADREF_A::VREF_CMP_PIN_,
            true => CMP0_VLADREF_A::VDDA_PIN_,
        }
    }
    #[doc = "Checks if the value of the field is `VREF_CMP_PIN_`"]
    #[inline(always)]
    pub fn is_vref_cmp_pin_(&self) -> bool {
        **self == CMP0_VLADREF_A::VREF_CMP_PIN_
    }
    #[doc = "Checks if the value of the field is `VDDA_PIN_`"]
    #[inline(always)]
    pub fn is_vdda_pin_(&self) -> bool {
        **self == CMP0_VLADREF_A::VDDA_PIN_
    }
}
impl core::ops::Deref for CMP0_VLADREF_R {
    type Target = crate::FieldReader<bool, CMP0_VLADREF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_VLADREF` writer - Voltage reference select for comparator 0 voltage ladder."]
pub struct CMP0_VLADREF_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_VLADREF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP0_VLADREF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VREF_CMP pin."]
    #[inline(always)]
    pub fn vref_cmp_pin_(self) -> &'a mut W {
        self.variant(CMP0_VLADREF_A::VREF_CMP_PIN_)
    }
    #[doc = "VDDA pin."]
    #[inline(always)]
    pub fn vdda_pin_(self) -> &'a mut W {
        self.variant(CMP0_VLADREF_A::VDDA_PIN_)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CMP0_VSEL` reader - Voltage ladder value for comparator 0. The reference voltage Vref depends on the setting of CMP0_VLADREF (either VDD(3V3) or voltage on pin VREF_CMP). 00000 = Vss. 00001 = 1 x Vref0 / 31. 00010 = 2 x Vref0 / 31. ... 11111 = Vref0"]
pub struct CMP0_VSEL_R(crate::FieldReader<u8, u8>);
impl CMP0_VSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMP0_VSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP0_VSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP0_VSEL` writer - Voltage ladder value for comparator 0. The reference voltage Vref depends on the setting of CMP0_VLADREF (either VDD(3V3) or voltage on pin VREF_CMP). 00000 = Vss. 00001 = 1 x Vref0 / 31. 00010 = 2 x Vref0 / 31. ... 11111 = Vref0"]
pub struct CMP0_VSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP0_VSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator 0 enable control."]
    #[inline(always)]
    pub fn cmp0_en(&self) -> CMP0_EN_R {
        CMP0_EN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Comparator 0 output enable."]
    #[inline(always)]
    pub fn cmp0_oe(&self) -> CMP0_OE_R {
        CMP0_OE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Comparator 0 status. This bit reflects the comparator 0 output, and is not affected by CMP0_OE."]
    #[inline(always)]
    pub fn cmp0_stat(&self) -> CMP0_STAT_R {
        CMP0_STAT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Comparator 0 VM input select."]
    #[inline(always)]
    pub fn cmp0_vm(&self) -> CMP0_VM_R {
        CMP0_VM_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Comparator 0 VP input select."]
    #[inline(always)]
    pub fn cmp0_vp(&self) -> CMP0_VP_R {
        CMP0_VP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Comparator 0 output synchronization control."]
    #[inline(always)]
    pub fn cmp0_sync(&self) -> CMP0_SYNC_R {
        CMP0_SYNC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Comparator 0 hysteresis control. When enabled, hysteresis determines the difference required between the comparator inputs before the comparator output switches. The difference must be in the direction opposite of the current comparator output."]
    #[inline(always)]
    pub fn cmp0_hys(&self) -> CMP0_HYS_R {
        CMP0_HYS_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Selects the polarity of the CMP0 output for purposes of generating level interrupts. See Table 412."]
    #[inline(always)]
    pub fn cmp0_intpol(&self) -> CMP0_INTPOL_R {
        CMP0_INTPOL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Select comparator 0 interrupt type. See Table 412."]
    #[inline(always)]
    pub fn cmp0_inttype(&self) -> CMP0_INTTYPE_R {
        CMP0_INTTYPE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - Select edge triggered interrupt to be active on either high or low transitions, when CMP0_IntType = 0. See Table 412."]
    #[inline(always)]
    pub fn cmp0_intedge(&self) -> CMP0_INTEDGE_R {
        CMP0_INTEDGE_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Comparator 0 interrupt flag."]
    #[inline(always)]
    pub fn cmp0_intflag(&self) -> CMP0_INTFLAG_R {
        CMP0_INTFLAG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Voltage ladder enable for comparator 0."]
    #[inline(always)]
    pub fn cmp0_vladen(&self) -> CMP0_VLADEN_R {
        CMP0_VLADEN_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Voltage reference select for comparator 0 voltage ladder."]
    #[inline(always)]
    pub fn cmp0_vladref(&self) -> CMP0_VLADREF_R {
        CMP0_VLADREF_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Voltage ladder value for comparator 0. The reference voltage Vref depends on the setting of CMP0_VLADREF (either VDD(3V3) or voltage on pin VREF_CMP). 00000 = Vss. 00001 = 1 x Vref0 / 31. 00010 = 2 x Vref0 / 31. ... 11111 = Vref0"]
    #[inline(always)]
    pub fn cmp0_vsel(&self) -> CMP0_VSEL_R {
        CMP0_VSEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator 0 enable control."]
    #[inline(always)]
    pub fn cmp0_en(&mut self) -> CMP0_EN_W {
        CMP0_EN_W { w: self }
    }
    #[doc = "Bit 2 - Comparator 0 output enable."]
    #[inline(always)]
    pub fn cmp0_oe(&mut self) -> CMP0_OE_W {
        CMP0_OE_W { w: self }
    }
    #[doc = "Bit 3 - Comparator 0 status. This bit reflects the comparator 0 output, and is not affected by CMP0_OE."]
    #[inline(always)]
    pub fn cmp0_stat(&mut self) -> CMP0_STAT_W {
        CMP0_STAT_W { w: self }
    }
    #[doc = "Bits 4:6 - Comparator 0 VM input select."]
    #[inline(always)]
    pub fn cmp0_vm(&mut self) -> CMP0_VM_W {
        CMP0_VM_W { w: self }
    }
    #[doc = "Bits 8:10 - Comparator 0 VP input select."]
    #[inline(always)]
    pub fn cmp0_vp(&mut self) -> CMP0_VP_W {
        CMP0_VP_W { w: self }
    }
    #[doc = "Bit 12 - Comparator 0 output synchronization control."]
    #[inline(always)]
    pub fn cmp0_sync(&mut self) -> CMP0_SYNC_W {
        CMP0_SYNC_W { w: self }
    }
    #[doc = "Bits 13:14 - Comparator 0 hysteresis control. When enabled, hysteresis determines the difference required between the comparator inputs before the comparator output switches. The difference must be in the direction opposite of the current comparator output."]
    #[inline(always)]
    pub fn cmp0_hys(&mut self) -> CMP0_HYS_W {
        CMP0_HYS_W { w: self }
    }
    #[doc = "Bit 15 - Selects the polarity of the CMP0 output for purposes of generating level interrupts. See Table 412."]
    #[inline(always)]
    pub fn cmp0_intpol(&mut self) -> CMP0_INTPOL_W {
        CMP0_INTPOL_W { w: self }
    }
    #[doc = "Bit 16 - Select comparator 0 interrupt type. See Table 412."]
    #[inline(always)]
    pub fn cmp0_inttype(&mut self) -> CMP0_INTTYPE_W {
        CMP0_INTTYPE_W { w: self }
    }
    #[doc = "Bits 17:18 - Select edge triggered interrupt to be active on either high or low transitions, when CMP0_IntType = 0. See Table 412."]
    #[inline(always)]
    pub fn cmp0_intedge(&mut self) -> CMP0_INTEDGE_W {
        CMP0_INTEDGE_W { w: self }
    }
    #[doc = "Bit 19 - Comparator 0 interrupt flag."]
    #[inline(always)]
    pub fn cmp0_intflag(&mut self) -> CMP0_INTFLAG_W {
        CMP0_INTFLAG_W { w: self }
    }
    #[doc = "Bits 20:21 - Voltage ladder enable for comparator 0."]
    #[inline(always)]
    pub fn cmp0_vladen(&mut self) -> CMP0_VLADEN_W {
        CMP0_VLADEN_W { w: self }
    }
    #[doc = "Bit 22 - Voltage reference select for comparator 0 voltage ladder."]
    #[inline(always)]
    pub fn cmp0_vladref(&mut self) -> CMP0_VLADREF_W {
        CMP0_VLADREF_W { w: self }
    }
    #[doc = "Bits 24:28 - Voltage ladder value for comparator 0. The reference voltage Vref depends on the setting of CMP0_VLADREF (either VDD(3V3) or voltage on pin VREF_CMP). 00000 = Vss. 00001 = 1 x Vref0 / 31. 00010 = 2 x Vref0 / 31. ... 11111 = Vref0"]
    #[inline(always)]
    pub fn cmp0_vsel(&mut self) -> CMP0_VSEL_W {
        CMP0_VSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
