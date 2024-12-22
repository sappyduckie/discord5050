# discord5050
A Discord bot that finds and validates randomly generated Discord media links. Submissions accepted.

## Due to the sensitivity of this project, I will NOT be publishing the search algorithm, rather the bot itself that returns an image stored from the algorithm.
- Image submissions will be accepted within the bot under /submit (attached image)
- User submitted images can be viewed via /user
- Generated images can be viewed via /gen
- Images will NOT be generated per user request due to the intensity of the search tree algorithm
- Images will be uploaded to the .sqlite file every time one is found
- I cannot provide an ETA at this time on generated links
- Generation times will be faster whenever I buy a new CPU
## Development Notes
The algorithm is complete and actively running on my computer, it's a 6 core processor so it's not fast right now but I'm saving up for a 16 core. It is written in Rust using Rusqlite to send returned images to a file. I will NOT be publicizing the program itself, ONLY the discord bot. All generation is happening locally on my own system and I will not be sharing the executable for others to generate with.
The Discord bot is incomplete at the moment because this is my first time writing one and I'm still figuring things out.
