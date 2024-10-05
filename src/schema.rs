use poise::serenity_prelude::*;

structstruck::strike! {
    #[allow(dead_code)]
    pub struct ReactionRole {
        role: Role,
        message: Message,
        reaction: ReactionType,
        variant:
            #[allow(dead_code)]
            enum ReactionRoleVariant {
                Standard,
                PermaAdd,
                PermaRemove,
                Reverse,
            }
    }
}
