#!/usr/bin/env python

if __name__ == "__main__":
    import json

    with open('./transcripts.json', 'r') as f:
        transcripts = json.loads(f.read())

    print(f'total number of transcripts: {len(transcripts.keys())}')
