#[doc = "Register `DATA_CHANNELS_L1` reader"]
pub struct R(crate::R<DATA_CHANNELS_L1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_CHANNELS_L1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_CHANNELS_L1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_CHANNELS_L1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_CHANNELS_L1` writer"]
pub struct W(crate::W<DATA_CHANNELS_L1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_CHANNELS_L1_SPEC>;
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
impl From<crate::W<DATA_CHANNELS_L1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_CHANNELS_L1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_CHANNELS_L1` reader - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
pub struct DATA_CHANNELS_L1_R(crate::FieldReader<u16, u16>);
impl DATA_CHANNELS_L1_R {
    pub(crate) fn new(bits: u16) -> Self {
        DATA_CHANNELS_L1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_CHANNELS_L1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_CHANNELS_L1` writer - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
pub struct DATA_CHANNELS_L1_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_CHANNELS_L1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_l1(&self) -> DATA_CHANNELS_L1_R {
        DATA_CHANNELS_L1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register field indicates which of the data channels are in use. This stores the information for the lower 16 (15:0) data channel indices. '1' indicates the corresponding data channel is used and '0' indicates the channel is unused."]
    #[inline(always)]
    pub fn data_channels_l1(&mut self) -> DATA_CHANNELS_L1_W {
        DATA_CHANNELS_L1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data channel map 1 (lower word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_channels_l1](index.html) module"]
pub struct DATA_CHANNELS_L1_SPEC;
impl crate::RegisterSpec for DATA_CHANNELS_L1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_channels_l1::R](R) reader structure"]
impl crate::Readable for DATA_CHANNELS_L1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_channels_l1::W](W) writer structure"]
impl crate::Writable for DATA_CHANNELS_L1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_CHANNELS_L1 to value 0"]
impl crate::Resettable for DATA_CHANNELS_L1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
