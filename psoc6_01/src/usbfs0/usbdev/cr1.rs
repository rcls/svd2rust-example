#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_ENABLE` reader - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
pub struct REG_ENABLE_R(crate::FieldReader<bool, bool>);
impl REG_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_ENABLE` writer - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
pub struct REG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_ENABLE_W<'a> {
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
#[doc = "Field `ENABLE_LOCK` reader - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
pub struct ENABLE_LOCK_R(crate::FieldReader<bool, bool>);
impl ENABLE_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_LOCK` writer - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
pub struct ENABLE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_LOCK_W<'a> {
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
#[doc = "Field `BUS_ACTIVITY` reader - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
pub struct BUS_ACTIVITY_R(crate::FieldReader<bool, bool>);
impl BUS_ACTIVITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUS_ACTIVITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_ACTIVITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUS_ACTIVITY` writer - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
pub struct BUS_ACTIVITY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ACTIVITY_W<'a> {
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
#[doc = "Field `RSVD_3` reader - N/A"]
pub struct RSVD_3_R(crate::FieldReader<bool, bool>);
impl RSVD_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVD_3` writer - N/A"]
pub struct RSVD_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
    #[inline(always)]
    pub fn reg_enable(&self) -> REG_ENABLE_R {
        REG_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
    #[inline(always)]
    pub fn enable_lock(&self) -> ENABLE_LOCK_R {
        ENABLE_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
    #[inline(always)]
    pub fn bus_activity(&self) -> BUS_ACTIVITY_R {
        BUS_ACTIVITY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rsvd_3(&self) -> RSVD_3_R {
        RSVD_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls the operation of the internal USB regulator. For applications with supply voltages in the 5V range this bit is set high to enable the internal regulator. For device supply voltage in the 3.3V range this bit is cleared to connect the transceiver directly to the supply."]
    #[inline(always)]
    pub fn reg_enable(&mut self) -> REG_ENABLE_W {
        REG_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - This bit is set to turn on the automatic frequency locking of the internal oscillator to USB traffic. Unless an external clock is being provided this bit should remain set for proper USB operation."]
    #[inline(always)]
    pub fn enable_lock(&mut self) -> ENABLE_LOCK_W {
        ENABLE_LOCK_W { w: self }
    }
    #[doc = "Bit 2 - The Bus Activity bit is a stickybit that detects any non-idle USB event that has occurred on the USB bus. Once set to High by the SIE to indicate the bus activity this bit retains its logical High value until firmware clears it."]
    #[inline(always)]
    pub fn bus_activity(&mut self) -> BUS_ACTIVITY_W {
        BUS_ACTIVITY_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn rsvd_3(&mut self) -> RSVD_3_W {
        RSVD_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
