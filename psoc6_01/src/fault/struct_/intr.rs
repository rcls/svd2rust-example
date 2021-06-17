#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAULT` reader - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to these field to clear the interrupt cause to '0'."]
pub struct FAULT_R(crate::FieldReader<bool, bool>);
impl FAULT_R {
    pub(crate) fn new(bits: bool) -> Self {
        FAULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAULT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FAULT` writer - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to these field to clear the interrupt cause to '0'."]
pub struct FAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> FAULT_W<'a> {
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
    #[doc = "Bit 0 - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to these field to clear the interrupt cause to '0'."]
    #[inline(always)]
    pub fn fault(&self) -> FAULT_R {
        FAULT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This interrupt cause field is activated (HW sets the field to '1') when an enabled (MASK0/MASK1/MASK2) pending fault source is captured: - STATUS.VALID is set to '1'. - STATUS.IDX specifies the fault source index. - DATA0 through DATA3 captures the fault source data. SW writes a '1' to these field to clear the interrupt cause to '0'."]
    #[inline(always)]
    pub fn fault(&mut self) -> FAULT_W {
        FAULT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
