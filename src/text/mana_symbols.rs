// This file has been deprecated and moved to src/mana/render modules
// Please use the new module structure in src/mana/render instead
// This file is kept temporarily to avoid import errors during transition
// and will be removed in a future cleanup

// Re-export from the new module structure
pub use crate::mana::render::styles::ManaSymbolOptions;
pub use crate::mana::render::styles::get_mana_symbol_width;
pub use crate::mana::render::systems::render_mana_symbol;
