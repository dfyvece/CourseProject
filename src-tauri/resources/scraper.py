#!/usr/bin/env python

import sys
import json
import time
from youtube_transcript_api import YouTubeTranscriptApi

def main():
    with open("./tv_shows.json", "r") as f:
        tv_shows = json.loads(f.read())

    native = "en"

    transcripts = {}
    print("Starting download...")
    tv_shows_list = tv_shows.get("data", [])
    for i,tv_show in enumerate(tv_shows_list):
        sys.stdout.write(f"Video {i+1}/{len(tv_shows_list)}")
        sys.stdout.flush()
        youtube_id = tv_show.get("youtube_id")
        try:
            transcipt = YouTubeTranscriptApi.get_transcript(
                youtube_id,
                languages=[native],
            )
            transcripts[youtube_id] = transcipt
        except:
            pass
        time.sleep(1)
        sys.stdout.write("\b"*32)
    print()

    with open("./transcripts.json", "w") as f:
        f.write(json.dumps(transcripts))

if __name__ == "__main__":
    main()
