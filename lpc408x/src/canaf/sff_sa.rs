#[doc = "Register `SFF_SA` reader"]
pub struct R(crate::R<SFF_SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFF_SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFF_SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFF_SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFF_SA` writer"]
pub struct W(crate::W<SFF_SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFF_SA_SPEC>;
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
impl From<crate::W<SFF_SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFF_SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFF_SA` reader - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
pub struct SFF_SA_R(crate::FieldReader<u16, u16>);
impl SFF_SA_R {
    pub(crate) fn new(bits: u16) -> Self {
        SFF_SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFF_SA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFF_SA` writer - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
pub struct SFF_SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SFF_SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 2)) | ((value as u32 & 0x01ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:10 - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
    #[inline(always)]
    pub fn sff_sa(&self) -> SFF_SA_R {
        SFF_SA_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:10 - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
    #[inline(always)]
    pub fn sff_sa(&mut self) -> SFF_SA_W {
        SFF_SA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Standard Frame Individual Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sff_sa](index.html) module"]
pub struct SFF_SA_SPEC;
impl crate::RegisterSpec for SFF_SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sff_sa::R](R) reader structure"]
impl crate::Readable for SFF_SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sff_sa::W](W) writer structure"]
impl crate::Writable for SFF_SA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SFF_SA to value 0"]
impl crate::Resettable for SFF_SA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
