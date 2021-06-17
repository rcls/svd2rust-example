#[doc = "Register `ENR2` reader"]
pub struct R(crate::R<ENR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENR2` writer"]
pub struct W(crate::W<ENR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENR2_SPEC>;
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
impl From<crate::W<ENR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2_0ER` reader - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_0ER_R(crate::FieldReader<bool, bool>);
impl P2_0ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_0ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_0ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_0ER` writer - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_0ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_0ER_W<'a> {
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
#[doc = "Field `P2_1ER` reader - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_1ER_R(crate::FieldReader<bool, bool>);
impl P2_1ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_1ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_1ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_1ER` writer - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_1ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_1ER_W<'a> {
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
#[doc = "Field `P2_2ER` reader - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_2ER_R(crate::FieldReader<bool, bool>);
impl P2_2ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_2ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_2ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_2ER` writer - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_2ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_2ER_W<'a> {
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
#[doc = "Field `P2_3ER` reader - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_3ER_R(crate::FieldReader<bool, bool>);
impl P2_3ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_3ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_3ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_3ER` writer - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_3ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_3ER_W<'a> {
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
#[doc = "Field `P2_4ER` reader - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_4ER_R(crate::FieldReader<bool, bool>);
impl P2_4ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_4ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_4ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_4ER` writer - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_4ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_4ER_W<'a> {
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
#[doc = "Field `P2_5ER` reader - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_5ER_R(crate::FieldReader<bool, bool>);
impl P2_5ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_5ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_5ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_5ER` writer - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_5ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_5ER_W<'a> {
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
#[doc = "Field `P2_6ER` reader - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_6ER_R(crate::FieldReader<bool, bool>);
impl P2_6ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_6ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_6ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_6ER` writer - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_6ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_6ER_W<'a> {
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
#[doc = "Field `P2_7ER` reader - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_7ER_R(crate::FieldReader<bool, bool>);
impl P2_7ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_7ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_7ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_7ER` writer - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_7ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_7ER_W<'a> {
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
#[doc = "Field `P2_8ER` reader - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_8ER_R(crate::FieldReader<bool, bool>);
impl P2_8ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_8ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_8ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_8ER` writer - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_8ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_8ER_W<'a> {
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
#[doc = "Field `P2_9ER` reader - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_9ER_R(crate::FieldReader<bool, bool>);
impl P2_9ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_9ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_9ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_9ER` writer - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_9ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_9ER_W<'a> {
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
#[doc = "Field `P2_10ER` reader - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_10ER_R(crate::FieldReader<bool, bool>);
impl P2_10ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_10ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_10ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_10ER` writer - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_10ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10ER_W<'a> {
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
#[doc = "Field `P2_11ER` reader - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_11ER_R(crate::FieldReader<bool, bool>);
impl P2_11ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_11ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_11ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_11ER` writer - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_11ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11ER_W<'a> {
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
#[doc = "Field `P2_12ER` reader - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_12ER_R(crate::FieldReader<bool, bool>);
impl P2_12ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_12ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_12ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_12ER` writer - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_12ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12ER_W<'a> {
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
#[doc = "Field `P2_13ER` reader - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_13ER_R(crate::FieldReader<bool, bool>);
impl P2_13ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_13ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_13ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_13ER` writer - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_13ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13ER_W<'a> {
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
#[doc = "Field `P2_14ER` reader - Enable rising edge interrupt for P2\\[14\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_14ER_R(crate::FieldReader<bool, bool>);
impl P2_14ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_14ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_14ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_14ER` writer - Enable rising edge interrupt for P2\\[14\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_14ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_14ER_W<'a> {
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
#[doc = "Field `P2_15ER` reader - Enable rising edge interrupt for P2\\[15\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_15ER_R(crate::FieldReader<bool, bool>);
impl P2_15ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_15ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_15ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_15ER` writer - Enable rising edge interrupt for P2\\[15\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_15ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_15ER_W<'a> {
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
#[doc = "Field `P2_16ER` reader - Enable rising edge interrupt for P2\\[16\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_16ER_R(crate::FieldReader<bool, bool>);
impl P2_16ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_16ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_16ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_16ER` writer - Enable rising edge interrupt for P2\\[16\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_16ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_16ER_W<'a> {
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
#[doc = "Field `P2_17ER` reader - Enable rising edge interrupt for P2\\[17\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_17ER_R(crate::FieldReader<bool, bool>);
impl P2_17ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_17ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_17ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_17ER` writer - Enable rising edge interrupt for P2\\[17\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_17ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_17ER_W<'a> {
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
#[doc = "Field `P2_18ER` reader - Enable rising edge interrupt for P2\\[18\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_18ER_R(crate::FieldReader<bool, bool>);
impl P2_18ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_18ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_18ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_18ER` writer - Enable rising edge interrupt for P2\\[18\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_18ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_18ER_W<'a> {
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
#[doc = "Field `P2_19ER` reader - Enable rising edge interrupt for P2\\[19\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_19ER_R(crate::FieldReader<bool, bool>);
impl P2_19ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_19ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_19ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_19ER` writer - Enable rising edge interrupt for P2\\[19\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_19ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_19ER_W<'a> {
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
#[doc = "Field `P2_20ER` reader - Enable rising edge interrupt for P2\\[20\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_20ER_R(crate::FieldReader<bool, bool>);
impl P2_20ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_20ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_20ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_20ER` writer - Enable rising edge interrupt for P2\\[20\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_20ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_20ER_W<'a> {
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
#[doc = "Field `P2_21ER` reader - Enable rising edge interrupt for P2\\[21\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_21ER_R(crate::FieldReader<bool, bool>);
impl P2_21ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_21ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_21ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_21ER` writer - Enable rising edge interrupt for P2\\[21\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_21ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_21ER_W<'a> {
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
#[doc = "Field `P2_22ER` reader - Enable rising edge interrupt for P2\\[22\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_22ER_R(crate::FieldReader<bool, bool>);
impl P2_22ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_22ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_22ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_22ER` writer - Enable rising edge interrupt for P2\\[22\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_22ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_22ER_W<'a> {
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
#[doc = "Field `P2_23ER` reader - Enable rising edge interrupt for P2\\[23\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_23ER_R(crate::FieldReader<bool, bool>);
impl P2_23ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_23ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_23ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_23ER` writer - Enable rising edge interrupt for P2\\[23\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_23ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_23ER_W<'a> {
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
#[doc = "Field `P2_24ER` reader - Enable rising edge interrupt for P2\\[24\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_24ER_R(crate::FieldReader<bool, bool>);
impl P2_24ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_24ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_24ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_24ER` writer - Enable rising edge interrupt for P2\\[24\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_24ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_24ER_W<'a> {
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
#[doc = "Field `P2_25ER` reader - Enable rising edge interrupt for P2\\[25\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_25ER_R(crate::FieldReader<bool, bool>);
impl P2_25ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_25ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_25ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_25ER` writer - Enable rising edge interrupt for P2\\[25\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_25ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_25ER_W<'a> {
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
#[doc = "Field `P2_26ER` reader - Enable rising edge interrupt for P2\\[26\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_26ER_R(crate::FieldReader<bool, bool>);
impl P2_26ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_26ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_26ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_26ER` writer - Enable rising edge interrupt for P2\\[26\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_26ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_26ER_W<'a> {
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
#[doc = "Field `P2_27ER` reader - Enable rising edge interrupt for P2\\[27\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_27ER_R(crate::FieldReader<bool, bool>);
impl P2_27ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_27ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_27ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_27ER` writer - Enable rising edge interrupt for P2\\[27\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_27ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_27ER_W<'a> {
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
#[doc = "Field `P2_28ER` reader - Enable rising edge interrupt for P2\\[28\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_28ER_R(crate::FieldReader<bool, bool>);
impl P2_28ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_28ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_28ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_28ER` writer - Enable rising edge interrupt for P2\\[28\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_28ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_28ER_W<'a> {
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
#[doc = "Field `P2_29ER` reader - Enable rising edge interrupt for P2\\[29\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_29ER_R(crate::FieldReader<bool, bool>);
impl P2_29ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_29ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_29ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_29ER` writer - Enable rising edge interrupt for P2\\[29\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_29ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_29ER_W<'a> {
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
#[doc = "Field `P2_30ER` reader - Enable rising edge interrupt for P2\\[30\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_30ER_R(crate::FieldReader<bool, bool>);
impl P2_30ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_30ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_30ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_30ER` writer - Enable rising edge interrupt for P2\\[30\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_30ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_30ER_W<'a> {
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
#[doc = "Field `P2_31ER` reader - Enable rising edge interrupt for P2\\[31\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_31ER_R(crate::FieldReader<bool, bool>);
impl P2_31ER_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_31ER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_31ER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_31ER` writer - Enable rising edge interrupt for P2\\[31\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
pub struct P2_31ER_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_31ER_W<'a> {
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
    #[doc = "Bit 0 - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_0er(&self) -> P2_0ER_R {
        P2_0ER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_1er(&self) -> P2_1ER_R {
        P2_1ER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_2er(&self) -> P2_2ER_R {
        P2_2ER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_3er(&self) -> P2_3ER_R {
        P2_3ER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_4er(&self) -> P2_4ER_R {
        P2_4ER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_5er(&self) -> P2_5ER_R {
        P2_5ER_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_6er(&self) -> P2_6ER_R {
        P2_6ER_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_7er(&self) -> P2_7ER_R {
        P2_7ER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_8er(&self) -> P2_8ER_R {
        P2_8ER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_9er(&self) -> P2_9ER_R {
        P2_9ER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_10er(&self) -> P2_10ER_R {
        P2_10ER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_11er(&self) -> P2_11ER_R {
        P2_11ER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_12er(&self) -> P2_12ER_R {
        P2_12ER_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_13er(&self) -> P2_13ER_R {
        P2_13ER_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable rising edge interrupt for P2\\[14\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_14er(&self) -> P2_14ER_R {
        P2_14ER_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable rising edge interrupt for P2\\[15\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_15er(&self) -> P2_15ER_R {
        P2_15ER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable rising edge interrupt for P2\\[16\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_16er(&self) -> P2_16ER_R {
        P2_16ER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable rising edge interrupt for P2\\[17\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_17er(&self) -> P2_17ER_R {
        P2_17ER_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable rising edge interrupt for P2\\[18\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_18er(&self) -> P2_18ER_R {
        P2_18ER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable rising edge interrupt for P2\\[19\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_19er(&self) -> P2_19ER_R {
        P2_19ER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable rising edge interrupt for P2\\[20\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_20er(&self) -> P2_20ER_R {
        P2_20ER_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable rising edge interrupt for P2\\[21\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_21er(&self) -> P2_21ER_R {
        P2_21ER_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable rising edge interrupt for P2\\[22\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_22er(&self) -> P2_22ER_R {
        P2_22ER_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable rising edge interrupt for P2\\[23\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_23er(&self) -> P2_23ER_R {
        P2_23ER_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable rising edge interrupt for P2\\[24\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_24er(&self) -> P2_24ER_R {
        P2_24ER_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable rising edge interrupt for P2\\[25\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_25er(&self) -> P2_25ER_R {
        P2_25ER_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable rising edge interrupt for P2\\[26\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_26er(&self) -> P2_26ER_R {
        P2_26ER_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable rising edge interrupt for P2\\[27\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_27er(&self) -> P2_27ER_R {
        P2_27ER_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable rising edge interrupt for P2\\[28\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_28er(&self) -> P2_28ER_R {
        P2_28ER_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable rising edge interrupt for P2\\[29\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_29er(&self) -> P2_29ER_R {
        P2_29ER_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable rising edge interrupt for P2\\[30\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_30er(&self) -> P2_30ER_R {
        P2_30ER_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable rising edge interrupt for P2\\[31\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_31er(&self) -> P2_31ER_R {
        P2_31ER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable rising edge interrupt for P2\\[0\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_0er(&mut self) -> P2_0ER_W {
        P2_0ER_W { w: self }
    }
    #[doc = "Bit 1 - Enable rising edge interrupt for P2\\[1\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_1er(&mut self) -> P2_1ER_W {
        P2_1ER_W { w: self }
    }
    #[doc = "Bit 2 - Enable rising edge interrupt for P2\\[2\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_2er(&mut self) -> P2_2ER_W {
        P2_2ER_W { w: self }
    }
    #[doc = "Bit 3 - Enable rising edge interrupt for P2\\[3\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_3er(&mut self) -> P2_3ER_W {
        P2_3ER_W { w: self }
    }
    #[doc = "Bit 4 - Enable rising edge interrupt for P2\\[4\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_4er(&mut self) -> P2_4ER_W {
        P2_4ER_W { w: self }
    }
    #[doc = "Bit 5 - Enable rising edge interrupt for P2\\[5\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_5er(&mut self) -> P2_5ER_W {
        P2_5ER_W { w: self }
    }
    #[doc = "Bit 6 - Enable rising edge interrupt for P2\\[6\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_6er(&mut self) -> P2_6ER_W {
        P2_6ER_W { w: self }
    }
    #[doc = "Bit 7 - Enable rising edge interrupt for P2\\[7\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_7er(&mut self) -> P2_7ER_W {
        P2_7ER_W { w: self }
    }
    #[doc = "Bit 8 - Enable rising edge interrupt for P2\\[8\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_8er(&mut self) -> P2_8ER_W {
        P2_8ER_W { w: self }
    }
    #[doc = "Bit 9 - Enable rising edge interrupt for P2\\[9\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_9er(&mut self) -> P2_9ER_W {
        P2_9ER_W { w: self }
    }
    #[doc = "Bit 10 - Enable rising edge interrupt for P2\\[10\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_10er(&mut self) -> P2_10ER_W {
        P2_10ER_W { w: self }
    }
    #[doc = "Bit 11 - Enable rising edge interrupt for P2\\[11\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_11er(&mut self) -> P2_11ER_W {
        P2_11ER_W { w: self }
    }
    #[doc = "Bit 12 - Enable rising edge interrupt for P2\\[12\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_12er(&mut self) -> P2_12ER_W {
        P2_12ER_W { w: self }
    }
    #[doc = "Bit 13 - Enable rising edge interrupt for P2\\[13\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_13er(&mut self) -> P2_13ER_W {
        P2_13ER_W { w: self }
    }
    #[doc = "Bit 14 - Enable rising edge interrupt for P2\\[14\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_14er(&mut self) -> P2_14ER_W {
        P2_14ER_W { w: self }
    }
    #[doc = "Bit 15 - Enable rising edge interrupt for P2\\[15\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_15er(&mut self) -> P2_15ER_W {
        P2_15ER_W { w: self }
    }
    #[doc = "Bit 16 - Enable rising edge interrupt for P2\\[16\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_16er(&mut self) -> P2_16ER_W {
        P2_16ER_W { w: self }
    }
    #[doc = "Bit 17 - Enable rising edge interrupt for P2\\[17\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_17er(&mut self) -> P2_17ER_W {
        P2_17ER_W { w: self }
    }
    #[doc = "Bit 18 - Enable rising edge interrupt for P2\\[18\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_18er(&mut self) -> P2_18ER_W {
        P2_18ER_W { w: self }
    }
    #[doc = "Bit 19 - Enable rising edge interrupt for P2\\[19\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_19er(&mut self) -> P2_19ER_W {
        P2_19ER_W { w: self }
    }
    #[doc = "Bit 20 - Enable rising edge interrupt for P2\\[20\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_20er(&mut self) -> P2_20ER_W {
        P2_20ER_W { w: self }
    }
    #[doc = "Bit 21 - Enable rising edge interrupt for P2\\[21\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_21er(&mut self) -> P2_21ER_W {
        P2_21ER_W { w: self }
    }
    #[doc = "Bit 22 - Enable rising edge interrupt for P2\\[22\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_22er(&mut self) -> P2_22ER_W {
        P2_22ER_W { w: self }
    }
    #[doc = "Bit 23 - Enable rising edge interrupt for P2\\[23\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_23er(&mut self) -> P2_23ER_W {
        P2_23ER_W { w: self }
    }
    #[doc = "Bit 24 - Enable rising edge interrupt for P2\\[24\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_24er(&mut self) -> P2_24ER_W {
        P2_24ER_W { w: self }
    }
    #[doc = "Bit 25 - Enable rising edge interrupt for P2\\[25\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_25er(&mut self) -> P2_25ER_W {
        P2_25ER_W { w: self }
    }
    #[doc = "Bit 26 - Enable rising edge interrupt for P2\\[26\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_26er(&mut self) -> P2_26ER_W {
        P2_26ER_W { w: self }
    }
    #[doc = "Bit 27 - Enable rising edge interrupt for P2\\[27\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_27er(&mut self) -> P2_27ER_W {
        P2_27ER_W { w: self }
    }
    #[doc = "Bit 28 - Enable rising edge interrupt for P2\\[28\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_28er(&mut self) -> P2_28ER_W {
        P2_28ER_W { w: self }
    }
    #[doc = "Bit 29 - Enable rising edge interrupt for P2\\[29\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_29er(&mut self) -> P2_29ER_W {
        P2_29ER_W { w: self }
    }
    #[doc = "Bit 30 - Enable rising edge interrupt for P2\\[30\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_30er(&mut self) -> P2_30ER_W {
        P2_30ER_W { w: self }
    }
    #[doc = "Bit 31 - Enable rising edge interrupt for P2\\[31\\]. 0 = Disable rising edge interrupt. 1 = Enable rising edge interrupt."]
    #[inline(always)]
    pub fn p2_31er(&mut self) -> P2_31ER_W {
        P2_31ER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enr2](index.html) module"]
pub struct ENR2_SPEC;
impl crate::RegisterSpec for ENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enr2::R](R) reader structure"]
impl crate::Readable for ENR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enr2::W](W) writer structure"]
impl crate::Writable for ENR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENR2 to value 0"]
impl crate::Resettable for ENR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
