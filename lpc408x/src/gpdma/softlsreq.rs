#[doc = "Register `SOFTLSREQ` reader"]
pub struct R(crate::R<SOFTLSREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTLSREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTLSREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTLSREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTLSREQ` writer"]
pub struct W(crate::W<SOFTLSREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTLSREQ_SPEC>;
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
impl From<crate::W<SOFTLSREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTLSREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFTLSREQ0` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ0_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ0` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ0_W<'a> {
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
#[doc = "Field `SOFTLSREQ1` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ1_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ1` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ1_W<'a> {
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
#[doc = "Field `SOFTLSREQ2` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ2_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ2` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ2_W<'a> {
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
#[doc = "Field `SOFTLSREQ3` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ3_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ3` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ3_W<'a> {
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
#[doc = "Field `SOFTLSREQ4` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ4_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ4` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ4_W<'a> {
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
#[doc = "Field `SOFTLSREQ5` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ5_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ5` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ5_W<'a> {
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
#[doc = "Field `SOFTLSREQ6` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ6_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ6` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ6_W<'a> {
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
#[doc = "Field `SOFTLSREQ7` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ7_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ7` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ7_W<'a> {
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
#[doc = "Field `SOFTLSREQ8` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ8_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ8` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ8_W<'a> {
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
#[doc = "Field `SOFTLSREQ9` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ9_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ9` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ9_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ9_W<'a> {
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
#[doc = "Field `SOFTLSREQ10` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ10_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ10` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ10_W<'a> {
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
#[doc = "Field `SOFTLSREQ11` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ11_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ11` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ11_W<'a> {
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
#[doc = "Field `SOFTLSREQ12` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ12_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ12` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ12_W<'a> {
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
#[doc = "Field `SOFTLSREQ13` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ13_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ13` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ13_W<'a> {
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
#[doc = "Field `SOFTLSREQ14` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ14_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ14` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ14_W<'a> {
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
#[doc = "Field `SOFTLSREQ15` reader - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ15_R(crate::FieldReader<bool, bool>);
impl SOFTLSREQ15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTLSREQ15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTLSREQ15` writer - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
pub struct SOFTLSREQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ15_W<'a> {
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
    #[doc = "Bit 0 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq0(&self) -> SOFTLSREQ0_R {
        SOFTLSREQ0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq1(&self) -> SOFTLSREQ1_R {
        SOFTLSREQ1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq2(&self) -> SOFTLSREQ2_R {
        SOFTLSREQ2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq3(&self) -> SOFTLSREQ3_R {
        SOFTLSREQ3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq4(&self) -> SOFTLSREQ4_R {
        SOFTLSREQ4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq5(&self) -> SOFTLSREQ5_R {
        SOFTLSREQ5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq6(&self) -> SOFTLSREQ6_R {
        SOFTLSREQ6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq7(&self) -> SOFTLSREQ7_R {
        SOFTLSREQ7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq8(&self) -> SOFTLSREQ8_R {
        SOFTLSREQ8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq9(&self) -> SOFTLSREQ9_R {
        SOFTLSREQ9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq10(&self) -> SOFTLSREQ10_R {
        SOFTLSREQ10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq11(&self) -> SOFTLSREQ11_R {
        SOFTLSREQ11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq12(&self) -> SOFTLSREQ12_R {
        SOFTLSREQ12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq13(&self) -> SOFTLSREQ13_R {
        SOFTLSREQ13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq14(&self) -> SOFTLSREQ14_R {
        SOFTLSREQ14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq15(&self) -> SOFTLSREQ15_R {
        SOFTLSREQ15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq0(&mut self) -> SOFTLSREQ0_W {
        SOFTLSREQ0_W { w: self }
    }
    #[doc = "Bit 1 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq1(&mut self) -> SOFTLSREQ1_W {
        SOFTLSREQ1_W { w: self }
    }
    #[doc = "Bit 2 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq2(&mut self) -> SOFTLSREQ2_W {
        SOFTLSREQ2_W { w: self }
    }
    #[doc = "Bit 3 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq3(&mut self) -> SOFTLSREQ3_W {
        SOFTLSREQ3_W { w: self }
    }
    #[doc = "Bit 4 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq4(&mut self) -> SOFTLSREQ4_W {
        SOFTLSREQ4_W { w: self }
    }
    #[doc = "Bit 5 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq5(&mut self) -> SOFTLSREQ5_W {
        SOFTLSREQ5_W { w: self }
    }
    #[doc = "Bit 6 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq6(&mut self) -> SOFTLSREQ6_W {
        SOFTLSREQ6_W { w: self }
    }
    #[doc = "Bit 7 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq7(&mut self) -> SOFTLSREQ7_W {
        SOFTLSREQ7_W { w: self }
    }
    #[doc = "Bit 8 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq8(&mut self) -> SOFTLSREQ8_W {
        SOFTLSREQ8_W { w: self }
    }
    #[doc = "Bit 9 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq9(&mut self) -> SOFTLSREQ9_W {
        SOFTLSREQ9_W { w: self }
    }
    #[doc = "Bit 10 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq10(&mut self) -> SOFTLSREQ10_W {
        SOFTLSREQ10_W { w: self }
    }
    #[doc = "Bit 11 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq11(&mut self) -> SOFTLSREQ11_W {
        SOFTLSREQ11_W { w: self }
    }
    #[doc = "Bit 12 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq12(&mut self) -> SOFTLSREQ12_W {
        SOFTLSREQ12_W { w: self }
    }
    #[doc = "Bit 13 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq13(&mut self) -> SOFTLSREQ13_W {
        SOFTLSREQ13_W { w: self }
    }
    #[doc = "Bit 14 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq14(&mut self) -> SOFTLSREQ14_W {
        SOFTLSREQ14_W { w: self }
    }
    #[doc = "Bit 15 - Software last single transfer request flags for each of 16 possible sources. Each bit represents one DMA request line or peripheral function: 0 - writing 0 has no effect. 1 - writing 1 generates a DMA last single transfer request for the corresponding request line."]
    #[inline(always)]
    pub fn softlsreq15(&mut self) -> SOFTLSREQ15_W {
        SOFTLSREQ15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Software Last Single Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softlsreq](index.html) module"]
pub struct SOFTLSREQ_SPEC;
impl crate::RegisterSpec for SOFTLSREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [softlsreq::R](R) reader structure"]
impl crate::Readable for SOFTLSREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [softlsreq::W](W) writer structure"]
impl crate::Writable for SOFTLSREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SOFTLSREQ to value 0"]
impl crate::Resettable for SOFTLSREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
