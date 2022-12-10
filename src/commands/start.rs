use serenity::builder::CreateApplicationCommand;
use serenity::framework::standard::help_commands::Command;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::{ChannelType, Guild, Member};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption,
    CommandDataOptionValue, ApplicationCommandInteraction,
};
use serenity::model::user::User;
use serenity::prelude::Context;
use songbird::join;

pub async fn run(ctx: &Context, interaction: ApplicationCommandInteraction) {
    let options = interaction.data.options;
    let option = options
        .get(0)
        .expect("Expected channel option")
        .resolved
        .as_ref();

    let guildId = interaction.guild_id.expect("given interaction should be executed in guild");
    let guild = guildId.to_guild_cached(ctx).unwrap();

    let vchannelId = match option {
        CommandDataOptionValue::Channel(channel) => Some(channel.id),
        _ => match guild.voice_states.get(&user.id) {
            Some(voiceState) => voiceState.channel_id,
            None => None
        }
    };

    if vchannelId.is_none() {
        interaction.create_interaction_response(ctx, |response|{
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| {
                    data
                        .content("join voice channel or fill which voice channel to join in command")
                        .ephemeral(true)
                })
        });
        return;
    }

    let manager = 
        songbird::get(ctx).await
            .expect("Songbird Voice client placed in at initialisation.").clone();
    
    let (handler_lock, conn_result) = manager.join(guild_id, connect_to).await;
    
    if Err(error) = conn_result {
        interaction.create_interaction_response(ctx, |response|{
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| {
                    data
                        .content("error occurs when connecting voice channel.")
                        .ephemeral(true)
                })
        });
        eprintln!(error.to_string());
        return;
    }

    
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("join").description("Start voice recognition.")
        .description_localized("ja", "音声認識を開始します")
        .create_option(|option| {
            option
                .name("voice_channel")
                .description("voice channel to join.")
                .kind(CommandOptionType::Channel)
                .channel_types(ChannelType::Voice)
                .required(false)
        })
}