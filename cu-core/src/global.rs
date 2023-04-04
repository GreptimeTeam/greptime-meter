use once_cell::sync::OnceCell;

use crate::registry::Registry;

pub fn global_registry() -> Registry {
    static GLOBAL_REGISTRY: OnceCell<Registry> = OnceCell::new();

    GLOBAL_REGISTRY.get_or_init(Registry::default).clone()
}
