import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EmotionalRegistry } from "../target/types/emotional_registry";
import type { EmotionalRegistry } from "../target/types/emotional_registry";

describe("emotional_registry", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EmotionalRegistry as anchor.Program<EmotionalRegistry>;
  
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.EmotionalRegistry as Program<EmotionalRegistry>;

  it("¡Crea un registro emocional!", async () => {
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("emotion"), anchor.AnchorProvider.env().wallet.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .createEmotion("Feliz", "¡Acabo de terminar mi proyecto de Solana!")
      .accounts({
        emotionAccount: pda,
        user: anchor.AnchorProvider.env().wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    console.log("Registro creado en la PDA:", pda.toBase58());
  });
});