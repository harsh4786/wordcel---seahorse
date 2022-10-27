#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use crate::{assign, index_assign, seahorse_util::*};
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct Connection {
    pub profile: Pubkey,
    pub authority: Pubkey,
    pub bump: u8,
}

impl<'info, 'entrypoint> Connection {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedConnection<'info, 'entrypoint>> {
        let profile = account.profile.clone();
        let authority = account.authority.clone();
        let bump = account.bump;

        Mutable::new(LoadedConnection {
            __account__: account,
            __programs__: programs_map,
            profile,
            authority,
            bump,
        })
    }

    pub fn store(loaded: Mutable<LoadedConnection>) {
        let mut loaded = loaded.borrow_mut();
        let profile = loaded.profile.clone();

        loaded.__account__.profile = profile;

        let authority = loaded.authority.clone();

        loaded.__account__.authority = authority;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;
    }
}

#[derive(Debug)]
pub struct LoadedConnection<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Connection>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub profile: Pubkey,
    pub authority: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(Debug)]
pub struct Profile {
    pub authority: Pubkey,
    pub bump: u8,
    pub random_hash: String,
}

impl<'info, 'entrypoint> Profile {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedProfile<'info, 'entrypoint>> {
        let authority = account.authority.clone();
        let bump = account.bump;
        let random_hash = account.random_hash.clone();

        Mutable::new(LoadedProfile {
            __account__: account,
            __programs__: programs_map,
            authority,
            bump,
            random_hash,
        })
    }

    pub fn store(loaded: Mutable<LoadedProfile>) {
        let mut loaded = loaded.borrow_mut();
        let authority = loaded.authority.clone();

        loaded.__account__.authority = authority;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;

        let random_hash = loaded.random_hash.clone();

        loaded.__account__.random_hash = random_hash;
    }
}

#[derive(Debug)]
pub struct LoadedProfile<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Profile>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub authority: Pubkey,
    pub bump: u8,
    pub random_hash: String,
}

#[account]
#[derive(Debug)]
pub struct Post {
    pub profile: Pubkey,
    pub metadata_uri: String,
    pub bump: u8,
    pub random_hash: String,
}

impl<'info, 'entrypoint> Post {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedPost<'info, 'entrypoint>> {
        let profile = account.profile.clone();
        let metadata_uri = account.metadata_uri.clone();
        let bump = account.bump;
        let random_hash = account.random_hash.clone();

        Mutable::new(LoadedPost {
            __account__: account,
            __programs__: programs_map,
            profile,
            metadata_uri,
            bump,
            random_hash,
        })
    }

    pub fn store(loaded: Mutable<LoadedPost>) {
        let mut loaded = loaded.borrow_mut();
        let profile = loaded.profile.clone();

        loaded.__account__.profile = profile;

        let metadata_uri = loaded.metadata_uri.clone();

        loaded.__account__.metadata_uri = metadata_uri;

        let bump = loaded.bump;

        loaded.__account__.bump = bump;

        let random_hash = loaded.random_hash.clone();

        loaded.__account__.random_hash = random_hash;
    }
}

#[derive(Debug)]
pub struct LoadedPost<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, Post>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub profile: Pubkey,
    pub metadata_uri: String,
    pub bump: u8,
    pub random_hash: String,
}

pub fn comment_handler<'info>(
    mut user: SeahorseSigner<'info, '_>,
    mut metadata_uri: String,
    mut random_hash: String,
    mut post: Empty<Mutable<LoadedPost<'info, '_>>>,
    mut profile: Mutable<LoadedProfile<'info, '_>>,
) -> () {
    if !(((profile.borrow().authority == user.key()) && (random_hash != metadata_uri))
        && (profile.borrow().random_hash != random_hash))
    {
        panic!("Invalid parameters");
    }

    if !((metadata_uri.len() as u64) > 128) {
        panic!("Uri length exceeded");
    }

    let mut comment_acc = post.account.clone();

    assign!(
        comment_acc.borrow_mut().profile,
        profile.borrow().__account__.key()
    );

    assign!(comment_acc.borrow_mut().bump, post.bump.unwrap());

    assign!(comment_acc.borrow_mut().random_hash, random_hash);

    assign!(comment_acc.borrow_mut().metadata_uri, metadata_uri);
}

pub fn create_post_handler<'info>(
    mut user: SeahorseSigner<'info, '_>,
    mut random_hash: String,
    mut post: Empty<Mutable<LoadedPost<'info, '_>>>,
    mut metadata_uri: String,
    mut profile: Mutable<LoadedProfile<'info, '_>>,
) -> () {
    if !(((user.key() == profile.borrow().authority)
        && (profile.borrow().random_hash != random_hash))
        && (profile.borrow().random_hash != metadata_uri))
    {
        panic!("Invalid parameters");
    }

    let mut post_account = post.account.clone();

    assign!(
        post_account.borrow_mut().profile,
        profile.borrow().__account__.key()
    );

    assign!(post_account.borrow_mut().metadata_uri, metadata_uri);

    assign!(post_account.borrow_mut().random_hash, random_hash);

    assign!(post_account.borrow_mut().bump, post.bump.unwrap());
}

pub fn follow_handler<'info>(
    mut user: SeahorseSigner<'info, '_>,
    mut user_profile: Mutable<LoadedProfile<'info, '_>>,
    mut profile_to_be_followed: Mutable<LoadedProfile<'info, '_>>,
    mut follow: Empty<Mutable<LoadedConnection<'info, '_>>>,
) -> () {
    if !((user_profile.borrow().authority == user.key())
        && ((user_profile.borrow().__account__.key()
            != profile_to_be_followed.borrow().__account__.key())
            && (profile_to_be_followed.borrow().__account__.key() != user.key())))
    {
        panic!("STOP CHEATING AND CHECK YOUR PROFILES !");
    }

    let mut follow_account = follow.account.clone();

    assign!(
        follow_account.borrow_mut().authority,
        user_profile.borrow().__account__.key()
    );

    assign!(
        follow_account.borrow_mut().profile,
        profile_to_be_followed.borrow().__account__.key()
    );

    assign!(follow_account.borrow_mut().bump, follow.bump.unwrap());
}

pub fn create_profile_handler<'info>(
    mut user: SeahorseSigner<'info, '_>,
    mut random_hash: String,
    mut profile: Empty<Mutable<LoadedProfile<'info, '_>>>,
) -> () {
    let mut profile_ = profile.account.clone();

    assign!(profile_.borrow_mut().authority, user.key());

    assign!(profile_.borrow_mut().random_hash, random_hash);

    assign!(profile_.borrow_mut().bump, profile.bump.unwrap());
}

pub fn update_post_handler<'info>(
    mut user: SeahorseSigner<'info, '_>,
    mut metadata_uri: String,
    mut profile: Mutable<LoadedProfile<'info, '_>>,
    mut post: Mutable<LoadedPost<'info, '_>>,
) -> () {
    if !((((((post.borrow().profile == profile.borrow().__account__.key())
        && (profile.borrow().authority == user.key()))
        && (profile.borrow().random_hash != post.borrow().random_hash))
        && (profile.borrow().random_hash != metadata_uri))
        && (post.borrow().random_hash != metadata_uri))
        && (post.borrow().metadata_uri != metadata_uri))
    {
        panic!("Invalid parameters");
    }

    if !((metadata_uri.len() as u64) > 128) {
        panic!("Uri length exceeded");
    }

    assign!(post.borrow_mut().metadata_uri, metadata_uri);
}
