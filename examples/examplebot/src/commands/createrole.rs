use fluxer_rs::{
    api::common::{create_role, send_message},
    command,
};

#[command(CreateRoleCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let args = &feedback.args;

    if args.len() != 4 {
        send_message(api, &data.channel_id, "Invalid syntax").await?;
        return Ok(());
    }

    create_role(
        api,
        args.first().unwrap(),
        args.get(1).unwrap(),
        args.get(2).unwrap(),
        args.get(3).unwrap(),
    )
    .await?;

    Ok(())
}
