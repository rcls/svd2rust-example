#[doc = "Register `SA1` reader"]
pub struct R(crate::R<SA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SA1` writer"]
pub struct W(crate::W<SA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SA1_SPEC>;
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
impl From<crate::W<SA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDR4` reader - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
pub struct SADDR4_R(crate::FieldReader<u8, u8>);
impl SADDR4_R {
    pub(crate) fn new(bits: u8) -> Self {
        SADDR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR4` writer - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
pub struct SADDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SADDR3` reader - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
pub struct SADDR3_R(crate::FieldReader<u8, u8>);
impl SADDR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        SADDR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR3` writer - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
pub struct SADDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&self) -> SADDR4_R {
        SADDR4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&self) -> SADDR3_R {
        SADDR3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&mut self) -> SADDR4_W {
        SADDR4_W { w: self }
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&mut self) -> SADDR3_W {
        SADDR3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Station Address 1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa1](index.html) module"]
pub struct SA1_SPEC;
impl crate::RegisterSpec for SA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa1::R](R) reader structure"]
impl crate::Readable for SA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa1::W](W) writer structure"]
impl crate::Writable for SA1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SA1 to value 0"]
impl crate::Resettable for SA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
