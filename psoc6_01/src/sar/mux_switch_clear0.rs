#[doc = "Register `MUX_SWITCH_CLEAR0` reader"]
pub struct R(crate::R<MUX_SWITCH_CLEAR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUX_SWITCH_CLEAR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUX_SWITCH_CLEAR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUX_SWITCH_CLEAR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUX_SWITCH_CLEAR0` writer"]
pub struct W(crate::W<MUX_SWITCH_CLEAR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUX_SWITCH_CLEAR0_SPEC>;
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
impl From<crate::W<MUX_SWITCH_CLEAR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUX_SWITCH_CLEAR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUX_FW_P0_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P0_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P0_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P0_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P0_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P0_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P0_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P0_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_P1_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P1_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P1_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P1_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P1_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P1_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P1_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P1_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_P2_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P2_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P2_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P2_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P2_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P2_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P2_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P2_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_P3_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P3_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P3_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P3_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P3_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P3_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P3_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P3_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_P4_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P4_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P4_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P4_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P4_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P4_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P4_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P4_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_P5_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P5_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P5_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P5_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P5_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P5_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P5_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P5_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_P6_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P6_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P6_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P6_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P6_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P6_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P6_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P6_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_P7_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P7_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P7_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P7_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P7_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P7_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P7_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P7_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_P0_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P0_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P0_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P0_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P0_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P0_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P0_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P0_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_P1_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P1_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P1_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P1_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P1_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P1_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P1_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P1_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_P2_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P2_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P2_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P2_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P2_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P2_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P2_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P2_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_P3_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P3_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P3_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P3_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P3_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P3_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P3_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P3_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_P4_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P4_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P4_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P4_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P4_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P4_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P4_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P4_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_P5_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P5_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P5_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P5_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P5_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P5_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P5_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P5_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_P6_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P6_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P6_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P6_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P6_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P6_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P6_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P6_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_P7_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P7_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P7_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P7_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P7_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P7_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P7_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P7_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_VSSA_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_VSSA_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_VSSA_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_VSSA_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_VSSA_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_VSSA_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_VSSA_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_VSSA_VMINUS_W<'a> {
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
#[doc = "Field `MUX_FW_TEMP_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_TEMP_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_TEMP_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_TEMP_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_TEMP_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_TEMP_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_TEMP_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_TEMP_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `MUX_FW_AMUXBUSA_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_AMUXBUSA_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_AMUXBUSA_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_AMUXBUSA_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_AMUXBUSA_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_AMUXBUSA_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_AMUXBUSA_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_AMUXBUSA_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `MUX_FW_AMUXBUSB_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_AMUXBUSB_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_AMUXBUSB_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_AMUXBUSB_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_AMUXBUSB_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_AMUXBUSB_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_AMUXBUSB_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_AMUXBUSB_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_AMUXBUSA_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_AMUXBUSA_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_AMUXBUSA_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_AMUXBUSA_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_AMUXBUSA_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_AMUXBUSA_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_AMUXBUSA_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_AMUXBUSA_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `MUX_FW_AMUXBUSB_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_AMUXBUSB_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_AMUXBUSB_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_AMUXBUSB_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_AMUXBUSB_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_AMUXBUSB_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_AMUXBUSB_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_AMUXBUSB_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `MUX_FW_SARBUS0_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_SARBUS0_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_SARBUS0_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_SARBUS0_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_SARBUS0_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_SARBUS0_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_SARBUS0_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_SARBUS0_VPLUS_W<'a> {
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
#[doc = "Field `MUX_FW_SARBUS1_VPLUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_SARBUS1_VPLUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_SARBUS1_VPLUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_SARBUS1_VPLUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_SARBUS1_VPLUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_SARBUS1_VPLUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_SARBUS1_VPLUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_SARBUS1_VPLUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `MUX_FW_SARBUS0_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_SARBUS0_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_SARBUS0_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_SARBUS0_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_SARBUS0_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_SARBUS0_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_SARBUS0_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_SARBUS0_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `MUX_FW_SARBUS1_VMINUS` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_SARBUS1_VMINUS_R(crate::FieldReader<bool, bool>);
impl MUX_FW_SARBUS1_VMINUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_SARBUS1_VMINUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_SARBUS1_VMINUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_SARBUS1_VMINUS` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_SARBUS1_VMINUS_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_SARBUS1_VMINUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `MUX_FW_P4_COREIO0` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P4_COREIO0_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P4_COREIO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P4_COREIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P4_COREIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P4_COREIO0` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P4_COREIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P4_COREIO0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `MUX_FW_P5_COREIO1` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P5_COREIO1_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P5_COREIO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P5_COREIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P5_COREIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P5_COREIO1` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P5_COREIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P5_COREIO1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `MUX_FW_P6_COREIO2` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P6_COREIO2_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P6_COREIO2_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P6_COREIO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P6_COREIO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P6_COREIO2` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P6_COREIO2_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P6_COREIO2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `MUX_FW_P7_COREIO3` reader - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P7_COREIO3_R(crate::FieldReader<bool, bool>);
impl MUX_FW_P7_COREIO3_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUX_FW_P7_COREIO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUX_FW_P7_COREIO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUX_FW_P7_COREIO3` writer - Write '1' to clear corresponding bit in MUX_SWITCH0"]
pub struct MUX_FW_P7_COREIO3_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_FW_P7_COREIO3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&self) -> MUX_FW_P0_VPLUS_R {
        MUX_FW_P0_VPLUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&self) -> MUX_FW_P1_VPLUS_R {
        MUX_FW_P1_VPLUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&self) -> MUX_FW_P2_VPLUS_R {
        MUX_FW_P2_VPLUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&self) -> MUX_FW_P3_VPLUS_R {
        MUX_FW_P3_VPLUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&self) -> MUX_FW_P4_VPLUS_R {
        MUX_FW_P4_VPLUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&self) -> MUX_FW_P5_VPLUS_R {
        MUX_FW_P5_VPLUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&self) -> MUX_FW_P6_VPLUS_R {
        MUX_FW_P6_VPLUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&self) -> MUX_FW_P7_VPLUS_R {
        MUX_FW_P7_VPLUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&self) -> MUX_FW_P0_VMINUS_R {
        MUX_FW_P0_VMINUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&self) -> MUX_FW_P1_VMINUS_R {
        MUX_FW_P1_VMINUS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&self) -> MUX_FW_P2_VMINUS_R {
        MUX_FW_P2_VMINUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&self) -> MUX_FW_P3_VMINUS_R {
        MUX_FW_P3_VMINUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&self) -> MUX_FW_P4_VMINUS_R {
        MUX_FW_P4_VMINUS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&self) -> MUX_FW_P5_VMINUS_R {
        MUX_FW_P5_VMINUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&self) -> MUX_FW_P6_VMINUS_R {
        MUX_FW_P6_VMINUS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&self) -> MUX_FW_P7_VMINUS_R {
        MUX_FW_P7_VMINUS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&self) -> MUX_FW_VSSA_VMINUS_R {
        MUX_FW_VSSA_VMINUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&self) -> MUX_FW_TEMP_VPLUS_R {
        MUX_FW_TEMP_VPLUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&self) -> MUX_FW_AMUXBUSA_VPLUS_R {
        MUX_FW_AMUXBUSA_VPLUS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&self) -> MUX_FW_AMUXBUSB_VPLUS_R {
        MUX_FW_AMUXBUSB_VPLUS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&self) -> MUX_FW_AMUXBUSA_VMINUS_R {
        MUX_FW_AMUXBUSA_VMINUS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&self) -> MUX_FW_AMUXBUSB_VMINUS_R {
        MUX_FW_AMUXBUSB_VMINUS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&self) -> MUX_FW_SARBUS0_VPLUS_R {
        MUX_FW_SARBUS0_VPLUS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&self) -> MUX_FW_SARBUS1_VPLUS_R {
        MUX_FW_SARBUS1_VPLUS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&self) -> MUX_FW_SARBUS0_VMINUS_R {
        MUX_FW_SARBUS0_VMINUS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&self) -> MUX_FW_SARBUS1_VMINUS_R {
        MUX_FW_SARBUS1_VMINUS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_coreio0(&self) -> MUX_FW_P4_COREIO0_R {
        MUX_FW_P4_COREIO0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_coreio1(&self) -> MUX_FW_P5_COREIO1_R {
        MUX_FW_P5_COREIO1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_coreio2(&self) -> MUX_FW_P6_COREIO2_R {
        MUX_FW_P6_COREIO2_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_coreio3(&self) -> MUX_FW_P7_COREIO3_R {
        MUX_FW_P7_COREIO3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vplus(&mut self) -> MUX_FW_P0_VPLUS_W {
        MUX_FW_P0_VPLUS_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vplus(&mut self) -> MUX_FW_P1_VPLUS_W {
        MUX_FW_P1_VPLUS_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vplus(&mut self) -> MUX_FW_P2_VPLUS_W {
        MUX_FW_P2_VPLUS_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vplus(&mut self) -> MUX_FW_P3_VPLUS_W {
        MUX_FW_P3_VPLUS_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vplus(&mut self) -> MUX_FW_P4_VPLUS_W {
        MUX_FW_P4_VPLUS_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vplus(&mut self) -> MUX_FW_P5_VPLUS_W {
        MUX_FW_P5_VPLUS_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vplus(&mut self) -> MUX_FW_P6_VPLUS_W {
        MUX_FW_P6_VPLUS_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vplus(&mut self) -> MUX_FW_P7_VPLUS_W {
        MUX_FW_P7_VPLUS_W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p0_vminus(&mut self) -> MUX_FW_P0_VMINUS_W {
        MUX_FW_P0_VMINUS_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p1_vminus(&mut self) -> MUX_FW_P1_VMINUS_W {
        MUX_FW_P1_VMINUS_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p2_vminus(&mut self) -> MUX_FW_P2_VMINUS_W {
        MUX_FW_P2_VMINUS_W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p3_vminus(&mut self) -> MUX_FW_P3_VMINUS_W {
        MUX_FW_P3_VMINUS_W { w: self }
    }
    #[doc = "Bit 12 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_vminus(&mut self) -> MUX_FW_P4_VMINUS_W {
        MUX_FW_P4_VMINUS_W { w: self }
    }
    #[doc = "Bit 13 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_vminus(&mut self) -> MUX_FW_P5_VMINUS_W {
        MUX_FW_P5_VMINUS_W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_vminus(&mut self) -> MUX_FW_P6_VMINUS_W {
        MUX_FW_P6_VMINUS_W { w: self }
    }
    #[doc = "Bit 15 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_vminus(&mut self) -> MUX_FW_P7_VMINUS_W {
        MUX_FW_P7_VMINUS_W { w: self }
    }
    #[doc = "Bit 16 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_vssa_vminus(&mut self) -> MUX_FW_VSSA_VMINUS_W {
        MUX_FW_VSSA_VMINUS_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_temp_vplus(&mut self) -> MUX_FW_TEMP_VPLUS_W {
        MUX_FW_TEMP_VPLUS_W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vplus(&mut self) -> MUX_FW_AMUXBUSA_VPLUS_W {
        MUX_FW_AMUXBUSA_VPLUS_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vplus(&mut self) -> MUX_FW_AMUXBUSB_VPLUS_W {
        MUX_FW_AMUXBUSB_VPLUS_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusa_vminus(&mut self) -> MUX_FW_AMUXBUSA_VMINUS_W {
        MUX_FW_AMUXBUSA_VMINUS_W { w: self }
    }
    #[doc = "Bit 21 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_amuxbusb_vminus(&mut self) -> MUX_FW_AMUXBUSB_VMINUS_W {
        MUX_FW_AMUXBUSB_VMINUS_W { w: self }
    }
    #[doc = "Bit 22 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vplus(&mut self) -> MUX_FW_SARBUS0_VPLUS_W {
        MUX_FW_SARBUS0_VPLUS_W { w: self }
    }
    #[doc = "Bit 23 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vplus(&mut self) -> MUX_FW_SARBUS1_VPLUS_W {
        MUX_FW_SARBUS1_VPLUS_W { w: self }
    }
    #[doc = "Bit 24 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus0_vminus(&mut self) -> MUX_FW_SARBUS0_VMINUS_W {
        MUX_FW_SARBUS0_VMINUS_W { w: self }
    }
    #[doc = "Bit 25 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_sarbus1_vminus(&mut self) -> MUX_FW_SARBUS1_VMINUS_W {
        MUX_FW_SARBUS1_VMINUS_W { w: self }
    }
    #[doc = "Bit 26 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p4_coreio0(&mut self) -> MUX_FW_P4_COREIO0_W {
        MUX_FW_P4_COREIO0_W { w: self }
    }
    #[doc = "Bit 27 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p5_coreio1(&mut self) -> MUX_FW_P5_COREIO1_W {
        MUX_FW_P5_COREIO1_W { w: self }
    }
    #[doc = "Bit 28 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p6_coreio2(&mut self) -> MUX_FW_P6_COREIO2_W {
        MUX_FW_P6_COREIO2_W { w: self }
    }
    #[doc = "Bit 29 - Write '1' to clear corresponding bit in MUX_SWITCH0"]
    #[inline(always)]
    pub fn mux_fw_p7_coreio3(&mut self) -> MUX_FW_P7_COREIO3_W {
        MUX_FW_P7_COREIO3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SARMUX Firmware switch control clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mux_switch_clear0](index.html) module"]
pub struct MUX_SWITCH_CLEAR0_SPEC;
impl crate::RegisterSpec for MUX_SWITCH_CLEAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mux_switch_clear0::R](R) reader structure"]
impl crate::Readable for MUX_SWITCH_CLEAR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mux_switch_clear0::W](W) writer structure"]
impl crate::Writable for MUX_SWITCH_CLEAR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUX_SWITCH_CLEAR0 to value 0"]
impl crate::Resettable for MUX_SWITCH_CLEAR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
