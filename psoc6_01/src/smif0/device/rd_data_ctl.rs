#[doc = "Register `RD_DATA_CTL` reader"]
pub struct R(crate::R<RD_DATA_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_DATA_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_DATA_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_DATA_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_DATA_CTL` writer"]
pub struct W(crate::W<RD_DATA_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_DATA_CTL_SPEC>;
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
impl From<crate::W<RD_DATA_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_DATA_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub struct WIDTH_R(crate::FieldReader<u8, u8>);
impl WIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub struct WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&mut self) -> WIDTH_W {
        WIDTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read data control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_data_ctl](index.html) module"]
pub struct RD_DATA_CTL_SPEC;
impl crate::RegisterSpec for RD_DATA_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_data_ctl::R](R) reader structure"]
impl crate::Readable for RD_DATA_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_data_ctl::W](W) writer structure"]
impl crate::Writable for RD_DATA_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RD_DATA_CTL to value 0"]
impl crate::Resettable for RD_DATA_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
