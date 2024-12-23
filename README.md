# Discord5050
A python program that recursively searches randomly generated discord media links for random images uploaded to discords servers.

Use multiple windows (a macro to open them would be helpful) on a high-core CPU, suggested 16 or more to yield results in any realistic timeframe.

This exclusively searches for images named img.png because I didn't want the headache of figuring out a relatively small image name generator. You can add SPOILER_ to the name to exclusively search for spoilered images if you want a higher ratio of NSFW.

I am not responsible for any images found or the harm that is caused by them. There can and will be NSFW. Discord is responsible by making all images uploaded as publicly viewable and not accurately describing that fact to its users in an easily accessible location.

I have only exploited their flaw, which they let me exploit by having one.

## V2.0 ONWARDS IS NOW WRITTEN IN RUST.
I will be dropping support for the Python edition, but you can still use it from V1.1, it works fine, but there's no real reason to I mean its a binary executable. If you wish to edit it for yourself I left the main.py file in the Python/ dir.

V2.0 is about 1.2x faster than V1.1, and uses less CPU overall.

This is going to continue receiving updates as I find ways to improve the speed and efficiency of the program.

V2.0 is built using the reqwest crate, I am currently figuring out the 'hyper' crate to replace it with, if anyone has pointers please let me know.
