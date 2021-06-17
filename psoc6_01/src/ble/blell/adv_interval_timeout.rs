#[doc = "Register `ADV_INTERVAL_TIMEOUT` reader"]
pub struct R(crate::R<ADV_INTERVAL_TIMEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADV_INTERVAL_TIMEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADV_INTERVAL_TIMEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADV_INTERVAL_TIMEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADV_INTERVAL_TIMEOUT` writer"]
pub struct W(crate::W<ADV_INTERVAL_TIMEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADV_INTERVAL_TIMEOUT_SPEC>;
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
impl From<crate::W<ADV_INTERVAL_TIMEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADV_INTERVAL_TIMEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADV_INTERVAL` reader - Range: 0x0020 to 0x4000 (For ADV_IND) 0x00A0 to 0x4000 (For ADV_SCAN_IND and NONCONN_IND) Invalid for ADV_DIRECT_IND Time = N * 0.625 msec Time Range: 20 ms to 10.24 sec. For directed advertising, firmware programs the default value of 1.28 seconds. In MMMS mode, this register is used as ADV_NI_TIMER when the ADV_NI_VALID is set by firmware"]
pub struct ADV_INTERVAL_R(crate::FieldReader<u16, u16>);
impl ADV_INTERVAL_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADV_INTERVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADV_INTERVAL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADV_INTERVAL` writer - Range: 0x0020 to 0x4000 (For ADV_IND) 0x00A0 to 0x4000 (For ADV_SCAN_IND and NONCONN_IND) Invalid for ADV_DIRECT_IND Time = N * 0.625 msec Time Range: 20 ms to 10.24 sec. For directed advertising, firmware programs the default value of 1.28 seconds. In MMMS mode, this register is used as ADV_NI_TIMER when the ADV_NI_VALID is set by firmware"]
pub struct ADV_INTERVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADV_INTERVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Range: 0x0020 to 0x4000 (For ADV_IND) 0x00A0 to 0x4000 (For ADV_SCAN_IND and NONCONN_IND) Invalid for ADV_DIRECT_IND Time = N * 0.625 msec Time Range: 20 ms to 10.24 sec. For directed advertising, firmware programs the default value of 1.28 seconds. In MMMS mode, this register is used as ADV_NI_TIMER when the ADV_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn adv_interval(&self) -> ADV_INTERVAL_R {
        ADV_INTERVAL_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Range: 0x0020 to 0x4000 (For ADV_IND) 0x00A0 to 0x4000 (For ADV_SCAN_IND and NONCONN_IND) Invalid for ADV_DIRECT_IND Time = N * 0.625 msec Time Range: 20 ms to 10.24 sec. For directed advertising, firmware programs the default value of 1.28 seconds. In MMMS mode, this register is used as ADV_NI_TIMER when the ADV_NI_VALID is set by firmware"]
    #[inline(always)]
    pub fn adv_interval(&mut self) -> ADV_INTERVAL_W {
        ADV_INTERVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Advertising interval register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adv_interval_timeout](index.html) module"]
pub struct ADV_INTERVAL_TIMEOUT_SPEC;
impl crate::RegisterSpec for ADV_INTERVAL_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adv_interval_timeout::R](R) reader structure"]
impl crate::Readable for ADV_INTERVAL_TIMEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adv_interval_timeout::W](W) writer structure"]
impl crate::Writable for ADV_INTERVAL_TIMEOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADV_INTERVAL_TIMEOUT to value 0x20"]
impl crate::Resettable for ADV_INTERVAL_TIMEOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
