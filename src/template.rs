use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process;

pub fn gen_basic(file_name: &str) {
    let code = r#"import discord
from discord.ext import commands

bot = commands.Bot(command_prefix='!', intents=discord.Intents.default())

@bot.event
async def on_ready():
    print(f'Logged on as {bot.user}!')

@bot.slash_command(name='ping', description='ping pong', guild_ids=[your_guild_id])
async def ping(interaction: discord.Interaction):
    await interaction.response.send_message(f'Pong!')

bot.run('your_token_here')"#;


    write_template(file_name, code);
}

pub fn gen_commands(file_name: &str) {
    let code = r#"import discord
from discord.ext import commands

bot = commands.Bot(command_prefix='!', intents=discord.Intents.all())

# classic ping pong command
@bot.slash_command(name='ping', description='ping pong', guild_ids=[your_guild_id])
async def ping(interaction: discord.Interaction):
    """Replies with latency"""
    await interaction.response.send_message(f'Pong!')

# addition command
@bot.slash_command(name='add', description='addition (plus)', guild_ids=[your_guild_id])
async def add(interaction: discord.Interaction, left: int, right: int):
    await interaction.response.send_message(str(left + right))

# kick command
@bot.slash_command(name='kick', description='kick a member', guild_ids=[your_guild_id])
async def kick(interaction: discord.Interaction, user: discord.Member, reason: str):
    """Kicks a member from the guild."""
    await interaction.guild.kick(user, reason=reason)
    await interaction.response.send_message(f'{user.mention} has been kicked by {interaction.user.mention} for {reason}')

bot.run('your_token_here')"#;

    write_template(file_name, code);
}

fn write_template(file_name: &str, code: &str) {
    let path = PathBuf::from(file_name);
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to create file: {}", err);
            process::exit(1);
        }
    };

    match file.write_all(code.as_bytes()) {
        Ok(_) => println!("Template '{}' created successfully.", file_name),
        Err(err) => {
            eprintln!("Failed to write to file: {}", err);
            process::exit(1);
        }
    }
}
