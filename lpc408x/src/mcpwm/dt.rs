#[doc = "Register `DT` reader"]
pub struct R(crate::R<DT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DT` writer"]
pub struct W(crate::W<DT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DT_SPEC>;
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
impl From<crate::W<DT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT0` reader - Dead time for channel 0.\\[1\\]"]
pub struct DT0_R(crate::FieldReader<u16, u16>);
impl DT0_R {
    pub(crate) fn new(bits: u16) -> Self {
        DT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT0` writer - Dead time for channel 0.\\[1\\]"]
pub struct DT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `DT1` reader - Dead time for channel 1.\\[2\\]"]
pub struct DT1_R(crate::FieldReader<u16, u16>);
impl DT1_R {
    pub(crate) fn new(bits: u16) -> Self {
        DT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT1` writer - Dead time for channel 1.\\[2\\]"]
pub struct DT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `DT2` reader - Dead time for channel 2.\\[2\\]"]
pub struct DT2_R(crate::FieldReader<u16, u16>);
impl DT2_R {
    pub(crate) fn new(bits: u16) -> Self {
        DT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DT2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DT2` writer - Dead time for channel 2.\\[2\\]"]
pub struct DT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&self) -> DT2_R {
        DT2_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&mut self) -> DT0_W {
        DT0_W { w: self }
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&mut self) -> DT1_W {
        DT1_W { w: self }
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&mut self) -> DT2_W {
        DT2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dead time register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dt](index.html) module"]
pub struct DT_SPEC;
impl crate::RegisterSpec for DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dt::R](R) reader structure"]
impl crate::Readable for DT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dt::W](W) writer structure"]
impl crate::Writable for DT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DT to value 0x3fff_ffff"]
impl crate::Resettable for DT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3fff_ffff
    }
}
