use anchor_lang::prelude::*;

declare_id!("DWwbdy9MC3Mg6Bagc7S1urbThv6qkBr6yNHGVWfqDzij");

#[program]
pub mod emotional_registry {
    use super::*;

    // CREATE: Inicializa la PDA y guarda la emoción
    pub fn create_emotion(ctx: Context<CreateEmotion>, emotion: String, description: String) -> Result<()> {
        let emotion_account = &mut ctx.accounts.emotion_account;
        
        // Validaciones de longitud según el espacio reservado
        require!(emotion.len() <= 50, EmotionError::StringTooLong);
        require!(description.len() <= 200, EmotionError::StringTooLong);

        emotion_account.author = *ctx.accounts.user.key;
        emotion_account.emotion = emotion;
        emotion_account.description = description;
        emotion_account.timestamp = Clock::get()?.unix_timestamp;
        
        msg!("Registro emocional creado con éxito para el usuario: {}", ctx.accounts.user.key());
        Ok(())
    }

    // UPDATE: Modifica los datos existentes en la PDA
    pub fn update_emotion(ctx: Context<UpdateEmotion>, new_emotion: String, new_description: String) -> Result<()> {
        let emotion_account = &mut ctx.accounts.emotion_account;

        require!(new_emotion.len() <= 50, EmotionError::StringTooLong);
        require!(new_description.len() <= 200, EmotionError::StringTooLong);

        emotion_account.emotion = new_emotion;
        emotion_account.description = new_description;
        emotion_account.timestamp = Clock::get()?.unix_timestamp;
        
        msg!("Registro emocional actualizado.");
        Ok(())
    }

    // DELETE: Cierra la cuenta y devuelve los SOL de la renta al usuario
    pub fn delete_emotion(_ctx: Context<DeleteEmotion>) -> Result<()> {
        msg!("Registro eliminado. Renta recuperada por el usuario.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEmotion<'info> {
    #[account(
        init, 
        payer = user, 
        // 8 (discrim.) + 32 (pubkey) + (4 + 50) (string) + (4 + 200) (string) + 8 (i64)
        space = 8 + 32 + 54 + 204 + 8, 
        seeds = [b"emotion", user.key().as_ref()], 
        bump
    )]
    pub emotion_account: Account<'info, EmotionEntry>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateEmotion<'info> {
    #[account(
        mut, 
        seeds = [b"emotion", user.key().as_ref()], 
        bump,
        has_one = author // Seguridad: Solo el autor original puede editar
    )]
    pub emotion_account: Account<'info, EmotionEntry>,
    pub author: SystemAccount<'info>, // No necesita firmar, solo validar la llave
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteEmotion<'info> {
    #[account(
        mut, 
        seeds = [b"emotion", user.key().as_ref()], 
        bump, 
        close = user // Transfiere el balance de la cuenta al firmante
    )]
    pub emotion_account: Account<'info, EmotionEntry>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct EmotionEntry {
    pub author: Pubkey,
    pub emotion: String,
    pub description: String,
    pub timestamp: i64,
}

#[error_code]
pub enum EmotionError {
    #[msg("El texto es demasiado largo. Máximo 50 para emoción y 200 para descripción.")]
    StringTooLong,
}