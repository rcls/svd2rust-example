#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWMMR0INT` reader - Interrupt flag for PWM match channel 0."]
pub struct PWMMR0INT_R(crate::FieldReader<bool, bool>);
impl PWMMR0INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMMR0INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMMR0INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMMR0INT` writer - Interrupt flag for PWM match channel 0."]
pub struct PWMMR0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR0INT_W<'a> {
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
#[doc = "Field `PWMMR1INT` reader - Interrupt flag for PWM match channel 1."]
pub struct PWMMR1INT_R(crate::FieldReader<bool, bool>);
impl PWMMR1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMMR1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMMR1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMMR1INT` writer - Interrupt flag for PWM match channel 1."]
pub struct PWMMR1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR1INT_W<'a> {
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
#[doc = "Field `PWMMR2INT` reader - Interrupt flag for PWM match channel 2."]
pub struct PWMMR2INT_R(crate::FieldReader<bool, bool>);
impl PWMMR2INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMMR2INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMMR2INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMMR2INT` writer - Interrupt flag for PWM match channel 2."]
pub struct PWMMR2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR2INT_W<'a> {
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
#[doc = "Field `PWMMR3INT` reader - Interrupt flag for PWM match channel 3."]
pub struct PWMMR3INT_R(crate::FieldReader<bool, bool>);
impl PWMMR3INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMMR3INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMMR3INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMMR3INT` writer - Interrupt flag for PWM match channel 3."]
pub struct PWMMR3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR3INT_W<'a> {
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
#[doc = "Field `PWMCAP0INT` reader - Interrupt flag for capture input 0"]
pub struct PWMCAP0INT_R(crate::FieldReader<bool, bool>);
impl PWMCAP0INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCAP0INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCAP0INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMCAP0INT` writer - Interrupt flag for capture input 0"]
pub struct PWMCAP0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCAP0INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PWMCAP1INT` reader - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
pub struct PWMCAP1INT_R(crate::FieldReader<bool, bool>);
impl PWMCAP1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMCAP1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMCAP1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMCAP1INT` writer - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
pub struct PWMCAP1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMCAP1INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PWMMR4INT` reader - Interrupt flag for PWM match channel 4."]
pub struct PWMMR4INT_R(crate::FieldReader<bool, bool>);
impl PWMMR4INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMMR4INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMMR4INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMMR4INT` writer - Interrupt flag for PWM match channel 4."]
pub struct PWMMR4INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR4INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PWMMR5INT` reader - Interrupt flag for PWM match channel 5."]
pub struct PWMMR5INT_R(crate::FieldReader<bool, bool>);
impl PWMMR5INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMMR5INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMMR5INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMMR5INT` writer - Interrupt flag for PWM match channel 5."]
pub struct PWMMR5INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR5INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PWMMR6INT` reader - Interrupt flag for PWM match channel 6."]
pub struct PWMMR6INT_R(crate::FieldReader<bool, bool>);
impl PWMMR6INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWMMR6INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWMMR6INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWMMR6INT` writer - Interrupt flag for PWM match channel 6."]
pub struct PWMMR6INT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMMR6INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    pub fn pwmmr0int(&self) -> PWMMR0INT_R {
        PWMMR0INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    pub fn pwmmr1int(&self) -> PWMMR1INT_R {
        PWMMR1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    pub fn pwmmr2int(&self) -> PWMMR2INT_R {
        PWMMR2INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    pub fn pwmmr3int(&self) -> PWMMR3INT_R {
        PWMMR3INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    pub fn pwmcap0int(&self) -> PWMCAP0INT_R {
        PWMCAP0INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    pub fn pwmcap1int(&self) -> PWMCAP1INT_R {
        PWMCAP1INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    pub fn pwmmr4int(&self) -> PWMMR4INT_R {
        PWMMR4INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    pub fn pwmmr5int(&self) -> PWMMR5INT_R {
        PWMMR5INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    pub fn pwmmr6int(&self) -> PWMMR6INT_R {
        PWMMR6INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for PWM match channel 0."]
    #[inline(always)]
    pub fn pwmmr0int(&mut self) -> PWMMR0INT_W {
        PWMMR0INT_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt flag for PWM match channel 1."]
    #[inline(always)]
    pub fn pwmmr1int(&mut self) -> PWMMR1INT_W {
        PWMMR1INT_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt flag for PWM match channel 2."]
    #[inline(always)]
    pub fn pwmmr2int(&mut self) -> PWMMR2INT_W {
        PWMMR2INT_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag for PWM match channel 3."]
    #[inline(always)]
    pub fn pwmmr3int(&mut self) -> PWMMR3INT_W {
        PWMMR3INT_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt flag for capture input 0"]
    #[inline(always)]
    pub fn pwmcap0int(&mut self) -> PWMCAP0INT_W {
        PWMCAP0INT_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt flag for capture input 1 (available in PWM1IR only; this bit is reserved in PWM0IR)."]
    #[inline(always)]
    pub fn pwmcap1int(&mut self) -> PWMCAP1INT_W {
        PWMCAP1INT_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt flag for PWM match channel 4."]
    #[inline(always)]
    pub fn pwmmr4int(&mut self) -> PWMMR4INT_W {
        PWMMR4INT_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt flag for PWM match channel 5."]
    #[inline(always)]
    pub fn pwmmr5int(&mut self) -> PWMMR5INT_W {
        PWMMR5INT_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt flag for PWM match channel 6."]
    #[inline(always)]
    pub fn pwmmr6int(&mut self) -> PWMMR6INT_W {
        PWMMR6INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
