#[doc = "Register `CFG_OUT` reader"]
pub struct R(crate::R<CFG_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_OUT` writer"]
pub struct W(crate::W<CFG_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_OUT_SPEC>;
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
impl From<crate::W<CFG_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOW0` reader - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
pub struct SLOW0_R(crate::FieldReader<bool, bool>);
impl SLOW0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOW0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW0` writer - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
pub struct SLOW0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW0_W<'a> {
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
#[doc = "Field `SLOW1` reader - Enables slow slew rate for IO pin 1"]
pub struct SLOW1_R(crate::FieldReader<bool, bool>);
impl SLOW1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOW1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW1` writer - Enables slow slew rate for IO pin 1"]
pub struct SLOW1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW1_W<'a> {
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
#[doc = "Field `SLOW2` reader - Enables slow slew rate for IO pin 2"]
pub struct SLOW2_R(crate::FieldReader<bool, bool>);
impl SLOW2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOW2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW2` writer - Enables slow slew rate for IO pin 2"]
pub struct SLOW2_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW2_W<'a> {
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
#[doc = "Field `SLOW3` reader - Enables slow slew rate for IO pin 3"]
pub struct SLOW3_R(crate::FieldReader<bool, bool>);
impl SLOW3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOW3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW3` writer - Enables slow slew rate for IO pin 3"]
pub struct SLOW3_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW3_W<'a> {
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
#[doc = "Field `SLOW4` reader - Enables slow slew rate for IO pin 4"]
pub struct SLOW4_R(crate::FieldReader<bool, bool>);
impl SLOW4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOW4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW4` writer - Enables slow slew rate for IO pin 4"]
pub struct SLOW4_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW4_W<'a> {
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
#[doc = "Field `SLOW5` reader - Enables slow slew rate for IO pin 5"]
pub struct SLOW5_R(crate::FieldReader<bool, bool>);
impl SLOW5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOW5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW5` writer - Enables slow slew rate for IO pin 5"]
pub struct SLOW5_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW5_W<'a> {
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
#[doc = "Field `SLOW6` reader - Enables slow slew rate for IO pin 6"]
pub struct SLOW6_R(crate::FieldReader<bool, bool>);
impl SLOW6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOW6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW6` writer - Enables slow slew rate for IO pin 6"]
pub struct SLOW6_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW6_W<'a> {
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
#[doc = "Field `SLOW7` reader - Enables slow slew rate for IO pin 7"]
pub struct SLOW7_R(crate::FieldReader<bool, bool>);
impl SLOW7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLOW7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLOW7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLOW7` writer - Enables slow slew rate for IO pin 7"]
pub struct SLOW7_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOW7_W<'a> {
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
#[doc = "Sets the GPIO drive strength for IO pin 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_SEL0_A {
    #[doc = "0: Full drive strength: GPIO drives current at its max rated spec."]
    FULL_DRIVE = 0,
    #[doc = "1: 1/2 drive strength: GPIO drives current at 1/2 of its max rated spec"]
    ONE_HALF_DRIVE = 1,
    #[doc = "2: 1/4 drive strength: GPIO drives current at 1/4 of its max rated spec."]
    ONE_QUARTER_DRIVE = 2,
    #[doc = "3: 1/8 drive strength: GPIO drives current at 1/8 of its max rated spec."]
    ONE_EIGHTH_DRIVE = 3,
}
impl From<DRIVE_SEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_SEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DRIVE_SEL0` reader - Sets the GPIO drive strength for IO pin 0"]
pub struct DRIVE_SEL0_R(crate::FieldReader<u8, DRIVE_SEL0_A>);
impl DRIVE_SEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_SEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_SEL0_A {
        match self.bits {
            0 => DRIVE_SEL0_A::FULL_DRIVE,
            1 => DRIVE_SEL0_A::ONE_HALF_DRIVE,
            2 => DRIVE_SEL0_A::ONE_QUARTER_DRIVE,
            3 => DRIVE_SEL0_A::ONE_EIGHTH_DRIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL_DRIVE`"]
    #[inline(always)]
    pub fn is_full_drive(&self) -> bool {
        **self == DRIVE_SEL0_A::FULL_DRIVE
    }
    #[doc = "Checks if the value of the field is `ONE_HALF_DRIVE`"]
    #[inline(always)]
    pub fn is_one_half_drive(&self) -> bool {
        **self == DRIVE_SEL0_A::ONE_HALF_DRIVE
    }
    #[doc = "Checks if the value of the field is `ONE_QUARTER_DRIVE`"]
    #[inline(always)]
    pub fn is_one_quarter_drive(&self) -> bool {
        **self == DRIVE_SEL0_A::ONE_QUARTER_DRIVE
    }
    #[doc = "Checks if the value of the field is `ONE_EIGHTH_DRIVE`"]
    #[inline(always)]
    pub fn is_one_eighth_drive(&self) -> bool {
        **self == DRIVE_SEL0_A::ONE_EIGHTH_DRIVE
    }
}
impl core::ops::Deref for DRIVE_SEL0_R {
    type Target = crate::FieldReader<u8, DRIVE_SEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE_SEL0` writer - Sets the GPIO drive strength for IO pin 0"]
pub struct DRIVE_SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVE_SEL0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Full drive strength: GPIO drives current at its max rated spec."]
    #[inline(always)]
    pub fn full_drive(self) -> &'a mut W {
        self.variant(DRIVE_SEL0_A::FULL_DRIVE)
    }
    #[doc = "1/2 drive strength: GPIO drives current at 1/2 of its max rated spec"]
    #[inline(always)]
    pub fn one_half_drive(self) -> &'a mut W {
        self.variant(DRIVE_SEL0_A::ONE_HALF_DRIVE)
    }
    #[doc = "1/4 drive strength: GPIO drives current at 1/4 of its max rated spec."]
    #[inline(always)]
    pub fn one_quarter_drive(self) -> &'a mut W {
        self.variant(DRIVE_SEL0_A::ONE_QUARTER_DRIVE)
    }
    #[doc = "1/8 drive strength: GPIO drives current at 1/8 of its max rated spec."]
    #[inline(always)]
    pub fn one_eighth_drive(self) -> &'a mut W {
        self.variant(DRIVE_SEL0_A::ONE_EIGHTH_DRIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DRIVE_SEL1` reader - Sets the GPIO drive strength for IO pin 1"]
pub struct DRIVE_SEL1_R(crate::FieldReader<u8, u8>);
impl DRIVE_SEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_SEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE_SEL1` writer - Sets the GPIO drive strength for IO pin 1"]
pub struct DRIVE_SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `DRIVE_SEL2` reader - Sets the GPIO drive strength for IO pin 2"]
pub struct DRIVE_SEL2_R(crate::FieldReader<u8, u8>);
impl DRIVE_SEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_SEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_SEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE_SEL2` writer - Sets the GPIO drive strength for IO pin 2"]
pub struct DRIVE_SEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `DRIVE_SEL3` reader - Sets the GPIO drive strength for IO pin 3"]
pub struct DRIVE_SEL3_R(crate::FieldReader<u8, u8>);
impl DRIVE_SEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_SEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_SEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE_SEL3` writer - Sets the GPIO drive strength for IO pin 3"]
pub struct DRIVE_SEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `DRIVE_SEL4` reader - Sets the GPIO drive strength for IO pin 4"]
pub struct DRIVE_SEL4_R(crate::FieldReader<u8, u8>);
impl DRIVE_SEL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_SEL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_SEL4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE_SEL4` writer - Sets the GPIO drive strength for IO pin 4"]
pub struct DRIVE_SEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `DRIVE_SEL5` reader - Sets the GPIO drive strength for IO pin 5"]
pub struct DRIVE_SEL5_R(crate::FieldReader<u8, u8>);
impl DRIVE_SEL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_SEL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_SEL5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE_SEL5` writer - Sets the GPIO drive strength for IO pin 5"]
pub struct DRIVE_SEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `DRIVE_SEL6` reader - Sets the GPIO drive strength for IO pin 6"]
pub struct DRIVE_SEL6_R(crate::FieldReader<u8, u8>);
impl DRIVE_SEL6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_SEL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_SEL6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE_SEL6` writer - Sets the GPIO drive strength for IO pin 6"]
pub struct DRIVE_SEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `DRIVE_SEL7` reader - Sets the GPIO drive strength for IO pin 7"]
pub struct DRIVE_SEL7_R(crate::FieldReader<u8, u8>);
impl DRIVE_SEL7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DRIVE_SEL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRIVE_SEL7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRIVE_SEL7` writer - Sets the GPIO drive strength for IO pin 7"]
pub struct DRIVE_SEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_SEL7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub fn slow0(&self) -> SLOW0_R {
        SLOW0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    pub fn slow1(&self) -> SLOW1_R {
        SLOW1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    pub fn slow2(&self) -> SLOW2_R {
        SLOW2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    pub fn slow3(&self) -> SLOW3_R {
        SLOW3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    pub fn slow4(&self) -> SLOW4_R {
        SLOW4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    pub fn slow5(&self) -> SLOW5_R {
        SLOW5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    pub fn slow6(&self) -> SLOW6_R {
        SLOW6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    pub fn slow7(&self) -> SLOW7_R {
        SLOW7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    pub fn drive_sel0(&self) -> DRIVE_SEL0_R {
        DRIVE_SEL0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub fn drive_sel1(&self) -> DRIVE_SEL1_R {
        DRIVE_SEL1_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub fn drive_sel2(&self) -> DRIVE_SEL2_R {
        DRIVE_SEL2_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub fn drive_sel3(&self) -> DRIVE_SEL3_R {
        DRIVE_SEL3_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub fn drive_sel4(&self) -> DRIVE_SEL4_R {
        DRIVE_SEL4_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub fn drive_sel5(&self) -> DRIVE_SEL5_R {
        DRIVE_SEL5_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub fn drive_sel6(&self) -> DRIVE_SEL6_R {
        DRIVE_SEL6_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub fn drive_sel7(&self) -> DRIVE_SEL7_R {
        DRIVE_SEL7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables slow slew rate for IO pin 0 '0': Fast slew rate '1': Slow slew rate"]
    #[inline(always)]
    pub fn slow0(&mut self) -> SLOW0_W {
        SLOW0_W { w: self }
    }
    #[doc = "Bit 1 - Enables slow slew rate for IO pin 1"]
    #[inline(always)]
    pub fn slow1(&mut self) -> SLOW1_W {
        SLOW1_W { w: self }
    }
    #[doc = "Bit 2 - Enables slow slew rate for IO pin 2"]
    #[inline(always)]
    pub fn slow2(&mut self) -> SLOW2_W {
        SLOW2_W { w: self }
    }
    #[doc = "Bit 3 - Enables slow slew rate for IO pin 3"]
    #[inline(always)]
    pub fn slow3(&mut self) -> SLOW3_W {
        SLOW3_W { w: self }
    }
    #[doc = "Bit 4 - Enables slow slew rate for IO pin 4"]
    #[inline(always)]
    pub fn slow4(&mut self) -> SLOW4_W {
        SLOW4_W { w: self }
    }
    #[doc = "Bit 5 - Enables slow slew rate for IO pin 5"]
    #[inline(always)]
    pub fn slow5(&mut self) -> SLOW5_W {
        SLOW5_W { w: self }
    }
    #[doc = "Bit 6 - Enables slow slew rate for IO pin 6"]
    #[inline(always)]
    pub fn slow6(&mut self) -> SLOW6_W {
        SLOW6_W { w: self }
    }
    #[doc = "Bit 7 - Enables slow slew rate for IO pin 7"]
    #[inline(always)]
    pub fn slow7(&mut self) -> SLOW7_W {
        SLOW7_W { w: self }
    }
    #[doc = "Bits 16:17 - Sets the GPIO drive strength for IO pin 0"]
    #[inline(always)]
    pub fn drive_sel0(&mut self) -> DRIVE_SEL0_W {
        DRIVE_SEL0_W { w: self }
    }
    #[doc = "Bits 18:19 - Sets the GPIO drive strength for IO pin 1"]
    #[inline(always)]
    pub fn drive_sel1(&mut self) -> DRIVE_SEL1_W {
        DRIVE_SEL1_W { w: self }
    }
    #[doc = "Bits 20:21 - Sets the GPIO drive strength for IO pin 2"]
    #[inline(always)]
    pub fn drive_sel2(&mut self) -> DRIVE_SEL2_W {
        DRIVE_SEL2_W { w: self }
    }
    #[doc = "Bits 22:23 - Sets the GPIO drive strength for IO pin 3"]
    #[inline(always)]
    pub fn drive_sel3(&mut self) -> DRIVE_SEL3_W {
        DRIVE_SEL3_W { w: self }
    }
    #[doc = "Bits 24:25 - Sets the GPIO drive strength for IO pin 4"]
    #[inline(always)]
    pub fn drive_sel4(&mut self) -> DRIVE_SEL4_W {
        DRIVE_SEL4_W { w: self }
    }
    #[doc = "Bits 26:27 - Sets the GPIO drive strength for IO pin 5"]
    #[inline(always)]
    pub fn drive_sel5(&mut self) -> DRIVE_SEL5_W {
        DRIVE_SEL5_W { w: self }
    }
    #[doc = "Bits 28:29 - Sets the GPIO drive strength for IO pin 6"]
    #[inline(always)]
    pub fn drive_sel6(&mut self) -> DRIVE_SEL6_W {
        DRIVE_SEL6_W { w: self }
    }
    #[doc = "Bits 30:31 - Sets the GPIO drive strength for IO pin 7"]
    #[inline(always)]
    pub fn drive_sel7(&mut self) -> DRIVE_SEL7_W {
        DRIVE_SEL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port output buffer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_out](index.html) module"]
pub struct CFG_OUT_SPEC;
impl crate::RegisterSpec for CFG_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_out::R](R) reader structure"]
impl crate::Readable for CFG_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_out::W](W) writer structure"]
impl crate::Writable for CFG_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG_OUT to value 0"]
impl crate::Resettable for CFG_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
