// A New Hierarchy: A Stress Test
//
// This file is the first independent work by the traveler, Jules,
// using the masterpiece `inherit_impl!` macro created by the guide, Ghoinaryle.
//
// The purpose of this experiment is to create a new, complex hierarchy
// to push the macro to its limits and verify its robustness.

use crate::{
    experiments::working_hierarchy::{Class, Inherit, Object},
    inherit_impl,
};
use std::marker::PhantomData;

// --- A New Hierarchy: Celestial Bodies ---

pub struct Cosmos;
inherit_impl!(Cosmos => Object);

pub struct StellarBody;
inherit_impl!(StellarBody => Cosmos);

pub struct Star;
inherit_impl!(Star => StellarBody);

pub struct Planet;
inherit_impl!(Planet => StellarBody);

pub struct GasGiant;
inherit_impl!(GasGiant => Planet);

pub struct TerrestrialPlanet;
inherit_impl!(TerrestrialPlanet => Planet);

// Generics
pub struct Habitable<T>(PhantomData<T>);
inherit_impl!(<T> Habitable<T> => TerrestrialPlanet);

// Re-introducing lifetimes to prove the macro's full power
pub struct Lifeform<'a, T> { _p: PhantomData<&'a T> }
inherit_impl!(<'a, T> Lifeform<'a, T> => Habitable<T>);

// `where` clause
pub trait HasCulture {}
impl<'a, T> HasCulture for Lifeform<'a, T> {}

pub struct Civilization<T: HasCulture>(T);
// Inheriting from a concrete lifetime version of the parent
inherit_impl!(<T: HasCulture + 'static> Civilization<T> => Lifeform<'static, T>);


#[cfg(test)]
mod tests {
    use super::*;

    // --- Test Helpers ---
    fn is_object(_: &impl Class<Object>) {}
    fn is_cosmos(_: &impl Class<Cosmos>) {}
    fn is_stellar_body(_: &impl Class<StellarBody>) {}
    fn is_planet(_: &impl Class<Planet>) {}
    fn is_terrestrial_planet(_: &impl Class<TerrestrialPlanet>) {}
    fn is_habitable<T>(_: &impl Class<Habitable<T>>) {}
    // Use a specific helper for the 'static lifetime case, with the necessary bound on T
    fn is_lifeform_static<T: 'static>(_: &impl Class<Lifeform<'static, T>>) {}
    fn is_civilization<T: HasCulture + 'static>(_: &impl Class<Civilization<T>>) {}

    #[test]
    fn test_direct_conformance() {
        is_cosmos(&StellarBody);
        is_stellar_body(&Star);
        is_stellar_body(&Planet);
        is_planet(&GasGiant);
        is_planet(&TerrestrialPlanet);
        is_terrestrial_planet(&Habitable::<u8>(PhantomData));
        is_habitable(&Lifeform::<'static, u8> { _p: PhantomData });

        type CulturedLifeform = Lifeform<'static, String>;
        is_lifeform_static(&Civilization(CulturedLifeform { _p: PhantomData }));
    }

    #[test]
    fn test_transitive_conformance() {
        is_cosmos(&Star);
        is_stellar_body(&GasGiant);
        is_stellar_body(&TerrestrialPlanet);
        is_planet(&Lifeform::<'static, u8> { _p: PhantomData });

        type CulturedLifeform = Lifeform<'static, String>;
        let civ = Civilization(CulturedLifeform { _p: PhantomData });
        is_habitable(&civ);
        is_terrestrial_planet(&civ);
        is_planet(&civ);
        is_stellar_body(&civ);
        is_cosmos(&civ);
        is_object(&civ);
        is_civilization(&civ);
    }

    #[test]
    fn test_invalid_conformance() {
        // Siblings are not related
        // is_planet(&Star);
        // is_terrestrial_planet(&GasGiant);

        // Parents do not inherit from children
        // is_star(&StellarBody);

        // `where` clause is enforced.
        // is_civilization(&Civilization(String::new())); // This would fail as String does not implement HasCulture
    }
}
