import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Math as Mathh } from "../target/types/math";
import { Canvas } from "../target/types/canvas";
import { Artist } from "../target/types/artist";

describe("tracer", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Math as Program<Mathh>;
  const canvasProgram = anchor.workspace.Canvas as Program<Canvas>;
  const artistProgram = anchor.workspace.Artist as Program<Artist>;

  it("canvas init, draw, output", async () => {
    const user = (program.provider as anchor.AnchorProvider).wallet;
    const wallet_canvas = anchor.web3.Keypair.generate();
    const address_canvas = wallet_canvas.publicKey;

    const wallet_1 = anchor.web3.Keypair.generate();
    const address_1 = wallet_1.publicKey;
    interface Env {
      gravity: number[];
      wind: number[];
    }
    interface Proj {
      position: number[];
      velocity: number[];
    }

    const w = 15;
    const h = 15;
    let env = {
      gravity: [0, -0.09, 0, 0],
      wind: [-0.03, 0, 0, 0],
    };

    let proj = {
      position: [0, 0, 0, 1],
      velocity: [0.707, 0.777, 0, 0],
    };

    const tx_canvas_init = await canvasProgram.methods
      .initialize(w, h)
      .accounts({
        canvas: address_canvas,
        operator: user.publicKey,
      })
      .signers([wallet_canvas])
      .rpc();
    console.log("canvas init", tx_canvas_init);

    const tx_init = await program.methods
      .new()
      .accounts({
        eqn: address_1,
        operator: user.publicKey,
      })
      .signers([wallet_1])
      .rpc();
    console.log("eqn init", tx_init);

    const [varsPda, varsBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("variables"), user.publicKey.toBuffer()],
      artistProgram.programId
    );

    const tx_vars = await artistProgram.methods
      .initialize(env.gravity, env.wind, proj.position, proj.velocity)
      .accounts({
        operator: user.publicKey,
        variables: varsPda,
      })
      .signers([])
      .rpc();
    console.log("variables pda init", tx_vars);

    async function tick(env: Env, proj: Proj) {
      let seed = String(h - 1 - Math.round(proj.position[1])).concat(
        "-",
        String(Math.round(proj.position[0]))
      );
      const [canvasPda, expectedBump] =
        await anchor.web3.PublicKey.findProgramAddress(
          [address_canvas.toBuffer(), anchor.utils.bytes.utf8.encode(seed)],
          canvasProgram.programId
        );
      const checkPda = await canvasProgram.provider.connection.getAccountInfo(
        canvasPda
      );
      if (checkPda == null) {
        await canvasProgram.methods
          .fill(seed, [0.9, 0.3, 0.1])
          .accounts({
            operator: user.publicKey,
            canvas: address_canvas,
            pda: canvasPda,
          })
          .signers([])
          .rpc();
      }

      const tx_tick = await artistProgram.methods
        .tick(seed)
        .accounts({
          eqn: address_1,
          var: varsPda,
          mathProgram: program.programId,
        })
        .signers([])
        .rpc();
      console.log("tick txn", tx_tick);
      let vars = await artistProgram.account.variables.fetch(varsPda);
      return { position: vars.proj.position, velocity: vars.proj.velocity };
    }

    while (proj.position[0] <= w && proj.position[1] >= 0) {
      proj = await tick(env, proj);
    }

    for (var i = 0; i < h; i++) {
      for (var j = 0; j < w; j++) {
        let seedString = String(i).concat("-", String(j));
        let account,
          accountBump = null;

        [account, accountBump] = await anchor.web3.PublicKey.findProgramAddress(
          [address_canvas.toBuffer(), Buffer.from(seedString)],
          canvasProgram.programId
        );

        const checkPda = await canvasProgram.provider.connection.getAccountInfo(
          account
        );
        let pdaColor = [0.0, 0.0, 0.0];
        if (checkPda !== null) {
          let pdaState = await canvasProgram.account.pda.fetch(account);
          pdaColor = [pdaState.color.r, pdaState.color.g, pdaState.color.b];
        }

        const tx_canvas_draw = await canvasProgram.methods
          .draw(pdaColor)
          .accounts({
            canvas: address_canvas,
          })
          .signers([])
          .rpc();
      }
    }
    let canvasState = await canvasProgram.account.canvas.fetch(address_canvas);
    let canvasString = canvasState.pixels;
    console.log("String\n", canvasString);
  });
});
