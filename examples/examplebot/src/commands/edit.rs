use fluxer_rs::{
    api::common::{edit_message_with_embeds, fetch_message, send_message},
    command,
};

#[command(EditCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let args = &feedback.args;

    if args.len() != 2 {
        send_message(api, &data.channel_id, "Invalid syntax").await?;
        return Ok(());
    }

    let resp = fetch_message(api, args.first().unwrap(), args.get(1).unwrap()).await?;
    edit_message_with_embeds(
        api,
        &resp.channel_id,
        &resp.id,
        "EEEEEEEEEEEEE",
        resp.embeds.unwrap(),
    )
    .await?;

    Ok(())
}
