use fluxer_rs::{
    api::common::{delete_role, send_message},
    command,
};

#[command(DeleteRoleCommand)]
async fn execute(api: &FluxerApiHandler, feedback: &CommandFeedback) {
    let data = feedback.data;
    let args = &feedback.args;

    if args.len() != 2 || args.is_empty() {
        send_message(api, &data.channel_id, "Invalid syntax").await?;
        return Ok(());
    }

    delete_role(api, args.first().unwrap(), args.get(1).unwrap()).await?;

    Ok(())
}
