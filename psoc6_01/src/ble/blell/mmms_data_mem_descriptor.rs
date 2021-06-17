#[doc = "Register `MMMS_DATA_MEM_DESCRIPTOR[%s]` reader"]
pub struct R(crate::R<MMMS_DATA_MEM_DESCRIPTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_DATA_MEM_DESCRIPTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_DATA_MEM_DESCRIPTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_DATA_MEM_DESCRIPTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMMS_DATA_MEM_DESCRIPTOR[%s]` writer"]
pub struct W(crate::W<MMMS_DATA_MEM_DESCRIPTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMMS_DATA_MEM_DESCRIPTOR_SPEC>;
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
impl From<crate::W<MMMS_DATA_MEM_DESCRIPTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMMS_DATA_MEM_DESCRIPTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLID_C1` reader - N/A"]
pub struct LLID_C1_R(crate::FieldReader<u8, u8>);
impl LLID_C1_R {
    pub(crate) fn new(bits: u8) -> Self {
        LLID_C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LLID_C1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLID_C1` writer - N/A"]
pub struct LLID_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> LLID_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DATA_LENGTH_C1` reader - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
pub struct DATA_LENGTH_C1_R(crate::FieldReader<u8, u8>);
impl DATA_LENGTH_C1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATA_LENGTH_C1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_LENGTH_C1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_LENGTH_C1` writer - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
pub struct DATA_LENGTH_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_LENGTH_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | ((value as u32 & 0xff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn llid_c1(&self) -> LLID_C1_R {
        LLID_C1_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:9 - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
    #[inline(always)]
    pub fn data_length_c1(&self) -> DATA_LENGTH_C1_R {
        DATA_LENGTH_C1_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn llid_c1(&mut self) -> LLID_C1_W {
        LLID_C1_W { w: self }
    }
    #[doc = "Bits 2:9 - This field indicates the length of the data packet. Bits \\[9:7\\]
are valid only if DLE is set. Range 0x00 to 0xFF."]
    #[inline(always)]
    pub fn data_length_c1(&mut self) -> DATA_LENGTH_C1_W {
        DATA_LENGTH_C1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data buffer descriptor 0 to 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_data_mem_descriptor](index.html) module"]
pub struct MMMS_DATA_MEM_DESCRIPTOR_SPEC;
impl crate::RegisterSpec for MMMS_DATA_MEM_DESCRIPTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_data_mem_descriptor::R](R) reader structure"]
impl crate::Readable for MMMS_DATA_MEM_DESCRIPTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmms_data_mem_descriptor::W](W) writer structure"]
impl crate::Writable for MMMS_DATA_MEM_DESCRIPTOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMMS_DATA_MEM_DESCRIPTOR[%s]
to value 0"]
impl crate::Resettable for MMMS_DATA_MEM_DESCRIPTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
