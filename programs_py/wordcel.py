# wordcel
# Built with Seahorse v0.2.2


from seahorse.prelude import *
declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class Profile(Account):
    authority: Pubkey
    bump: u8
    random_hash: str
    

class Post(Account):
    profile: Pubkey
    metadata_uri: str
    bump: u8
    random_hash: str


class Connection(Account):
    profile: Pubkey
    authority: Pubkey
    bump: u8


@instruction
def create_profile(
    user: Signer,
    random_hash: str,
    profile: Empty[Profile]
):

    profile_ = profile.init(
        payer = user,
        seeds = ["profile", random_hash]
   )
    profile_.authority = user.key()
    profile_.random_hash = random_hash
    profile_.bump = profile.bump()

@instruction
def create_post(
    user: Signer,
    random_hash: str,
    post: Empty[Post],
    metadata_uri: str, 
    profile: Profile, 
):
 assert ((user.key() == profile.authority) 
 and (profile.random_hash != random_hash) 
 and (profile.random_hash != metadata_uri)), "Invalid parameters"
 post_account = post.init(
    payer = user, 
    seeds = ['post', random_hash]
 )
 post_account.profile = profile.key()
 post_account.metadata_uri = metadata_uri
 post_account.random_hash = random_hash
 post_account.bump = post.bump()

@instruction
def update_post(
    user: Signer,
    metadata_uri: str, 
    profile: Profile, 
    post: Post,
):
# since we have used strings in random_hash i chose to do exhaustive sanity checks
# this will change when seahorse supports array deserialization
 assert ((post.profile == profile.key())
    and (profile.authority == user.key())
    and (profile.random_hash != post.random_hash) 
    and (profile.random_hash != metadata_uri)
    and (post.random_hash != metadata_uri)
    and (post.metadata_uri != metadata_uri)), "Invalid parameters"

 assert len(metadata_uri) > 128, "Uri length exceeded"

 post.metadata_uri = metadata_uri
 
@instruction
def comment(
    user: Signer, 
    metadata_uri: str,
    random_hash: str,
    post: Empty[Post],
    profile: Profile,  #since comments are just micro posts or replies it would need a post account
):
  assert (((profile.authority == user.key()) 
  and (random_hash != metadata_uri)
  and (profile.random_hash != random_hash))), "Invalid parameters"

  assert len(metadata_uri) > 128, "Uri length exceeded"

  comment_acc = post.init(
    payer = user,
    seeds =['comment', random_hash]
  )
  comment_acc.profile = profile.key()
  comment_acc.bump = post.bump()
  comment_acc.random_hash = random_hash
  comment_acc.metadata_uri = metadata_uri

@instruction
def follow(
    user: Signer, 
    user_profile: Profile,
    profile_to_be_followed: Profile,
    follow: Empty[Connection],
):
 assert ((user_profile.authority == user.key()) 
 and (user_profile.key() != profile_to_be_followed.key()
 and (profile_to_be_followed.key() != user.key()))), "STOP CHEATING AND CHECK YOUR PROFILES !"

#seems like passing raw pubkeys as seeds is  
# not yet supported in seahorse v0.2.2
 follow_account = follow.init(
    payer = user,
    seeds = ['connection', user, 'follows'],
 )
 follow_account.authority = user_profile.key()
 follow_account.profile = profile_to_be_followed.key()
 follow_account.bump = follow.bump()