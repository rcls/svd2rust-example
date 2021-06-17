#[doc = "Register `RXFILTERCTRL` reader"]
pub struct R(crate::R<RXFILTERCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFILTERCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFILTERCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFILTERCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXFILTERCTRL` writer"]
pub struct W(crate::W<RXFILTERCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXFILTERCTRL_SPEC>;
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
impl From<crate::W<RXFILTERCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXFILTERCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUE` reader - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
pub struct AUE_R(crate::FieldReader<bool, bool>);
impl AUE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUE` writer - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
pub struct AUE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUE_W<'a> {
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
#[doc = "Field `ABE` reader - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
pub struct ABE_R(crate::FieldReader<bool, bool>);
impl ABE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ABE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ABE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABE` writer - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
pub struct ABE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABE_W<'a> {
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
#[doc = "Field `AME` reader - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
pub struct AME_R(crate::FieldReader<bool, bool>);
impl AME_R {
    pub(crate) fn new(bits: bool) -> Self {
        AME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AME` writer - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
pub struct AME_W<'a> {
    w: &'a mut W,
}
impl<'a> AME_W<'a> {
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
#[doc = "Field `AUHE` reader - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
pub struct AUHE_R(crate::FieldReader<bool, bool>);
impl AUHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUHE` writer - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
pub struct AUHE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUHE_W<'a> {
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
#[doc = "Field `AMHE` reader - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
pub struct AMHE_R(crate::FieldReader<bool, bool>);
impl AMHE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMHE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMHE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMHE` writer - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
pub struct AMHE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMHE_W<'a> {
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
#[doc = "Field `APE` reader - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
pub struct APE_R(crate::FieldReader<bool, bool>);
impl APE_R {
    pub(crate) fn new(bits: bool) -> Self {
        APE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APE` writer - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
pub struct APE_W<'a> {
    w: &'a mut W,
}
impl<'a> APE_W<'a> {
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
#[doc = "Field `MPEW` reader - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
pub struct MPEW_R(crate::FieldReader<bool, bool>);
impl MPEW_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPEW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPEW` writer - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
pub struct MPEW_W<'a> {
    w: &'a mut W,
}
impl<'a> MPEW_W<'a> {
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
#[doc = "Field `RFEW` reader - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
pub struct RFEW_R(crate::FieldReader<bool, bool>);
impl RFEW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFEW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFEW` writer - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
pub struct RFEW_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEW_W<'a> {
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
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    pub fn aue(&self) -> AUE_R {
        AUE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    pub fn abe(&self) -> ABE_R {
        ABE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    pub fn ame(&self) -> AME_R {
        AME_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn auhe(&self) -> AUHE_R {
        AUHE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn amhe(&self) -> AMHE_R {
        AMHE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    pub fn ape(&self) -> APE_R {
        APE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn mpew(&self) -> MPEW_R {
        MPEW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn rfew(&self) -> RFEW_R {
        RFEW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AcceptUnicastEn. When set to 1, all unicast frames are accepted."]
    #[inline(always)]
    pub fn aue(&mut self) -> AUE_W {
        AUE_W { w: self }
    }
    #[doc = "Bit 1 - AcceptBroadcastEn. When set to 1, all broadcast frames are accepted."]
    #[inline(always)]
    pub fn abe(&mut self) -> ABE_W {
        ABE_W { w: self }
    }
    #[doc = "Bit 2 - AcceptMulticastEn. When set to 1, all multicast frames are accepted."]
    #[inline(always)]
    pub fn ame(&mut self) -> AME_W {
        AME_W { w: self }
    }
    #[doc = "Bit 3 - AcceptUnicastHashEn. When set to 1, unicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn auhe(&mut self) -> AUHE_W {
        AUHE_W { w: self }
    }
    #[doc = "Bit 4 - AcceptMulticastHashEn. When set to 1, multicast frames that pass the imperfect hash filter are accepted."]
    #[inline(always)]
    pub fn amhe(&mut self) -> AMHE_W {
        AMHE_W { w: self }
    }
    #[doc = "Bit 5 - AcceptPerfectEn. When set to 1, the frames with a destination address identical to the station address are accepted."]
    #[inline(always)]
    pub fn ape(&mut self) -> APE_W {
        APE_W { w: self }
    }
    #[doc = "Bit 12 - MagicPacketEnWoL. When set to 1, the result of the magic packet filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn mpew(&mut self) -> MPEW_W {
        MPEW_W { w: self }
    }
    #[doc = "Bit 13 - RxFilterEnWoL. When set to 1, the result of the perfect address matching filter and the imperfect hash filter will generate a WoL interrupt when there is a match."]
    #[inline(always)]
    pub fn rfew(&mut self) -> RFEW_W {
        RFEW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive filter control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfilterctrl](index.html) module"]
pub struct RXFILTERCTRL_SPEC;
impl crate::RegisterSpec for RXFILTERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfilterctrl::R](R) reader structure"]
impl crate::Readable for RXFILTERCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxfilterctrl::W](W) writer structure"]
impl crate::Writable for RXFILTERCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXFILTERCTRL to value 0"]
impl crate::Resettable for RXFILTERCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
