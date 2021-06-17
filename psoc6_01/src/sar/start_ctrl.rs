#[doc = "Register `START_CTRL` reader"]
pub struct R(crate::R<START_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<START_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<START_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<START_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `START_CTRL` writer"]
pub struct W(crate::W<START_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<START_CTRL_SPEC>;
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
impl From<crate::W<START_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<START_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FW_TRIGGER` reader - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
pub struct FW_TRIGGER_R(crate::FieldReader<bool, bool>);
impl FW_TRIGGER_R {
    pub(crate) fn new(bits: bool) -> Self {
        FW_TRIGGER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FW_TRIGGER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FW_TRIGGER` writer - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
pub struct FW_TRIGGER_W<'a> {
    w: &'a mut W,
}
impl<'a> FW_TRIGGER_W<'a> {
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
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub fn fw_trigger(&self) -> FW_TRIGGER_R {
        FW_TRIGGER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When firmware writes a 1 here it will trigger the next scan of enabled channels, hardware clears this bit when the scan started with this trigger is completed. If scanning continuously the trigger is ignored and hardware clears this bit after the next scan is done. This bit is also cleared when the SAR is disabled."]
    #[inline(always)]
    pub fn fw_trigger(&mut self) -> FW_TRIGGER_W {
        FW_TRIGGER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start control register (firmware trigger).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [start_ctrl](index.html) module"]
pub struct START_CTRL_SPEC;
impl crate::RegisterSpec for START_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [start_ctrl::R](R) reader structure"]
impl crate::Readable for START_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [start_ctrl::W](W) writer structure"]
impl crate::Writable for START_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets START_CTRL to value 0"]
impl crate::Resettable for START_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
