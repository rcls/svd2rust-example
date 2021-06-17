#[doc = "Register `EEPWRDWN` reader"]
pub struct R(crate::R<EEPWRDWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEPWRDWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEPWRDWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEPWRDWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEPWRDWN` writer"]
pub struct W(crate::W<EEPWRDWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEPWRDWN_SPEC>;
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
impl From<crate::W<EEPWRDWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEPWRDWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Power down mode bit. 0: not in power down mode. 1: power down mode (this will put all EEPROM devices in power down)."]
pub struct PWRDWN_R(crate::FieldReader<bool, bool>);
impl PWRDWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRDWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRDWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRDWN` writer - Power down mode bit. 0: not in power down mode. 1: power down mode (this will put all EEPROM devices in power down)."]
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Power down mode bit. 0: not in power down mode. 1: power down mode (this will put all EEPROM devices in power down)."]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power down mode bit. 0: not in power down mode. 1: power down mode (this will put all EEPROM devices in power down)."]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eepwrdwn](index.html) module"]
pub struct EEPWRDWN_SPEC;
impl crate::RegisterSpec for EEPWRDWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eepwrdwn::R](R) reader structure"]
impl crate::Readable for EEPWRDWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eepwrdwn::W](W) writer structure"]
impl crate::Writable for EEPWRDWN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEPWRDWN to value 0"]
impl crate::Resettable for EEPWRDWN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
