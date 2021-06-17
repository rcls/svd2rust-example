#[doc = "Register `SA2` reader"]
pub struct R(crate::R<SA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SA2` writer"]
pub struct W(crate::W<SA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SA2_SPEC>;
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
impl From<crate::W<SA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDR6` reader - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
pub struct SADDR6_R(crate::FieldReader<u8, u8>);
impl SADDR6_R {
    pub(crate) fn new(bits: u8) -> Self {
        SADDR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR6` writer - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
pub struct SADDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SADDR5` reader - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
pub struct SADDR5_R(crate::FieldReader<u8, u8>);
impl SADDR5_R {
    pub(crate) fn new(bits: u8) -> Self {
        SADDR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR5` writer - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
pub struct SADDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
    #[inline(always)]
    pub fn saddr6(&self) -> SADDR6_R {
        SADDR6_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
    #[inline(always)]
    pub fn saddr5(&self) -> SADDR5_R {
        SADDR5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
    #[inline(always)]
    pub fn saddr6(&mut self) -> SADDR6_W {
        SADDR6_W { w: self }
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
    #[inline(always)]
    pub fn saddr5(&mut self) -> SADDR5_W {
        SADDR5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Station Address 2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa2](index.html) module"]
pub struct SA2_SPEC;
impl crate::RegisterSpec for SA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa2::R](R) reader structure"]
impl crate::Readable for SA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa2::W](W) writer structure"]
impl crate::Writable for SA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SA2 to value 0"]
impl crate::Resettable for SA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
