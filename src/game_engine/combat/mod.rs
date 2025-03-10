mod combat;
mod test_utils;

pub use combat::{
    AssignCombatDamageEvent, AttackerDeclaredEvent, BlockRestriction, BlockedStatus,
    BlockerDeclaredEvent, CombatBeginEvent, CombatDamageCompleteEvent, CombatEndEvent, CombatState,
    Comparison, CreatureAttacksEvent, CreatureBlockedEvent, CreatureBlocksEvent,
    DeclareAttackersEvent, DeclareAttackersStepBeginEvent, DeclareAttackersStepEndEvent,
    DeclareBlockersEvent, DeclareBlockersStepBeginEvent, DeclareBlockersStepEndEvent,
    assign_combat_damage_system, declare_attackers_system, declare_blockers_system,
    end_combat_system, handle_declare_attackers_event, handle_declare_blockers_event,
    initialize_combat_phase, process_combat_damage_system,
};
pub use test_utils::{
    add_attacker_with_target, apply_combat_damage, assign_blocker, deal_damage_to_players,
    setup_test_combat,
};
