//! This module provides an extension trait for the semihonest garbler.
//! The `GarblerExt` trait adds a convenient `garble` method that generates
//! garbled data (including labels for the garbler and evaluator) for a given circuit.
//!
//! It also defines the [`GarbledData`] struct that encapsulates the generated labels.

use crate::{
    circuit::BinaryCircuit, errors::TwopacError, twopac::semihonest::garbler::Garbler, FancyInput,
    FancyReveal,
};
use scuttlebutt::AbstractChannel;

/// A struct representing the garbled data produced.
#[derive(Debug)]
pub struct GarbledData<Wire> {
    /// garber input
    pub garbler_input_labels: Vec<Wire>,
    /// evaluator input
    pub evaluator_input_labels: Vec<Wire>,
    // Additional fields (e.g. the garbled circuit) could be added here.
}

/// An extension trait for Garbler to add a `garble` method.
pub trait GarblerExt {
    /// Wire
    type Wire;
    /// Error
    type Error;

    /// Generates garbled data for the given circuit and garbler inputs.
    fn garble(
        &mut self,
        circuit: &mut BinaryCircuit,
        garbler_binary: Vec<bool>,
    ) -> Result<GarbledData<Self::Wire>, Self::Error>;
}

impl<C, RNG, OT, Wire> GarblerExt for Garbler<C, RNG, OT, Wire>
where
    C: AbstractChannel,
    // Add the SeedableRng bounds with the correct Seed.
    RNG: rand::CryptoRng + rand::Rng + rand::SeedableRng<Seed = scuttlebutt::Block>,
    OT: ocelot::ot::Sender<Msg = scuttlebutt::Block> + scuttlebutt::SemiHonest,
    Wire: crate::wire::WireLabel + Clone,
{
    type Wire = Wire;
    type Error = TwopacError;

    fn garble(
        &mut self,
        _circuit: &mut BinaryCircuit,
        garbler_binary: Vec<bool>,
    ) -> Result<GarbledData<Self::Wire>, Self::Error> {
        // Convert boolean inputs to u16 bits: false -> 0, true -> 1.
        let gb_inputs: Vec<u16> = garbler_binary
            .into_iter()
            .map(|b| if b { 1 } else { 0 })
            .collect();
        // For a bit value, assume the modulus is 2.
        let gb_moduli = vec![2u16; gb_inputs.len()];
        // Encode garbler’s inputs using the FancyInput trait.
        let garbler_labels = self.encode_many(&gb_inputs, &gb_moduli)?;

        // For evaluator inputs the garbler does not know the bits.
        // For demonstration, assume evaluator input count is a fixed value (e.g. 16 bits)
        let evaluator_input_count = 16;
        let evaluator_moduli = vec![2u16; evaluator_input_count];
        let evaluator_labels = self.receive_many(&evaluator_moduli)?;

        Ok(GarbledData {
            garbler_input_labels: garbler_labels,
            evaluator_input_labels: evaluator_labels,
        })
    }
}
