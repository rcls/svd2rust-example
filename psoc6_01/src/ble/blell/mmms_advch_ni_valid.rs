#[doc = "Register `MMMS_ADVCH_NI_VALID` reader"]
pub struct R(crate::R<MMMS_ADVCH_NI_VALID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMMS_ADVCH_NI_VALID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMMS_ADVCH_NI_VALID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMMS_ADVCH_NI_VALID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMMS_ADVCH_NI_VALID` writer"]
pub struct W(crate::W<MMMS_ADVCH_NI_VALID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMMS_ADVCH_NI_VALID_SPEC>;
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
impl From<crate::W<MMMS_ADVCH_NI_VALID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMMS_ADVCH_NI_VALID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_NI_VALID` reader - This bit indicates if the programmed advertisement NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the advertisment event 0 - ADV_NI timer is not valid 1 - ADV_NI timer is valid"]
pub struct ADV_NI_VALID_R(crate::FieldReader<bool, bool>);
impl ADV_NI_VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADV_NI_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_NI_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_NI_VALID` writer - This bit indicates if the programmed advertisement NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the advertisment event 0 - ADV_NI timer is not valid 1 - ADV_NI timer is valid"]
pub struct ADV_NI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_NI_VALID_W<'a> {
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
#[doc = "Field `SCAN_NI_VALID` reader - This bit indicates if the programmed scan NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the scanner event 0 - SCAN_NI timer is not valid 1 - SCAN_NI timer is valid"]
pub struct SCAN_NI_VALID_R(crate::FieldReader<bool, bool>);
impl SCAN_NI_VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_NI_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_NI_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN_NI_VALID` writer - This bit indicates if the programmed scan NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the scanner event 0 - SCAN_NI timer is not valid 1 - SCAN_NI timer is valid"]
pub struct SCAN_NI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> SCAN_NI_VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `INIT_NI_VALID` reader - This bit indicates if the programmed initiator NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the initiator event 0 - INIT_NI timer is not valid 1 - INIT_NI timer is valid"]
pub struct INIT_NI_VALID_R(crate::FieldReader<bool, bool>);
impl INIT_NI_VALID_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_NI_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_NI_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT_NI_VALID` writer - This bit indicates if the programmed initiator NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the initiator event 0 - INIT_NI timer is not valid 1 - INIT_NI timer is valid"]
pub struct INIT_NI_VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_NI_VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit indicates if the programmed advertisement NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the advertisment event 0 - ADV_NI timer is not valid 1 - ADV_NI timer is valid"]
    #[inline(always)]
    pub fn adv_ni_valid(&self) -> ADV_NI_VALID_R {
        ADV_NI_VALID_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit indicates if the programmed scan NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the scanner event 0 - SCAN_NI timer is not valid 1 - SCAN_NI timer is valid"]
    #[inline(always)]
    pub fn scan_ni_valid(&self) -> SCAN_NI_VALID_R {
        SCAN_NI_VALID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit indicates if the programmed initiator NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the initiator event 0 - INIT_NI timer is not valid 1 - INIT_NI timer is valid"]
    #[inline(always)]
    pub fn init_ni_valid(&self) -> INIT_NI_VALID_R {
        INIT_NI_VALID_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit indicates if the programmed advertisement NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the advertisment event 0 - ADV_NI timer is not valid 1 - ADV_NI timer is valid"]
    #[inline(always)]
    pub fn adv_ni_valid(&mut self) -> ADV_NI_VALID_W {
        ADV_NI_VALID_W { w: self }
    }
    #[doc = "Bit 1 - This bit indicates if the programmed scan NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the scanner event 0 - SCAN_NI timer is not valid 1 - SCAN_NI timer is valid"]
    #[inline(always)]
    pub fn scan_ni_valid(&mut self) -> SCAN_NI_VALID_W {
        SCAN_NI_VALID_W { w: self }
    }
    #[doc = "Bit 2 - This bit indicates if the programmed initiator NI_TIMER is valid. FW sets this bit to indicate that the NI_TIMER is programmed. HW clears this bit on servicing the initiator event 0 - INIT_NI timer is not valid 1 - INIT_NI timer is valid"]
    #[inline(always)]
    pub fn init_ni_valid(&mut self) -> INIT_NI_VALID_W {
        INIT_NI_VALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Next instant valid for ADV, SCAN, INIT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmms_advch_ni_valid](index.html) module"]
pub struct MMMS_ADVCH_NI_VALID_SPEC;
impl crate::RegisterSpec for MMMS_ADVCH_NI_VALID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmms_advch_ni_valid::R](R) reader structure"]
impl crate::Readable for MMMS_ADVCH_NI_VALID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmms_advch_ni_valid::W](W) writer structure"]
impl crate::Writable for MMMS_ADVCH_NI_VALID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMMS_ADVCH_NI_VALID to value 0"]
impl crate::Resettable for MMMS_ADVCH_NI_VALID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
