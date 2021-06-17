#[doc = "Register `INTENABLE` reader"]
pub struct R(crate::R<INTENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENABLE` writer"]
pub struct W(crate::W<INTENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENABLE_SPEC>;
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
impl From<crate::W<INTENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXOVERRUNINTEN` reader - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
pub struct RXOVERRUNINTEN_R(crate::FieldReader<bool, bool>);
impl RXOVERRUNINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERRUNINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERRUNINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERRUNINTEN` writer - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
pub struct RXOVERRUNINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUNINTEN_W<'a> {
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
#[doc = "Field `RXERRORINTEN` reader - Enable for interrupt trigger on receive errors."]
pub struct RXERRORINTEN_R(crate::FieldReader<bool, bool>);
impl RXERRORINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXERRORINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXERRORINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXERRORINTEN` writer - Enable for interrupt trigger on receive errors."]
pub struct RXERRORINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERRORINTEN_W<'a> {
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
#[doc = "Field `RXFINISHEDINTEN` reader - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub struct RXFINISHEDINTEN_R(crate::FieldReader<bool, bool>);
impl RXFINISHEDINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFINISHEDINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFINISHEDINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFINISHEDINTEN` writer - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub struct RXFINISHEDINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFINISHEDINTEN_W<'a> {
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
#[doc = "Field `RXDONEINTEN` reader - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub struct RXDONEINTEN_R(crate::FieldReader<bool, bool>);
impl RXDONEINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDONEINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDONEINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDONEINTEN` writer - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub struct RXDONEINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDONEINTEN_W<'a> {
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
#[doc = "Field `TXUNDERRUNINTEN` reader - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
pub struct TXUNDERRUNINTEN_R(crate::FieldReader<bool, bool>);
impl TXUNDERRUNINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERRUNINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERRUNINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERRUNINTEN` writer - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
pub struct TXUNDERRUNINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERRUNINTEN_W<'a> {
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
#[doc = "Field `TXERRORINTEN` reader - Enable for interrupt trigger on transmit errors."]
pub struct TXERRORINTEN_R(crate::FieldReader<bool, bool>);
impl TXERRORINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXERRORINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXERRORINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXERRORINTEN` writer - Enable for interrupt trigger on transmit errors."]
pub struct TXERRORINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXERRORINTEN_W<'a> {
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
#[doc = "Field `TXFINISHEDINTEN` reader - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub struct TXFINISHEDINTEN_R(crate::FieldReader<bool, bool>);
impl TXFINISHEDINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFINISHEDINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFINISHEDINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFINISHEDINTEN` writer - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub struct TXFINISHEDINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFINISHEDINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TXDONEINTEN` reader - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub struct TXDONEINTEN_R(crate::FieldReader<bool, bool>);
impl TXDONEINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDONEINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDONEINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDONEINTEN` writer - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub struct TXDONEINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDONEINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SOFTINTEN` reader - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub struct SOFTINTEN_R(crate::FieldReader<bool, bool>);
impl SOFTINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTINTEN` writer - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub struct SOFTINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `WAKEUPINTEN` reader - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
pub struct WAKEUPINTEN_R(crate::FieldReader<bool, bool>);
impl WAKEUPINTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAKEUPINTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAKEUPINTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAKEUPINTEN` writer - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
pub struct WAKEUPINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    pub fn rxoverruninten(&self) -> RXOVERRUNINTEN_R {
        RXOVERRUNINTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    pub fn rxerrorinten(&self) -> RXERRORINTEN_R {
        RXERRORINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedinten(&self) -> RXFINISHEDINTEN_R {
        RXFINISHEDINTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneinten(&self) -> RXDONEINTEN_R {
        RXDONEINTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    pub fn txunderruninten(&self) -> TXUNDERRUNINTEN_R {
        TXUNDERRUNINTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    pub fn txerrorinten(&self) -> TXERRORINTEN_R {
        TXERRORINTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedinten(&self) -> TXFINISHEDINTEN_R {
        TXFINISHEDINTEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneinten(&self) -> TXDONEINTEN_R {
        TXDONEINTEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softinten(&self) -> SOFTINTEN_R {
        SOFTINTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupinten(&self) -> WAKEUPINTEN_R {
        WAKEUPINTEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable for interrupt trigger on receive buffer overrun or descriptor underrun situations."]
    #[inline(always)]
    pub fn rxoverruninten(&mut self) -> RXOVERRUNINTEN_W {
        RXOVERRUNINTEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable for interrupt trigger on receive errors."]
    #[inline(always)]
    pub fn rxerrorinten(&mut self) -> RXERRORINTEN_W {
        RXERRORINTEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable for interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedinten(&mut self) -> RXFINISHEDINTEN_W {
        RXFINISHEDINTEN_W { w: self }
    }
    #[doc = "Bit 3 - Enable for interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneinten(&mut self) -> RXDONEINTEN_W {
        RXDONEINTEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable for interrupt trigger on transmit buffer or descriptor underrun situations."]
    #[inline(always)]
    pub fn txunderruninten(&mut self) -> TXUNDERRUNINTEN_W {
        TXUNDERRUNINTEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable for interrupt trigger on transmit errors."]
    #[inline(always)]
    pub fn txerrorinten(&mut self) -> TXERRORINTEN_W {
        TXERRORINTEN_W { w: self }
    }
    #[doc = "Bit 6 - Enable for interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedinten(&mut self) -> TXFINISHEDINTEN_W {
        TXFINISHEDINTEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable for interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneinten(&mut self) -> TXDONEINTEN_W {
        TXDONEINTEN_W { w: self }
    }
    #[doc = "Bit 12 - Enable for interrupt triggered by the SoftInt bit in the IntStatus register, caused by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softinten(&mut self) -> SOFTINTEN_W {
        SOFTINTEN_W { w: self }
    }
    #[doc = "Bit 13 - Enable for interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupinten(&mut self) -> WAKEUPINTEN_W {
        WAKEUPINTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenable](index.html) module"]
pub struct INTENABLE_SPEC;
impl crate::RegisterSpec for INTENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenable::R](R) reader structure"]
impl crate::Readable for INTENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenable::W](W) writer structure"]
impl crate::Writable for INTENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENABLE to value 0"]
impl crate::Resettable for INTENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
