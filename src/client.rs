use serenity::client::Context;
use serenity::model::guild::{Member, Role};
use serenity::model::permissions::Permissions;

pub async fn is_role_higher(ctx: &Context, member_orig: &Member, member_targ: &Member) -> bool {
    return get_highest_role(ctx, member_orig).await.position
        > get_highest_role(ctx, member_targ).await.position;
}

pub async fn get_highest_role(ctx: &Context, member: &Member) -> Role {
    let mut roles = member.roles(&ctx.cache).await.unwrap();
    roles.sort_by(|a, b| b.position.cmp(&a.position));
    roles[0].clone()
}

pub async fn check_perm(ctx: &Context, member: &Member, perm: Permissions) -> bool {
    return member.roles(&ctx.cache).await.unwrap().iter().any(|r| {
        r.permissions.contains(perm) || r.permissions.contains(Permissions::ADMINISTRATOR)
    });
}