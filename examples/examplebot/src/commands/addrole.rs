use fluxer_rs::api::common::{give_role, send_message};
use fluxer_rs::command;

#[command(AddRoleCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let args = &feedback.args;

    if args.len() != 3 {
        send_message(api, &data.channel_id, "Invalid syntax").await?;
        return Ok(());
    }

    give_role(
        api,
        args.first().unwrap(),
        args.get(1).unwrap(),
        args.get(2).unwrap(),
    )
    .await?;

    Ok(())
}
