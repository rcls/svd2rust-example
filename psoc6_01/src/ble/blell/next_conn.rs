#[doc = "Register `NEXT_CONN` reader"]
pub struct R(crate::R<NEXT_CONN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEXT_CONN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEXT_CONN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEXT_CONN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NEXT_CONN` writer"]
pub struct W(crate::W<NEXT_CONN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NEXT_CONN_SPEC>;
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
impl From<crate::W<NEXT_CONN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NEXT_CONN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEXT_CONN_INDEX` reader - Connection Index to be serviced. Allowed values are 0,1,2,3."]
pub struct NEXT_CONN_INDEX_R(crate::FieldReader<u8, u8>);
impl NEXT_CONN_INDEX_R {
    pub(crate) fn new(bits: u8) -> Self {
        NEXT_CONN_INDEX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEXT_CONN_INDEX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEXT_CONN_INDEX` writer - Connection Index to be serviced. Allowed values are 0,1,2,3."]
pub struct NEXT_CONN_INDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_CONN_INDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `NEXT_CONN_TYPE` reader - Connection type 1 - Master Connection 0 - Slave Connection"]
pub struct NEXT_CONN_TYPE_R(crate::FieldReader<bool, bool>);
impl NEXT_CONN_TYPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEXT_CONN_TYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEXT_CONN_TYPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEXT_CONN_TYPE` writer - Connection type 1 - Master Connection 0 - Slave Connection"]
pub struct NEXT_CONN_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> NEXT_CONN_TYPE_W<'a> {
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
#[doc = "Field `NI_VALID` reader - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
pub struct NI_VALID_R(crate::FieldReader<bool, bool>);
impl NI_VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        NI_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NI_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NI_VALID` writer - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
pub struct NI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> NI_VALID_W<'a> {
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
impl R {
    #[doc = "Bits 0:4 - Connection Index to be serviced. Allowed values are 0,1,2,3."]
    #[inline(always)]
    pub fn next_conn_index(&self) -> NEXT_CONN_INDEX_R {
        NEXT_CONN_INDEX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Connection type 1 - Master Connection 0 - Slave Connection"]
    #[inline(always)]
    pub fn next_conn_type(&self) -> NEXT_CONN_TYPE_R {
        NEXT_CONN_TYPE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
    #[inline(always)]
    pub fn ni_valid(&self) -> NI_VALID_R {
        NI_VALID_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Connection Index to be serviced. Allowed values are 0,1,2,3."]
    #[inline(always)]
    pub fn next_conn_index(&mut self) -> NEXT_CONN_INDEX_W {
        NEXT_CONN_INDEX_W { w: self }
    }
    #[doc = "Bit 5 - Connection type 1 - Master Connection 0 - Slave Connection"]
    #[inline(always)]
    pub fn next_conn_type(&mut self) -> NEXT_CONN_TYPE_W {
        NEXT_CONN_TYPE_W { w: self }
    }
    #[doc = "Bit 6 - Flag indication if programmed NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the connection of if NI_TIMER is pointing to past value"]
    #[inline(always)]
    pub fn ni_valid(&mut self) -> NI_VALID_W {
        NI_VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Next Connection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [next_conn](index.html) module"]
pub struct NEXT_CONN_SPEC;
impl crate::RegisterSpec for NEXT_CONN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [next_conn::R](R) reader structure"]
impl crate::Readable for NEXT_CONN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [next_conn::W](W) writer structure"]
impl crate::Writable for NEXT_CONN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NEXT_CONN to value 0"]
impl crate::Resettable for NEXT_CONN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
