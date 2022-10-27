# wordcel
# Built with Seahorse v0.2.2


from seahorse.prelude import *
declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

MAX_LEN_URI: u64 = 128 
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
 assert ((user.key() == profile.authority) and (profile.random_hash != random_hash) and (profile.random_hash != metadata_uri)), "Invalid parameters"
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
# since we have used strings in random_hash we have to do exhaustive checks
# this will change when seahorse supports array deserialization
 assert ((post.profile == profile.key()) 
    and (profile.random_hash != post.random_hash) 
    and (profile.random_hash != metadata_uri)
    and (post.random_hash != metadata_uri)
    and (post.metadata_uri != metadata_uri)), "Invalid parameters"

 