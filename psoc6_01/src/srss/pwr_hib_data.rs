#[doc = "Register `PWR_HIB_DATA[%s]` reader"]
pub struct R(crate::R<PWR_HIB_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_HIB_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_HIB_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_HIB_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_HIB_DATA[%s]` writer"]
pub struct W(crate::W<PWR_HIB_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_HIB_DATA_SPEC>;
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
impl From<crate::W<PWR_HIB_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_HIB_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HIB_DATA` reader - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
pub struct HIB_DATA_R(crate::FieldReader<u32, u32>);
impl HIB_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        HIB_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIB_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIB_DATA` writer - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
pub struct HIB_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> HIB_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn hib_data(&self) -> HIB_DATA_R {
        HIB_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional data that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware for any application-specific purpose. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn hib_data(&mut self) -> HIB_DATA_W {
        HIB_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIBERNATE Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_hib_data](index.html) module"]
pub struct PWR_HIB_DATA_SPEC;
impl crate::RegisterSpec for PWR_HIB_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_hib_data::R](R) reader structure"]
impl crate::Readable for PWR_HIB_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_hib_data::W](W) writer structure"]
impl crate::Writable for PWR_HIB_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_HIB_DATA[%s]
to value 0"]
impl crate::Resettable for PWR_HIB_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
