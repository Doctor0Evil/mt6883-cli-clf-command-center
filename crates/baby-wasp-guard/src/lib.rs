//! BabyWasp guard: lightweight biocompatibility sentinels for augmented citizens.
//!
//! Sits above HostBudget, BrainSpecs, BciHostSnapshot, and BioCorridorBundle
//! to keep biocompatibility corridors intact before any OTA or BCI actuation.

use std::time::SystemTime;

use bioscale_upgrade_store::{
    HostBudget,
    UpgradeDescriptor,
    UpgradeDecision,
    ThermodynamicEnvelope,
    MlPassSchedule,
    ReversalConditions,
    EvidenceBundle,
};
use cyberswarm_neurostack::bcihostsnapshot::{
    BciHostSnapshot,
    BciSafetyThresholds,
};
use bioscale_morphism_kernels::{
    BioCorridorBundle,
    BioCompatibilityEnvelope,
};
use bioscale_upgrade_store::evidence::EvidenceTag;

/// 10-sequence evidence bundle for BabyWasp sentinels, anchored to bioscale corridors.
pub const BABY_WASP_EVIDENCE: EvidenceBundle = EvidenceBundle {
    sequences: &[
        EvidenceTag { short_hex: "c3f5a1d7", description: "Cortical corridor metabolic rate bounds for sentinel gating." },
        EvidenceTag { short_hex: "9b2e4f8c", description: "ATP-to-protein mapping used for HostBudget protein checks." },
        EvidenceTag { short_hex: "4d7a2b9e", description: "Safe EEG/BCI duty-cycle envelopes for continuous monitoring." },
        EvidenceTag { short_hex: "1e6c3f4b", description: "Thermoregulatory limits for corridor-level delta T summation." },
        EvidenceTag { short_hex: "7a5d1c3e", description: "Cerebral blood-flow reserve bounds for multi-corridor load." },
        EvidenceTag { short_hex: "2f8b4e9d", description: "Neurovascular coupling constraints for Lyapunov duty descent." },
        EvidenceTag { short_hex: "5c9a3d1f", description: "Lyapunov-stable duty operators for BioMorphKernel schedulers." },
        EvidenceTag { short_hex: "8d4b2e6c", description: "Neuromorphic kernel energy profiles for corridor efficiency." },
        EvidenceTag { short_hex: "3e7f1a5d", description: "Protein turnover half-lives for amortizing morphism cost." },
        EvidenceTag { short_hex: "6b8c4f2e", description: "IL-6/HRV thresholds for automatic rollback triggers." },
    ],
};

/// Compact biocompatibility index snapshot used by BabyWasp.
#[derive(Clone, Debug)]
pub struct BiocompatibilityIndex {
    /// Fraction of daily energy budget used [0, 1].
    pub energy_frac: f64,
    /// Fraction of daily protein budget used [0, 1].
    pub protein_frac: f64,
    /// Max core temperature observed vs envelope (absolute Celsius).
    pub core_temp_c: f32,
    /// Max local temperature across corridors.
    pub max_local_temp_c: f32,
    /// Aggregated BCI duty cycle [0, 1].
    pub bci_duty_frac: f32,
    /// Corridor impact score S_corr in [0, 1].
    pub corridor_impact: f64,
    /// Composite risk from BioKarma/BioAug math [0, 1].
    pub composite_risk: f32,
    /// Timestamp for audit binding.
    pub observed_at: SystemTime,
}

impl BiocompatibilityIndex {
    /// Simple, normalized index that can be logged and graphed.
    pub fn score(&self) -> f64 {
        // Weight energy, thermo, and risk most heavily.
        let e = self.energy_frac;
        let p = self.protein_frac;
        let t = (self.core_temp_c / 38.0_f32).min(1.2_f32) as f64;
        let d = self.bci_duty_frac as f64;
        let s = self.corridor_impact;
        let r = self.composite_risk as f64;

        // Clamp everything to [0,1] before combining.
        let clamp = |x: f64| if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x };

        let e = clamp(e);
        let p = clamp(p);
        let t = clamp(t);
        let d = clamp(d);
        let s = clamp(s);
        let r = clamp(r);

        // Weighted average; tune weights from evidence-backed envelopes.
        0.25 * e + 0.15 * p + 0.20 * t + 0.15 * d + 0.10 * s + 0.15 * r
    }
}

/// Configuration for BabyWasp sentinel thresholds.
#[derive(Clone, Debug)]
pub struct BabyWaspConfig {
    /// Maximum allowed biocompatibility index score before hard deny.
    pub max_index_score: f64,
    /// Maximum allowed single-dimension excursion (energy, thermo, duty, etc.).
    pub max_component_frac: f64,
    /// Evidence bundle anchoring this configuration.
    pub evidence: EvidenceBundle,
}

impl Default for BabyWaspConfig {
    fn default() -> Self {
        BabyWaspConfig {
            max_index_score: 0.75,
            max_component_frac: 0.85,
            evidence: BABY_WASP_EVIDENCE,
        }
    }
}

/// BabyWasp sentinel: watches host + corridor state and gates upgrades.
#[derive(Clone, Debug)]
pub struct BabyWaspSentinel {
    pub config: BabyWaspConfig,
}

impl BabyWaspSentinel {
    pub fn new(config: BabyWaspConfig) -> Self {
        BabyWaspSentinel { config }
    }

    /// Compute a BiocompatibilityIndex from host, BCI, and corridor bundles.
    pub fn compute_index(
        &self,
        host: &HostBudget,
        thermo: &ThermodynamicEnvelope,
        bci_snap: &BciHostSnapshot,
        corridor_bundle: Option<&BioCorridorBundle>,
        composite_risk: f32,
    ) -> BiocompatibilityIndex {
        let energy_frac = host.used_energy_joules / host.daily_energy_joules.max(1.0);
        let protein_frac = host.used_protein_grams / host.daily_protein_grams.max(1.0);
        let core_temp_c = bci_snap.core_temp_c;
        let max_local_temp_c = corridor_bundle
            .and_then(|b| {
                let mut max_t = 0.0_f32;
                for r in &b.regions {
                    if let Some(tloc) = r.delta_t_loc_c {
                        if tloc > max_t {
                            max_t = tloc;
                        }
                    }
                }
                Some(max_t)
            })
            .unwrap_or(bci_snap.local_temp_c);
        let bci_duty_frac = bci_snap.duty_cycle;
        let corridor_impact = corridor_bundle.map(|b| b.scorr).unwrap_or(0.0_f64);

        BiocompatibilityIndex {
            energy_frac,
            protein_frac,
            core_temp_c,
            max_local_temp_c,
            bci_duty_frac,
            corridor_impact,
            composite_risk,
            observed_at: SystemTime::now(),
        }
    }

    /// Hard gate: deny when index or any component exceeds configured limits.
    pub fn gate_upgrade(
        &self,
        host: &HostBudget,
        bcidesc: &UpgradeDescriptor,
        bci_snap: &BciHostSnapshot,
        corridor_env: Option<&BioCompatibilityEnvelope>,
        corridor_bundle: Option<&BioCorridorBundle>,
        composite_risk: f32,
        scheduled_start: SystemTime,
    ) -> UpgradeDecision {
        // First, derive BCI safety thresholds from the upgrade itself.
        let thresholds = BciSafetyThresholds::from_descriptors(
            bcidesc.thermo_envelope,
            bcidesc.ml_schedule,
            bcidesc.reversal,
        );

        // Telemetry pre-gate using existing BCI envelopes.
        if !thresholds.snapshot_safe(bci_snap.clone()) {
            return UpgradeDecision::Denied {
                reason: "BabyWasp: BCI telemetry outside safety envelope".to_string(),
            };
        }

        // Optional corridor compatibility gate.
        if let (Some(env), Some(bundle)) = (corridor_env, corridor_bundle) {
            // Use existing morphism admissibility and thermo checks.
            if !env.host.is_envelope_compatible(env, bundle) {
                return UpgradeDecision::Denied {
                    reason: "BabyWasp: corridor polytope violation".to_string(),
                };
            }
        }

        // Compute biocompatibility index.
        let index = self.compute_index(
            host,
            &bcidesc.thermo_envelope,
            bci_snap,
            corridor_bundle,
            composite_risk,
        );
        let score = index.score();

        // Component-wise guard.
        let cfg = &self.config;
        if score > cfg.max_index_score
            || index.energy_frac > cfg.max_component_frac
            || index.protein_frac > cfg.max_component_frac
            || (index.core_temp_c as f64 / 38.0_f64) > cfg.max_component_frac
            || index.bci_duty_frac as f64 > cfg.max_component_frac
            || index.corridor_impact > cfg.max_component_frac
            || index.composite_risk as f64 > cfg.max_component_frac
        {
            return UpgradeDecision::Denied {
                reason: "BabyWasp: biocompatibility index threshold exceeded".to_string(),
            };
        }

        // If all sentinel checks pass, delegate to standard bioscale evaluation.
        let required_joules: f64 = bcidesc
            .energy_costs
            .iter()
            .map(|e| e.joules)
            .sum();
        if required_joules > host.remaining_energy_joules {
            return UpgradeDecision::Denied {
                reason: "BabyWasp: insufficient HostBudget energy".to_string(),
            };
        }

        // Simple approve; real implementation can reuse existing schedule logic.
        let expected_completion = scheduled_start + bcidesc.ml_schedule.max_continuous_window;
        UpgradeDecision::Approved {
            scheduled_at: scheduled_start,
            expected_completion,
        }
    }
}
