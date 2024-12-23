from bs4 import BeautifulSoup
import random
import requests
import os

def pattern_generator():
    # randomized generic strings
    format_width = random.randint(1, 999)
    format_height = random.randint(1, 999)
    one_two = random.choices([1, 2])
    one_two_string = str(one_two).replace("[", "").replace("]", "").replace("(", "").replace(")", "")

    # randomized specific values
    guild_id = random.randint(1, 999999999999999999)
    channel_id = random.randint(1, 9999999999999999)
    # file_name = ''.join(random.choices(ascii_string, k=16))
    # file_ext = random.choices(["jpg", "png", "gif", "jpeg"])
    hex_8 = os.urandom(8).hex()
    hex_64 = os.urandom(64).hex()

    # final url
    return f"https://media.discordapp.net/attachments/1{guild_id}/13{one_two_string}{channel_id}/img.png?ex={hex_8}&is={hex_8}&hm={hex_8}&=&format=webp&quality=lossless{hex_64}&width={format_width}&height={format_height}"

pattern = pattern_generator()

def url_valid(pattern):
    try:
        response = requests.get(pattern)
        response.raise_for_status()

        soup = BeautifulSoup(response.content, 'html.parser')

        page_text = soup.get_text()

        if "Page" in page_text:
            return False
        else:
            return True

    except requests.exceptions.RequestException as e:
        return f"{e} URL: {pattern}"

def main():
    iteration = 0
    while True:
        pattern = pattern_generator()
        iteration += 1
        print(iteration, pattern)
        if url_valid(pattern) == True:
            print(pattern)
            break

if __name__ == "__main__":
    main()
