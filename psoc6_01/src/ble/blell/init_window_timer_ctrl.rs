#[doc = "Register `INIT_WINDOW_TIMER_CTRL` reader"]
pub struct R(crate::R<INIT_WINDOW_TIMER_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_WINDOW_TIMER_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_WINDOW_TIMER_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_WINDOW_TIMER_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INIT_WINDOW_TIMER_CTRL` writer"]
pub struct W(crate::W<INIT_WINDOW_TIMER_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_WINDOW_TIMER_CTRL_SPEC>;
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
impl From<crate::W<INIT_WINDOW_TIMER_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_WINDOW_TIMER_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT_WINDOW_OFFSET_SEL` reader - Controls the INIT Window offset source 1 - Pick INIT Window Offset from HW calculated INIT_WINDOW_OFFSET 0 - Pick INIT Window Offset from FW loaded register"]
pub struct INIT_WINDOW_OFFSET_SEL_R(crate::FieldReader<bool, bool>);
impl INIT_WINDOW_OFFSET_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        INIT_WINDOW_OFFSET_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INIT_WINDOW_OFFSET_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INIT_WINDOW_OFFSET_SEL` writer - Controls the INIT Window offset source 1 - Pick INIT Window Offset from HW calculated INIT_WINDOW_OFFSET 0 - Pick INIT Window Offset from FW loaded register"]
pub struct INIT_WINDOW_OFFSET_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INIT_WINDOW_OFFSET_SEL_W<'a> {
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
    #[doc = "Bit 0 - Controls the INIT Window offset source 1 - Pick INIT Window Offset from HW calculated INIT_WINDOW_OFFSET 0 - Pick INIT Window Offset from FW loaded register"]
    #[inline(always)]
    pub fn init_window_offset_sel(&self) -> INIT_WINDOW_OFFSET_SEL_R {
        INIT_WINDOW_OFFSET_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the INIT Window offset source 1 - Pick INIT Window Offset from HW calculated INIT_WINDOW_OFFSET 0 - Pick INIT Window Offset from FW loaded register"]
    #[inline(always)]
    pub fn init_window_offset_sel(&mut self) -> INIT_WINDOW_OFFSET_SEL_W {
        INIT_WINDOW_OFFSET_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Initiator Window NI timer control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init_window_timer_ctrl](index.html) module"]
pub struct INIT_WINDOW_TIMER_CTRL_SPEC;
impl crate::RegisterSpec for INIT_WINDOW_TIMER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [init_window_timer_ctrl::R](R) reader structure"]
impl crate::Readable for INIT_WINDOW_TIMER_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [init_window_timer_ctrl::W](W) writer structure"]
impl crate::Writable for INIT_WINDOW_TIMER_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INIT_WINDOW_TIMER_CTRL to value 0"]
impl crate::Resettable for INIT_WINDOW_TIMER_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
