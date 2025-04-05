use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use super::color::*;
use super::cost::Mana;

/// A pool of mana that can be used to cast spells.
///
/// This tracks both the amount and type of mana available to a player.
///
#[derive(Default, Debug, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ManaPool {
    /// Map of colors to mana amounts - we need to use a wrapper here since ManaColor can't be a HashMap key with Reflect
    #[reflect(ignore)]
    pub mana: HashMap<ManaColor, Mana>,
    /// Reflectable version of the mana map using u32 keys (the bits of ManaColor)
    pub reflectable_mana: Vec<(ReflectableColor, Mana)>,
}

impl ManaPool {
    /// Creates a new empty mana pool.
    ///
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add mana to the mana pool.
    ///
    #[allow(dead_code)]
    pub fn add(&mut self, mana: Mana) {
        if mana.is_empty() {
            return;
        }

        // Update or add mana of the given color
        let color = mana.color;
        let entry = self.mana.entry(color).or_insert_with(|| {
            // Directly initialize Mana with the correct color fields
            Mana {
                color,
                reflectable_color: color.into(),
                ..Default::default() // Use struct update syntax with default for the rest
            }
        });

        // Add mana of each color
        entry.white += mana.white;
        entry.blue += mana.blue;
        entry.black += mana.black;
        entry.red += mana.red;
        entry.green += mana.green;
        entry.colorless += mana.colorless;

        // Update reflectable version
        self.sync_reflectable_mana();
    }

    /// Remove mana from the mana pool. Returns true if successful, false if not enough mana.
    ///
    #[allow(dead_code)]
    pub fn remove(&mut self, mana: Mana) -> bool {
        if mana.is_empty() {
            return true;
        }

        // Check if we can pay with current mana
        if !mana.can_pay(self) {
            return false;
        }

        // Implementation that removes mana properly is complex
        // For simplicity, we'll just subtract from any source, prioritizing color matches

        // Remove colored mana first (must come from matching sources)
        // TODO: Implement proper mana payment choice algorithm
        let mut remaining_white = mana.white;
        let mut remaining_blue = mana.blue;
        let mut remaining_black = mana.black;
        let mut remaining_red = mana.red;
        let mut remaining_green = mana.green;
        let mut remaining_colorless = mana.colorless;

        // Process each mana source (for simplicity, we're not providing choice)
        for (_, source) in self.mana.iter_mut() {
            // Remove colored mana from matching sources
            let white_to_remove = remaining_white.min(source.white);
            source.white -= white_to_remove;
            remaining_white -= white_to_remove;

            let blue_to_remove = remaining_blue.min(source.blue);
            source.blue -= blue_to_remove;
            remaining_blue -= blue_to_remove;

            let black_to_remove = remaining_black.min(source.black);
            source.black -= black_to_remove;
            remaining_black -= black_to_remove;

            let red_to_remove = remaining_red.min(source.red);
            source.red -= red_to_remove;
            remaining_red -= red_to_remove;

            let green_to_remove = remaining_green.min(source.green);
            source.green -= green_to_remove;
            remaining_green -= green_to_remove;

            // Use whatever is left for colorless costs
            let colorless_to_remove = remaining_colorless.min(source.colorless);
            source.colorless -= colorless_to_remove;
            remaining_colorless -= colorless_to_remove;
        }

        // Update reflectable version
        self.sync_reflectable_mana();
        true
    }

    /// Clear the mana pool of all mana.
    ///
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.mana.clear();
        self.reflectable_mana.clear();
    }

    /// Synchronize the reflectable mana vector with the mana HashMap
    fn sync_reflectable_mana(&mut self) {
        self.reflectable_mana.clear();
        for (&color, &mana) in &self.mana {
            self.reflectable_mana
                .push((ReflectableColor::from(color), mana));
        }
    }

    /// Rebuild the mana HashMap from the reflectable mana vector
    /// This is useful when loading a saved game where only the reflectable data was serialized
    #[allow(dead_code)]
    pub fn rebuild_from_reflectable(&mut self) {
        self.mana.clear();
        for (reflectable_color, mana) in &self.reflectable_mana {
            self.mana.insert((*reflectable_color).into(), *mana);
        }
    }
}
