use crate::elements::Tuple;
use crate::equation::EqnExists;
use anchor_lang::prelude::*;

pub fn operate(ctx: Context<EqnExists>) -> Result<()> {
    let tuple_1 = ctx
        .accounts
        .eqn
        .element_1
        .as_ref()
        .expect("tuple 1 not set")
        .clone();
    let tuple_2 = ctx
        .accounts
        .eqn
        .element_2
        .as_ref()
        .expect("tuple 2 not set")
        .clone();

    let result = match ctx.accounts.eqn.operation {
        Operation::Addition => tuple_1 + tuple_2,

        _ => Tuple {
            data: [
                tuple_1.x() - tuple_2.x(),
                tuple_1.y() - tuple_2.y(),
                tuple_1.z() - tuple_2.z(),
                0.0,
            ],
        },
    };

    ctx.accounts.eqn.result = Some(result);
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub enum Operation {
    Addition,
    Subtraction,
}
