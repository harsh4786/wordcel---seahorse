#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

pub mod dot;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token::{self, Mint, Token, TokenAccount},
};

use dot::program::*;
use std::{cell::RefCell, rc::Rc};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod seahorse_util {
    use super::*;
    use std::{collections::HashMap, fmt::Debug, ops::Deref};

    pub struct Mutable<T>(Rc<RefCell<T>>);

    impl<T> Mutable<T> {
        pub fn new(obj: T) -> Self {
            Self(Rc::new(RefCell::new(obj)))
        }
    }

    impl<T> Clone for Mutable<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T> Deref for Mutable<T> {
        type Target = Rc<RefCell<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T: Debug> Debug for Mutable<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: Default> Default for Mutable<T> {
        fn default() -> Self {
            Self::new(T::default())
        }
    }

    impl<T: Clone> Mutable<Vec<T>> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index > 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    impl<T: Clone, const N: usize> Mutable<[T; N]> {
        pub fn wrapped_index(&self, mut index: i128) -> usize {
            if index > 0 {
                return index.try_into().unwrap();
            }

            index += self.borrow().len() as i128;

            return index.try_into().unwrap();
        }
    }

    #[derive(Clone)]
    pub struct Empty<T: Clone> {
        pub account: T,
        pub bump: Option<u8>,
    }

    #[derive(Clone, Debug)]
    pub struct ProgramsMap<'info>(pub HashMap<&'static str, AccountInfo<'info>>);

    impl<'info> ProgramsMap<'info> {
        pub fn get(&self, name: &'static str) -> AccountInfo<'info> {
            self.0.get(name).unwrap().clone()
        }
    }

    #[derive(Clone, Debug)]
    pub struct WithPrograms<'info, 'entrypoint, A> {
        pub account: &'entrypoint A,
        pub programs: &'entrypoint ProgramsMap<'info>,
    }

    impl<'info, 'entrypoint, A> Deref for WithPrograms<'info, 'entrypoint, A> {
        type Target = A;

        fn deref(&self) -> &Self::Target {
            &self.account
        }
    }

    pub type SeahorseAccount<'info, 'entrypoint, A> =
        WithPrograms<'info, 'entrypoint, Box<Account<'info, A>>>;

    pub type SeahorseSigner<'info, 'entrypoint> = WithPrograms<'info, 'entrypoint, Signer<'info>>;

    #[derive(Clone, Debug)]
    pub struct CpiAccount<'info> {
        #[doc = "CHECK: CpiAccounts temporarily store AccountInfos."]
        pub account_info: AccountInfo<'info>,
        pub is_writable: bool,
        pub is_signer: bool,
        pub seeds: Option<Vec<Vec<u8>>>,
    }

    #[macro_export]
    macro_rules! assign {
        ($ lval : expr , $ rval : expr) => {{
            let temp = $rval;

            $lval = temp;
        }};
    }

    #[macro_export]
    macro_rules! index_assign {
        ($ lval : expr , $ idx : expr , $ rval : expr) => {
            let temp_rval = $rval;
            let temp_idx = $idx;

            $lval[temp_idx] = temp_rval;
        };
    }
}

#[program]
mod wordcel {
    use super::*;
    use seahorse_util::*;
    use std::collections::HashMap;

    #[derive(Accounts)]
    # [instruction (random_hash : String)]
    pub struct CreateProfile<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Profile > () + 8 , payer = user , seeds = ["profile" . as_bytes () . as_ref () , random_hash . as_bytes () . as_ref ()] , bump)]
        pub profile: Box<Account<'info, dot::program::Profile>>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

    pub fn create_profile(ctx: Context<CreateProfile>, random_hash: String) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let user = SeahorseSigner {
            account: &ctx.accounts.user,
            programs: &programs_map,
        };

        let profile = Empty {
            account: dot::program::Profile::load(&mut ctx.accounts.profile, &programs_map),
            bump: ctx.bumps.get("profile").map(|bump| *bump),
        };

        create_profile_handler(user.clone(), random_hash, profile.clone());

        dot::program::Profile::store(profile.account);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (random_hash : String , metadata_uri : String)]
    pub struct CreatePost<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Post > () + 8 , payer = user , seeds = ["post" . as_bytes () . as_ref () , random_hash . as_bytes () . as_ref ()] , bump)]
        pub post: Box<Account<'info, dot::program::Post>>,
        #[account(mut)]
        pub profile: Box<Account<'info, dot::program::Profile>>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: Program<'info, System>,
    }

    pub fn create_post(
        ctx: Context<CreatePost>,
        random_hash: String,
        metadata_uri: String,
    ) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let user = SeahorseSigner {
            account: &ctx.accounts.user,
            programs: &programs_map,
        };

        let post = Empty {
            account: dot::program::Post::load(&mut ctx.accounts.post, &programs_map),
            bump: ctx.bumps.get("post").map(|bump| *bump),
        };

        let profile = dot::program::Profile::load(&mut ctx.accounts.profile, &programs_map);

        create_post_handler(
            user.clone(),
            random_hash,
            post.clone(),
            metadata_uri,
            profile.clone(),
        );

        dot::program::Post::store(post.account);

        dot::program::Profile::store(profile);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (metadata_uri : String , random_hash : String)]
    pub struct Comment<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        # [account (init , space = std :: mem :: size_of :: < dot :: program :: Post > () + 8 , payer = user , seeds = ["comment" . as_bytes () . as_ref () , random_hash . as_bytes () . as_ref ()] , bump)]
        pub post: Box<Account<'info, dot::program::Post>>,
        #[account(mut)]
        pub profile: Box<Account<'info, dot::program::Profile>>,
        pub system_program: Program<'info, System>,
        pub rent: Sysvar<'info, Rent>,
    }

    pub fn comment(ctx: Context<Comment>, metadata_uri: String, random_hash: String) -> Result<()> {
        let mut programs = HashMap::new();

        programs.insert(
            "system_program",
            ctx.accounts.system_program.to_account_info(),
        );

        let programs_map = ProgramsMap(programs);
        let user = SeahorseSigner {
            account: &ctx.accounts.user,
            programs: &programs_map,
        };

        let post = Empty {
            account: dot::program::Post::load(&mut ctx.accounts.post, &programs_map),
            bump: ctx.bumps.get("post").map(|bump| *bump),
        };

        let profile = dot::program::Profile::load(&mut ctx.accounts.profile, &programs_map);

        comment_handler(
            user.clone(),
            metadata_uri,
            random_hash,
            post.clone(),
            profile.clone(),
        );

        dot::program::Post::store(post.account);

        dot::program::Profile::store(profile);

        return Ok(());
    }

    #[derive(Accounts)]
    # [instruction (metadata_uri : String)]
    pub struct UpdatePost<'info> {
        #[account(mut)]
        pub user: Signer<'info>,
        #[account(mut)]
        pub profile: Box<Account<'info, dot::program::Profile>>,
        #[account(mut)]
        pub post: Box<Account<'info, dot::program::Post>>,
    }

    pub fn update_post(ctx: Context<UpdatePost>, metadata_uri: String) -> Result<()> {
        let mut programs = HashMap::new();
        let programs_map = ProgramsMap(programs);
        let user = SeahorseSigner {
            account: &ctx.accounts.user,
            programs: &programs_map,
        };

        let profile = dot::program::Profile::load(&mut ctx.accounts.profile, &programs_map);
        let post = dot::program::Post::load(&mut ctx.accounts.post, &programs_map);

        update_post_handler(user.clone(), metadata_uri, profile.clone(), post.clone());

        dot::program::Profile::store(profile);

        dot::program::Post::store(post);

        return Ok(());
    }
}
