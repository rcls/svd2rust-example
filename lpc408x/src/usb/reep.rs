#[doc = "Register `REEP` reader"]
pub struct R(crate::R<REEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REEP` writer"]
pub struct W(crate::W<REEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REEP_SPEC>;
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
impl From<crate::W<REEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPR0` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR0_R(crate::FieldReader<bool, bool>);
impl EPR0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR0` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR0_W<'a> {
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
#[doc = "Field `EPR1` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR1_R(crate::FieldReader<bool, bool>);
impl EPR1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR1` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR1_W<'a> {
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
#[doc = "Field `EPR2` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR2_R(crate::FieldReader<bool, bool>);
impl EPR2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR2` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR2_W<'a> {
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
#[doc = "Field `EPR3` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR3_R(crate::FieldReader<bool, bool>);
impl EPR3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR3` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR3_W<'a> {
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
#[doc = "Field `EPR4` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR4_R(crate::FieldReader<bool, bool>);
impl EPR4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR4` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR4_W<'a> {
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
#[doc = "Field `EPR5` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR5_R(crate::FieldReader<bool, bool>);
impl EPR5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR5` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR5_W<'a> {
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
#[doc = "Field `EPR6` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR6_R(crate::FieldReader<bool, bool>);
impl EPR6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR6` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR6_W<'a> {
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
#[doc = "Field `EPR7` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR7_R(crate::FieldReader<bool, bool>);
impl EPR7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR7` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR7_W<'a> {
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
#[doc = "Field `EPR8` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR8_R(crate::FieldReader<bool, bool>);
impl EPR8_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR8` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR8_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR8_W<'a> {
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
#[doc = "Field `EPR9` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR9_R(crate::FieldReader<bool, bool>);
impl EPR9_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR9` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR9_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR9_W<'a> {
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
#[doc = "Field `EPR10` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR10_R(crate::FieldReader<bool, bool>);
impl EPR10_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR10` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR10_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR10_W<'a> {
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
#[doc = "Field `EPR11` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR11_R(crate::FieldReader<bool, bool>);
impl EPR11_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR11` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR11_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR11_W<'a> {
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
#[doc = "Field `EPR12` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR12_R(crate::FieldReader<bool, bool>);
impl EPR12_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR12` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR12_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR12_W<'a> {
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
#[doc = "Field `EPR13` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR13_R(crate::FieldReader<bool, bool>);
impl EPR13_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR13` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR13_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR13_W<'a> {
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
#[doc = "Field `EPR14` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR14_R(crate::FieldReader<bool, bool>);
impl EPR14_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR14` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR14_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR14_W<'a> {
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
#[doc = "Field `EPR15` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR15_R(crate::FieldReader<bool, bool>);
impl EPR15_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR15` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR15_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR15_W<'a> {
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
#[doc = "Field `EPR16` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR16_R(crate::FieldReader<bool, bool>);
impl EPR16_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR16` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR16_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR16_W<'a> {
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
#[doc = "Field `EPR17` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR17_R(crate::FieldReader<bool, bool>);
impl EPR17_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR17` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR17_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR17_W<'a> {
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
#[doc = "Field `EPR18` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR18_R(crate::FieldReader<bool, bool>);
impl EPR18_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR18` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR18_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR18_W<'a> {
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
#[doc = "Field `EPR19` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR19_R(crate::FieldReader<bool, bool>);
impl EPR19_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR19` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR19_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR19_W<'a> {
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
#[doc = "Field `EPR20` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR20_R(crate::FieldReader<bool, bool>);
impl EPR20_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR20` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR20_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR20_W<'a> {
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
#[doc = "Field `EPR21` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR21_R(crate::FieldReader<bool, bool>);
impl EPR21_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR21` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR21_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR21_W<'a> {
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
#[doc = "Field `EPR22` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR22_R(crate::FieldReader<bool, bool>);
impl EPR22_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR22` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR22_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR22_W<'a> {
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
#[doc = "Field `EPR23` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR23_R(crate::FieldReader<bool, bool>);
impl EPR23_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR23` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR23_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR23_W<'a> {
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
#[doc = "Field `EPR24` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR24_R(crate::FieldReader<bool, bool>);
impl EPR24_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR24` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR24_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR24_W<'a> {
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
#[doc = "Field `EPR25` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR25_R(crate::FieldReader<bool, bool>);
impl EPR25_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR25` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR25_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR25_W<'a> {
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
#[doc = "Field `EPR26` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR26_R(crate::FieldReader<bool, bool>);
impl EPR26_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR26` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR26_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR26_W<'a> {
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
#[doc = "Field `EPR27` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR27_R(crate::FieldReader<bool, bool>);
impl EPR27_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR27` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR27_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR27_W<'a> {
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
#[doc = "Field `EPR28` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR28_R(crate::FieldReader<bool, bool>);
impl EPR28_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR28` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR28_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR28_W<'a> {
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
#[doc = "Field `EPR29` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR29_R(crate::FieldReader<bool, bool>);
impl EPR29_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR29` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR29_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR29_W<'a> {
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
#[doc = "Field `EPR30` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR30_R(crate::FieldReader<bool, bool>);
impl EPR30_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR30` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR30_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `EPR31` reader - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR31_R(crate::FieldReader<bool, bool>);
impl EPR31_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPR31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPR31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPR31` writer - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
pub struct EPR31_W<'a> {
    w: &'a mut W,
}
impl<'a> EPR31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr0(&self) -> EPR0_R {
        EPR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr1(&self) -> EPR1_R {
        EPR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr2(&self) -> EPR2_R {
        EPR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr3(&self) -> EPR3_R {
        EPR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr4(&self) -> EPR4_R {
        EPR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr5(&self) -> EPR5_R {
        EPR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr6(&self) -> EPR6_R {
        EPR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr7(&self) -> EPR7_R {
        EPR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr8(&self) -> EPR8_R {
        EPR8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr9(&self) -> EPR9_R {
        EPR9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr10(&self) -> EPR10_R {
        EPR10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr11(&self) -> EPR11_R {
        EPR11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr12(&self) -> EPR12_R {
        EPR12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr13(&self) -> EPR13_R {
        EPR13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr14(&self) -> EPR14_R {
        EPR14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr15(&self) -> EPR15_R {
        EPR15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr16(&self) -> EPR16_R {
        EPR16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr17(&self) -> EPR17_R {
        EPR17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr18(&self) -> EPR18_R {
        EPR18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr19(&self) -> EPR19_R {
        EPR19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr20(&self) -> EPR20_R {
        EPR20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr21(&self) -> EPR21_R {
        EPR21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr22(&self) -> EPR22_R {
        EPR22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr23(&self) -> EPR23_R {
        EPR23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr24(&self) -> EPR24_R {
        EPR24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr25(&self) -> EPR25_R {
        EPR25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr26(&self) -> EPR26_R {
        EPR26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr27(&self) -> EPR27_R {
        EPR27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr28(&self) -> EPR28_R {
        EPR28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr29(&self) -> EPR29_R {
        EPR29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr30(&self) -> EPR30_R {
        EPR30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr31(&self) -> EPR31_R {
        EPR31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr0(&mut self) -> EPR0_W {
        EPR0_W { w: self }
    }
    #[doc = "Bit 1 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr1(&mut self) -> EPR1_W {
        EPR1_W { w: self }
    }
    #[doc = "Bit 2 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr2(&mut self) -> EPR2_W {
        EPR2_W { w: self }
    }
    #[doc = "Bit 3 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr3(&mut self) -> EPR3_W {
        EPR3_W { w: self }
    }
    #[doc = "Bit 4 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr4(&mut self) -> EPR4_W {
        EPR4_W { w: self }
    }
    #[doc = "Bit 5 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr5(&mut self) -> EPR5_W {
        EPR5_W { w: self }
    }
    #[doc = "Bit 6 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr6(&mut self) -> EPR6_W {
        EPR6_W { w: self }
    }
    #[doc = "Bit 7 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr7(&mut self) -> EPR7_W {
        EPR7_W { w: self }
    }
    #[doc = "Bit 8 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr8(&mut self) -> EPR8_W {
        EPR8_W { w: self }
    }
    #[doc = "Bit 9 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr9(&mut self) -> EPR9_W {
        EPR9_W { w: self }
    }
    #[doc = "Bit 10 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr10(&mut self) -> EPR10_W {
        EPR10_W { w: self }
    }
    #[doc = "Bit 11 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr11(&mut self) -> EPR11_W {
        EPR11_W { w: self }
    }
    #[doc = "Bit 12 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr12(&mut self) -> EPR12_W {
        EPR12_W { w: self }
    }
    #[doc = "Bit 13 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr13(&mut self) -> EPR13_W {
        EPR13_W { w: self }
    }
    #[doc = "Bit 14 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr14(&mut self) -> EPR14_W {
        EPR14_W { w: self }
    }
    #[doc = "Bit 15 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr15(&mut self) -> EPR15_W {
        EPR15_W { w: self }
    }
    #[doc = "Bit 16 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr16(&mut self) -> EPR16_W {
        EPR16_W { w: self }
    }
    #[doc = "Bit 17 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr17(&mut self) -> EPR17_W {
        EPR17_W { w: self }
    }
    #[doc = "Bit 18 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr18(&mut self) -> EPR18_W {
        EPR18_W { w: self }
    }
    #[doc = "Bit 19 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr19(&mut self) -> EPR19_W {
        EPR19_W { w: self }
    }
    #[doc = "Bit 20 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr20(&mut self) -> EPR20_W {
        EPR20_W { w: self }
    }
    #[doc = "Bit 21 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr21(&mut self) -> EPR21_W {
        EPR21_W { w: self }
    }
    #[doc = "Bit 22 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr22(&mut self) -> EPR22_W {
        EPR22_W { w: self }
    }
    #[doc = "Bit 23 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr23(&mut self) -> EPR23_W {
        EPR23_W { w: self }
    }
    #[doc = "Bit 24 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr24(&mut self) -> EPR24_W {
        EPR24_W { w: self }
    }
    #[doc = "Bit 25 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr25(&mut self) -> EPR25_W {
        EPR25_W { w: self }
    }
    #[doc = "Bit 26 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr26(&mut self) -> EPR26_W {
        EPR26_W { w: self }
    }
    #[doc = "Bit 27 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr27(&mut self) -> EPR27_W {
        EPR27_W { w: self }
    }
    #[doc = "Bit 28 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr28(&mut self) -> EPR28_W {
        EPR28_W { w: self }
    }
    #[doc = "Bit 29 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr29(&mut self) -> EPR29_W {
        EPR29_W { w: self }
    }
    #[doc = "Bit 30 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr30(&mut self) -> EPR30_W {
        EPR30_W { w: self }
    }
    #[doc = "Bit 31 - 0 = Endpoint EPxx is not realized. 1 = Endpoint EPxx is realized."]
    #[inline(always)]
    pub fn epr31(&mut self) -> EPR31_W {
        EPR31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Realize Endpoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reep](index.html) module"]
pub struct REEP_SPEC;
impl crate::RegisterSpec for REEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reep::R](R) reader structure"]
impl crate::Readable for REEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reep::W](W) writer structure"]
impl crate::Writable for REEP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REEP to value 0x03"]
impl crate::Resettable for REEP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
