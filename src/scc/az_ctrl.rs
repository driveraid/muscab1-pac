#[doc = "Register `AZ_CTRL` reader"]
pub type R = crate::R<AzCtrlSpec>;
#[doc = "Register `AZ_CTRL` writer"]
pub type W = crate::W<AzCtrlSpec>;
#[doc = "Field `AZ_BOOT_REMAP` reader - Alcatraz remap at boot"]
pub type AzBootRemapR = crate::BitReader;
#[doc = "Field `AZ_BOOT_REMAP` writer - Alcatraz remap at boot"]
pub type AzBootRemapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPUWAIT` reader - Alcatraz CPU wait at boot:"]
pub type CpuwaitR = crate::BitReader;
#[doc = "Field `CPUWAIT` writer - Alcatraz CPU wait at boot:"]
pub type CpuwaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMOVE_CHACHA_ENGINE` reader - Alcatraz CryptoCell remove CHACHA engine"]
pub type RemoveChachaEngineR = crate::BitReader;
#[doc = "Field `REMOVE_CHACHA_ENGINE` writer - Alcatraz CryptoCell remove CHACHA engine"]
pub type RemoveChachaEngineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REMOVE_GHASH_ENGINE` reader - Alcatraz CryptoCell remove Ghash engine"]
pub type RemoveGhashEngineR = crate::BitReader;
#[doc = "Field `REMOVE_GHASH_ENGINE` writer - Alcatraz CryptoCell remove Ghash engine"]
pub type RemoveGhashEngineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEC_ISO_ENB` reader - Alcatraz CryptoCell Secure Frame Isolation enable"]
pub type ChsecIsoEnbR = crate::BitReader;
#[doc = "Field `CHSEC_ISO_ENB` writer - Alcatraz CryptoCell Secure Frame Isolation enable"]
pub type ChsecIsoEnbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSEC_MISC_7` reader - Alcatraz CryptoCell secure Secure Frame control"]
pub type ChsecMisc7R = crate::BitReader;
#[doc = "Field `CHSEC_MISC_7` writer - Alcatraz CryptoCell secure Secure Frame control"]
pub type ChsecMisc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGRESETn` reader - Alcatraz reset DBGRESETn"]
pub type DbgresetnR = crate::BitReader;
#[doc = "Field `DBGRESETn` writer - Alcatraz reset DBGRESETn"]
pub type DbgresetnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETn` reader - Alcatraz reset HRESETn"]
pub type HresetnR = crate::BitReader;
#[doc = "Field `HRESETn` writer - Alcatraz reset HRESETn"]
pub type HresetnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCC_nPORESETAON_nPORESET_SEL` reader - Alcatraz reset control"]
pub type SccNPoresetaonNPoresetSelR = crate::BitReader;
#[doc = "Field `SCC_nPORESETAON_nPORESET_SEL` writer - Alcatraz reset control"]
pub type SccNPoresetaonNPoresetSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCC_PSI_FEATURE_EN` reader - Value of SCC_PSI_FEATURE_EN from SCC"]
pub type SccPsiFeatureEnR = crate::BitReader;
#[doc = "Field `SCC_PSI_FEATURE_EN` writer - Value of SCC_PSI_FEATURE_EN from SCC"]
pub type SccPsiFeatureEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCC_PSI_FEATURE_EN_SEL` reader - Select PSI_FEATURE_EN source"]
pub type SccPsiFeatureEnSelR = crate::BitReader;
#[doc = "Field `SCC_PSI_FEATURE_EN_SEL` writer - Select PSI_FEATURE_EN source"]
pub type SccPsiFeatureEnSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Alcatraz remap at boot"]
    #[inline(always)]
    pub fn az_boot_remap(&self) -> AzBootRemapR {
        AzBootRemapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Alcatraz CPU wait at boot:"]
    #[inline(always)]
    pub fn cpuwait(&self) -> CpuwaitR {
        CpuwaitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alcatraz CryptoCell remove CHACHA engine"]
    #[inline(always)]
    pub fn remove_chacha_engine(&self) -> RemoveChachaEngineR {
        RemoveChachaEngineR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alcatraz CryptoCell remove Ghash engine"]
    #[inline(always)]
    pub fn remove_ghash_engine(&self) -> RemoveGhashEngineR {
        RemoveGhashEngineR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Alcatraz CryptoCell Secure Frame Isolation enable"]
    #[inline(always)]
    pub fn chsec_iso_enb(&self) -> ChsecIsoEnbR {
        ChsecIsoEnbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Alcatraz CryptoCell secure Secure Frame control"]
    #[inline(always)]
    pub fn chsec_misc_7(&self) -> ChsecMisc7R {
        ChsecMisc7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Alcatraz reset DBGRESETn"]
    #[inline(always)]
    pub fn dbgresetn(&self) -> DbgresetnR {
        DbgresetnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Alcatraz reset HRESETn"]
    #[inline(always)]
    pub fn hresetn(&self) -> HresetnR {
        HresetnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alcatraz reset control"]
    #[inline(always)]
    pub fn scc_n_poresetaon_n_poreset_sel(&self) -> SccNPoresetaonNPoresetSelR {
        SccNPoresetaonNPoresetSelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Value of SCC_PSI_FEATURE_EN from SCC"]
    #[inline(always)]
    pub fn scc_psi_feature_en(&self) -> SccPsiFeatureEnR {
        SccPsiFeatureEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Select PSI_FEATURE_EN source"]
    #[inline(always)]
    pub fn scc_psi_feature_en_sel(&self) -> SccPsiFeatureEnSelR {
        SccPsiFeatureEnSelR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alcatraz remap at boot"]
    #[inline(always)]
    pub fn az_boot_remap(&mut self) -> AzBootRemapW<AzCtrlSpec> {
        AzBootRemapW::new(self, 0)
    }
    #[doc = "Bit 1 - Alcatraz CPU wait at boot:"]
    #[inline(always)]
    pub fn cpuwait(&mut self) -> CpuwaitW<AzCtrlSpec> {
        CpuwaitW::new(self, 1)
    }
    #[doc = "Bit 2 - Alcatraz CryptoCell remove CHACHA engine"]
    #[inline(always)]
    pub fn remove_chacha_engine(&mut self) -> RemoveChachaEngineW<AzCtrlSpec> {
        RemoveChachaEngineW::new(self, 2)
    }
    #[doc = "Bit 3 - Alcatraz CryptoCell remove Ghash engine"]
    #[inline(always)]
    pub fn remove_ghash_engine(&mut self) -> RemoveGhashEngineW<AzCtrlSpec> {
        RemoveGhashEngineW::new(self, 3)
    }
    #[doc = "Bit 4 - Alcatraz CryptoCell Secure Frame Isolation enable"]
    #[inline(always)]
    pub fn chsec_iso_enb(&mut self) -> ChsecIsoEnbW<AzCtrlSpec> {
        ChsecIsoEnbW::new(self, 4)
    }
    #[doc = "Bit 5 - Alcatraz CryptoCell secure Secure Frame control"]
    #[inline(always)]
    pub fn chsec_misc_7(&mut self) -> ChsecMisc7W<AzCtrlSpec> {
        ChsecMisc7W::new(self, 5)
    }
    #[doc = "Bit 7 - Alcatraz reset DBGRESETn"]
    #[inline(always)]
    pub fn dbgresetn(&mut self) -> DbgresetnW<AzCtrlSpec> {
        DbgresetnW::new(self, 7)
    }
    #[doc = "Bit 8 - Alcatraz reset HRESETn"]
    #[inline(always)]
    pub fn hresetn(&mut self) -> HresetnW<AzCtrlSpec> {
        HresetnW::new(self, 8)
    }
    #[doc = "Bit 9 - Alcatraz reset control"]
    #[inline(always)]
    pub fn scc_n_poresetaon_n_poreset_sel(&mut self) -> SccNPoresetaonNPoresetSelW<AzCtrlSpec> {
        SccNPoresetaonNPoresetSelW::new(self, 9)
    }
    #[doc = "Bit 10 - Value of SCC_PSI_FEATURE_EN from SCC"]
    #[inline(always)]
    pub fn scc_psi_feature_en(&mut self) -> SccPsiFeatureEnW<AzCtrlSpec> {
        SccPsiFeatureEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Select PSI_FEATURE_EN source"]
    #[inline(always)]
    pub fn scc_psi_feature_en_sel(&mut self) -> SccPsiFeatureEnSelW<AzCtrlSpec> {
        SccPsiFeatureEnSelW::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`az_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`az_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AzCtrlSpec;
impl crate::RegisterSpec for AzCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`az_ctrl::R`](R) reader structure"]
impl crate::Readable for AzCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`az_ctrl::W`](W) writer structure"]
impl crate::Writable for AzCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AZ_CTRL to value 0x0600"]
impl crate::Resettable for AzCtrlSpec {
    const RESET_VALUE: u32 = 0x0600;
}
