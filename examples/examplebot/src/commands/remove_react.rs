use fluxer_rs::{
    api::common::{remove_all_emoji_reactions, send_message},
    command, util::get_emoji,
};

#[command(RemoveReactCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let args = &feedback.args;

    if args.len() != 3 {
        send_message(api, &data.channel_id, "Invalid syntax").await?;
        return Ok(());
    }

    remove_all_emoji_reactions(
        api,
        args.first().unwrap(),
        args.get(1).unwrap(),
        &get_emoji(args.get(2).unwrap()),
    )
    .await?;

    Ok(())
}
