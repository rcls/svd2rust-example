#[doc = "Register `EEADDR` reader"]
pub struct R(crate::R<EEADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEADDR` writer"]
pub struct W(crate::W<EEADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEADDR_SPEC>;
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
impl From<crate::W<EEADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - EEPROM Address. Lower 6 bits are don't care."]
pub struct ADDR_R(crate::FieldReader<u16, u16>);
impl ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - EEPROM Address. Lower 6 bits are don't care."]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - EEPROM Address. Lower 6 bits are don't care."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - EEPROM Address. Lower 6 bits are don't care."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeaddr](index.html) module"]
pub struct EEADDR_SPEC;
impl crate::RegisterSpec for EEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eeaddr::R](R) reader structure"]
impl crate::Readable for EEADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eeaddr::W](W) writer structure"]
impl crate::Writable for EEADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEADDR to value 0"]
impl crate::Resettable for EEADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
