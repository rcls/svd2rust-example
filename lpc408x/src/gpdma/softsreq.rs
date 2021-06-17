#[doc = "Register `SOFTSREQ` reader"]
pub struct R(crate::R<SOFTSREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTSREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTSREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTSREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTSREQ` writer"]
pub struct W(crate::W<SOFTSREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTSREQ_SPEC>;
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
impl From<crate::W<SOFTSREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTSREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTSREQ0` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ0_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ0` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ0_W<'a> {
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
#[doc = "Field `SOFTSREQ1` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ1_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ1` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ1_W<'a> {
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
#[doc = "Field `SOFTSREQ2` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ2_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ2` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ2_W<'a> {
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
#[doc = "Field `SOFTSREQ3` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ3_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ3` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ3_W<'a> {
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
#[doc = "Field `SOFTSREQ4` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ4_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ4` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ4_W<'a> {
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
#[doc = "Field `SOFTSREQ5` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ5_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ5` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ5_W<'a> {
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
#[doc = "Field `SOFTSREQ6` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ6_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ6` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ6_W<'a> {
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
#[doc = "Field `SOFTSREQ7` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ7_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ7` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ7_W<'a> {
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
#[doc = "Field `SOFTSREQ8` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ8_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ8` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ8_W<'a> {
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
#[doc = "Field `SOFTSREQ9` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ9_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ9` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ9_W<'a> {
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
#[doc = "Field `SOFTSREQ10` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ10_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ10` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ10_W<'a> {
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
#[doc = "Field `SOFTSREQ11` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ11_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ11` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ11_W<'a> {
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
#[doc = "Field `SOFTSREQ12` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ12_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ12` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ12_W<'a> {
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
#[doc = "Field `SOFTSREQ13` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ13_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ13` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ13_W<'a> {
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
#[doc = "Field `SOFTSREQ14` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ14_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ14` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ14_W<'a> {
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
#[doc = "Field `SOFTSREQ15` reader - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ15_R(crate::FieldReader<bool, bool>);
impl SOFTSREQ15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTSREQ15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTSREQ15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTSREQ15` writer - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
pub struct SOFTSREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTSREQ15_W<'a> {
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
    #[doc = "Bit 0 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq0(&self) -> SOFTSREQ0_R {
        SOFTSREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq1(&self) -> SOFTSREQ1_R {
        SOFTSREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq2(&self) -> SOFTSREQ2_R {
        SOFTSREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq3(&self) -> SOFTSREQ3_R {
        SOFTSREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq4(&self) -> SOFTSREQ4_R {
        SOFTSREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq5(&self) -> SOFTSREQ5_R {
        SOFTSREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq6(&self) -> SOFTSREQ6_R {
        SOFTSREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq7(&self) -> SOFTSREQ7_R {
        SOFTSREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq8(&self) -> SOFTSREQ8_R {
        SOFTSREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq9(&self) -> SOFTSREQ9_R {
        SOFTSREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq10(&self) -> SOFTSREQ10_R {
        SOFTSREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq11(&self) -> SOFTSREQ11_R {
        SOFTSREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq12(&self) -> SOFTSREQ12_R {
        SOFTSREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq13(&self) -> SOFTSREQ13_R {
        SOFTSREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq14(&self) -> SOFTSREQ14_R {
        SOFTSREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq15(&self) -> SOFTSREQ15_R {
        SOFTSREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq0(&mut self) -> SOFTSREQ0_W {
        SOFTSREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq1(&mut self) -> SOFTSREQ1_W {
        SOFTSREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq2(&mut self) -> SOFTSREQ2_W {
        SOFTSREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq3(&mut self) -> SOFTSREQ3_W {
        SOFTSREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq4(&mut self) -> SOFTSREQ4_W {
        SOFTSREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq5(&mut self) -> SOFTSREQ5_W {
        SOFTSREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq6(&mut self) -> SOFTSREQ6_W {
        SOFTSREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq7(&mut self) -> SOFTSREQ7_W {
        SOFTSREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq8(&mut self) -> SOFTSREQ8_W {
        SOFTSREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq9(&mut self) -> SOFTSREQ9_W {
        SOFTSREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq10(&mut self) -> SOFTSREQ10_W {
        SOFTSREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq11(&mut self) -> SOFTSREQ11_W {
        SOFTSREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq12(&mut self) -> SOFTSREQ12_W {
        SOFTSREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq13(&mut self) -> SOFTSREQ13_W {
        SOFTSREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq14(&mut self) -> SOFTSREQ14_W {
        SOFTSREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Software single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softsreq15(&mut self) -> SOFTSREQ15_W {
        SOFTSREQ15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Software Single Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softsreq](index.html) module"]
pub struct SOFTSREQ_SPEC;
impl crate::RegisterSpec for SOFTSREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [softsreq::R](R) reader structure"]
impl crate::Readable for SOFTSREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [softsreq::W](W) writer structure"]
impl crate::Writable for SOFTSREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTSREQ to value 0"]
impl crate::Resettable for SOFTSREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
