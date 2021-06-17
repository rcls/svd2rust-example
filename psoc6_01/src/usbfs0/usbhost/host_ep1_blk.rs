#[doc = "Register `HOST_EP1_BLK` reader"]
pub struct R(crate::R<HOST_EP1_BLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_EP1_BLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_EP1_BLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_EP1_BLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_EP1_BLK` writer"]
pub struct W(crate::W<HOST_EP1_BLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_EP1_BLK_SPEC>;
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
impl From<crate::W<HOST_EP1_BLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_EP1_BLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLK_NUM` reader - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
pub struct BLK_NUM_R(crate::FieldReader<u16, u16>);
impl BLK_NUM_R {
    pub(crate) fn new(bits: u16) -> Self {
        BLK_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLK_NUM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLK_NUM` writer - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
pub struct BLK_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BLK_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
    #[inline(always)]
    pub fn blk_num(&self) -> BLK_NUM_R {
        BLK_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Set the total byte number for DMA transfer. If HOST_EP1_RW1_DR or HOST_EP1_RW2_DR is written, the block number counter is decremented when DMAE='1'. - Set this bits before DMA transfer is enabled (HOST_DMA_ENBL.DM_DP1DRQE='1')"]
    #[inline(always)]
    pub fn blk_num(&mut self) -> BLK_NUM_W {
        BLK_NUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Endpoint 1 Block Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_ep1_blk](index.html) module"]
pub struct HOST_EP1_BLK_SPEC;
impl crate::RegisterSpec for HOST_EP1_BLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_ep1_blk::R](R) reader structure"]
impl crate::Readable for HOST_EP1_BLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_ep1_blk::W](W) writer structure"]
impl crate::Writable for HOST_EP1_BLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HOST_EP1_BLK to value 0"]
impl crate::Resettable for HOST_EP1_BLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
