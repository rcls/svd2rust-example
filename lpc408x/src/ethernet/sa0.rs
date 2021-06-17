#[doc = "Register `SA0` reader"]
pub struct R(crate::R<SA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SA0` writer"]
pub struct W(crate::W<SA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SA0_SPEC>;
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
impl From<crate::W<SA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDR2` reader - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
pub struct SADDR2_R(crate::FieldReader<u8, u8>);
impl SADDR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SADDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR2` writer - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
pub struct SADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SADDR1` reader - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
pub struct SADDR1_R(crate::FieldReader<u8, u8>);
impl SADDR1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SADDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR1` writer - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
pub struct SADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
    #[inline(always)]
    pub fn saddr2(&self) -> SADDR2_R {
        SADDR2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
    #[inline(always)]
    pub fn saddr1(&self) -> SADDR1_R {
        SADDR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
    #[inline(always)]
    pub fn saddr2(&mut self) -> SADDR2_W {
        SADDR2_W { w: self }
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
    #[inline(always)]
    pub fn saddr1(&mut self) -> SADDR1_W {
        SADDR1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Station Address 0 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa0](index.html) module"]
pub struct SA0_SPEC;
impl crate::RegisterSpec for SA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa0::R](R) reader structure"]
impl crate::Readable for SA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa0::W](W) writer structure"]
impl crate::Writable for SA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SA0 to value 0"]
impl crate::Resettable for SA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
