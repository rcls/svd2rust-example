#[doc = "Register `BTR` reader"]
pub struct R(crate::R<BTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BTR` writer"]
pub struct W(crate::W<BTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTR_SPEC>;
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
impl From<crate::W<BTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
pub struct BRP_R(crate::FieldReader<u16, u16>);
impl BRP_R {
    pub(crate) fn new(bits: u16) -> Self {
        BRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRP` writer - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `SJW` reader - The Synchronization Jump Width is (this value plus one) CAN clocks."]
pub struct SJW_R(crate::FieldReader<u8, u8>);
impl SJW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJW` writer - The Synchronization Jump Width is (this value plus one) CAN clocks."]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `TESG1` reader - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
pub struct TESG1_R(crate::FieldReader<u8, u8>);
impl TESG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        TESG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TESG1` writer - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
pub struct TESG1_W<'a> {
    w: &'a mut W,
}
impl<'a> TESG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TESG2` reader - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
pub struct TESG2_R(crate::FieldReader<u8, u8>);
impl TESG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TESG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TESG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TESG2` writer - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
pub struct TESG2_W<'a> {
    w: &'a mut W,
}
impl<'a> TESG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAM_A {
    #[doc = "0: The bus is sampled once (recommended for high speed buses)"]
    THE_BUS_IS_SAMPLED_O = 0,
    #[doc = "1: The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    THE_BUS_IS_SAMPLED_3 = 1,
}
impl From<SAM_A> for bool {
    #[inline(always)]
    fn from(variant: SAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAM` reader - Sampling"]
pub struct SAM_R(crate::FieldReader<bool, SAM_A>);
impl SAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAM_A {
        match self.bits {
            false => SAM_A::THE_BUS_IS_SAMPLED_O,
            true => SAM_A::THE_BUS_IS_SAMPLED_3,
        }
    }
    #[doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_O`"]
    #[inline(always)]
    pub fn is_the_bus_is_sampled_o(&self) -> bool {
        **self == SAM_A::THE_BUS_IS_SAMPLED_O
    }
    #[doc = "Checks if the value of the field is `THE_BUS_IS_SAMPLED_3`"]
    #[inline(always)]
    pub fn is_the_bus_is_sampled_3(&self) -> bool {
        **self == SAM_A::THE_BUS_IS_SAMPLED_3
    }
}
impl core::ops::Deref for SAM_R {
    type Target = crate::FieldReader<bool, SAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAM` writer - Sampling"]
pub struct SAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    #[inline(always)]
    pub fn the_bus_is_sampled_o(self) -> &'a mut W {
        self.variant(SAM_A::THE_BUS_IS_SAMPLED_O)
    }
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    #[inline(always)]
    pub fn the_bus_is_sampled_3(self) -> &'a mut W {
        self.variant(SAM_A::THE_BUS_IS_SAMPLED_3)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&self) -> TESG1_R {
        TESG1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&self) -> TESG2_R {
        TESG2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&mut self) -> TESG1_W {
        TESG1_W { w: self }
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&mut self) -> TESG2_W {
        TESG2_W { w: self }
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&mut self) -> SAM_W {
        SAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [btr](index.html) module"]
pub struct BTR_SPEC;
impl crate::RegisterSpec for BTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [btr::R](R) reader structure"]
impl crate::Readable for BTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [btr::W](W) writer structure"]
impl crate::Writable for BTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BTR to value 0x001c_0000"]
impl crate::Resettable for BTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x001c_0000
    }
}
