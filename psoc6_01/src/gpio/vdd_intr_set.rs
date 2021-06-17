#[doc = "Register `VDD_INTR_SET` reader"]
pub struct R(crate::R<VDD_INTR_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VDD_INTR_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VDD_INTR_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VDD_INTR_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VDD_INTR_SET` writer"]
pub struct W(crate::W<VDD_INTR_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VDD_INTR_SET_SPEC>;
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
impl From<crate::W<VDD_INTR_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VDD_INTR_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VDDIO_ACTIVE` reader - Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
pub struct VDDIO_ACTIVE_R(crate::FieldReader<u16, u16>);
impl VDDIO_ACTIVE_R {
    pub(crate) fn new(bits: u16) -> Self {
        VDDIO_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDIO_ACTIVE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDIO_ACTIVE` writer - Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
pub struct VDDIO_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDIO_ACTIVE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `VDDA_ACTIVE` reader - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub struct VDDA_ACTIVE_R(crate::FieldReader<bool, bool>);
impl VDDA_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDA_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDA_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDA_ACTIVE` writer - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub struct VDDA_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDA_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `VDDD_ACTIVE` reader - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub struct VDDD_ACTIVE_R(crate::FieldReader<bool, bool>);
impl VDDD_ACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VDDD_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDD_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDD_ACTIVE` writer - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub struct VDDD_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDD_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VDDIO_ACTIVE_R {
        VDDIO_ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VDDA_ACTIVE_R {
        VDDA_ACTIVE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&self) -> VDDD_ACTIVE_R {
        VDDD_ACTIVE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sets supply interrupt. '0': Interrupt state not affected '1': Interrupt set"]
    #[inline(always)]
    pub fn vddio_active(&mut self) -> VDDIO_ACTIVE_W {
        VDDIO_ACTIVE_W { w: self }
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&mut self) -> VDDA_ACTIVE_W {
        VDDA_ACTIVE_W { w: self }
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&mut self) -> VDDD_ACTIVE_W {
        VDDD_ACTIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply detection interrupt set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdd_intr_set](index.html) module"]
pub struct VDD_INTR_SET_SPEC;
impl crate::RegisterSpec for VDD_INTR_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vdd_intr_set::R](R) reader structure"]
impl crate::Readable for VDD_INTR_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vdd_intr_set::W](W) writer structure"]
impl crate::Writable for VDD_INTR_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VDD_INTR_SET to value 0"]
impl crate::Resettable for VDD_INTR_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
