#[doc = "Register `TRANSMIT_WINDOW_SIZE` reader"]
pub struct R(crate::R<TRANSMIT_WINDOW_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSMIT_WINDOW_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSMIT_WINDOW_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSMIT_WINDOW_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSMIT_WINDOW_SIZE` writer"]
pub struct W(crate::W<TRANSMIT_WINDOW_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSMIT_WINDOW_SIZE_SPEC>;
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
impl From<crate::W<TRANSMIT_WINDOW_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSMIT_WINDOW_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_WINDOW_SIZE` reader - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
pub struct TX_WINDOW_SIZE_R(crate::FieldReader<u8, u8>);
impl TX_WINDOW_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_WINDOW_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_WINDOW_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_WINDOW_SIZE` writer - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
pub struct TX_WINDOW_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_WINDOW_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size(&self) -> TX_WINDOW_SIZE_R {
        TX_WINDOW_SIZE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - window_size along with the window_offset is used to calculate the first connection point anchor point for the master. This shall be a multiple of 1.25 ms in the range of 1.25 ms to the lesser of 10 ms and (connInterval - 1.25 ms). Values range from 0 to 10 ms."]
    #[inline(always)]
    pub fn tx_window_size(&mut self) -> TX_WINDOW_SIZE_W {
        TX_WINDOW_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit window size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [transmit_window_size](index.html) module"]
pub struct TRANSMIT_WINDOW_SIZE_SPEC;
impl crate::RegisterSpec for TRANSMIT_WINDOW_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [transmit_window_size::R](R) reader structure"]
impl crate::Readable for TRANSMIT_WINDOW_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [transmit_window_size::W](W) writer structure"]
impl crate::Writable for TRANSMIT_WINDOW_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRANSMIT_WINDOW_SIZE to value 0"]
impl crate::Resettable for TRANSMIT_WINDOW_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
